#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Function-level evidence ledger; label coverage cannot produce `verified`.

Imports are grandfathered explicitly at the first inventory. New identities
must carry behavioural, mutant and per-environment evidence to pass --gate.
No absent field or unrecorded comparison counts as a passing comparison.
"""
from __future__ import annotations
import argparse
from collections import Counter, defaultdict
import hashlib
import json
from pathlib import Path
import re
from spec_loader import load_api_tests

ROOT=Path(__file__).resolve().parents[2]
LEVELS={'untested':0,'exercised':1,'behavioural':2,'verified':3,'exempt':3}
CONTEXTS={'synced_gadget':'rules-synced','unsynced_gadget':'rules-unsynced','gaia_synced':'gaia-synced','gaia_unsynced':'gaia-unsynced','widget':'ui','ui':'ui'}

def snake(s):return re.sub(r'([a-z0-9])([A-Z])',r'\1_\2',re.sub(r'([A-Z]+)([A-Z][a-z])',r'\1_\2',s)).lower()
def default(v):
 if isinstance(v,dict):return all(default(value) for value in v.values())
 if isinstance(v,list):return all(default(value) for value in v)
 return v is None or v is False or v==0 or v==''
def load_rows(path):return [json.loads(line) for line in path.read_text().splitlines() if line.strip()] if path.exists() else []
def specs(root):
 directory=root/'test/native_api_parity'
 result=load_api_tests(directory/'api_tests.json')
 for test in json.loads((directory/'surface_tests.json').read_text()).get('tests',[]):
  result.append(dict(test,native={'get':test.get('native',[])}))
 return result

def inventory(root,model):
 tests=specs(root);functions={}
 for module in model['modules']:
  for f in module['functions']:
   identity=module['name']+'.'+f['name'];wrapper=snake(f['name'])
   matched=[t for t in tests if any(name.rsplit('.',1)[-1]==wrapper and snake(name.split('.',1)[0])==module['name'] for names in t.get('native',{}).values() for name in names)]
   functions[identity]={'level':'untested','grandfathered':True,'required_environments':f['environments'],
    'tests':sorted(t['id'] for t in matched),'fields':{},'compared_environments':[],
    'killed_mutants':[],'required_mutants':['zero']+(['noop'] if f['mutating'] else []),'required_returns_mapped':False,'runs':[]}
 return {'schema':1,'functions':functions,'counts':dict(Counter(f['level'] for f in functions.values()))}

def observe(ledger,tests,run):
 tests={t['id']:t for t in tests};rows=defaultdict(list)
 # A completed run is required; result names alone do not prove equality.
 report=run/'report.md'
 if not report.exists() or '- Result: PASS' not in report.read_text():raise ValueError(f'not a passing parity run: {run}')
 for path in (run/'lua').glob('*.jsonl'):
  for row in load_rows(path):
   if row.get('source'):continue # Wasm probe rows are a separate evidence stream.
   name=row.get('testName',row.get('name',''));rows[name].append(row)
 comparison_rows=load_rows(run/'native/comparisons.jsonl')
 checks=defaultdict(list)
 for row in comparison_rows:checks[(row['test'],row['field'])].append(row)
 for identity,f in ledger['functions'].items():
  fields=defaultdict(list);environments=set()
  for test_id in f['tests']:
   spec=tests[test_id];declared=spec.get('compare',{}).get('fields',[])
   for row in rows[test_id]:
    environment=CONTEXTS.get(row.get('context',spec.get('context')))
    if environment:environments.add(environment)
    values=row.get('fields',row)
    for field in declared:fields[field].append(values.get(field,{'__missing__':True}))
  if not fields:continue
  f['level']='exercised';f['compared_environments']=sorted(environments)
  for field,values in fields.items():
   missing=sum(isinstance(v,dict) and v.get('__missing__') is True for v in values)
   compared=[x for tid in f['tests'] for x in checks[(tid,field)] if x.get('compared') is True and x.get('kind')=='value']
   f['fields'][field]={'observations':len(values),'missing':missing,'nondefault':sum(not default(v) for v in values if not (isinstance(v,dict) and v.get('__missing__'))),
    'distinct':len({json.dumps(v,sort_keys=True) for v in values}),'recorded_comparisons':len(compared)}
  f['runs'].append({'path':str(run),'report_sha256':hashlib.sha256(report.read_bytes()).hexdigest()})
  # `behavioural` additionally needs an explicit return mapping and direct
  # comparator evidence. Old *_if_present tests can never silently qualify.
  if f['required_returns_mapped'] and all(v['nondefault'] and v['distinct']>1 and v['missing']==0 and v['recorded_comparisons']>=v['observations'] for v in f['fields'].values()):f['level']='behavioural'
  if f['level']=='behavioural' and set(f['required_mutants']).issubset(f['killed_mutants']) and set(f['required_environments'])<=set(f.get('wasm_compared_environments',[])):f['level']='verified'
 ledger['counts']=dict(Counter(f['level'] for f in ledger['functions'].values()))
 return ledger

def markdown(ledger):
 modules=defaultdict(Counter)
 for identity,entry in ledger['functions'].items():modules[identity.split('.',1)[0]][entry['level']]+=1
 levels=['untested','exercised','behavioural','verified','exempt']
 lines=['| Module | '+' | '.join(levels)+' |','| --- | '+' | '.join(['---:']*len(levels))+' |']
 total=Counter()
 for module,counts in sorted(modules.items()):
  lines.append('| '+module+' | '+' | '.join(str(counts[level]) for level in levels)+' |');total.update(counts)
 lines.append('| **Total** | '+' | '.join(str(total[level]) for level in levels)+' |')
 return lines

def observe_native_audit(ledger, log):
 text=log.read_text()
 if 'NO_STUBS_PASS' not in text or 'NO_STUBS_FAIL' in text:raise ValueError('native audit did not pass')
 names=set(re.findall(r'NO_STUBS_MUTANT_KILLED (\w+)',text))
 for identity,entry in ledger['functions'].items():
  if identity.split('.',1)[1] not in names:continue
  entry['level']='exercised'
  entry['killed_mutants']=['zero']
  if identity=='unit_control.SetUnitHeading':entry['killed_mutants'].append('noop')
  entry['runs'].append({'kind':'targeted-native-audit','path':str(log),'sha256':hashlib.sha256(log.read_bytes()).hexdigest()})
 ledger['counts']=dict(Counter(f['level'] for f in ledger['functions'].values()))
 return ledger

def gate(actual,baseline):
 errors=[];aa=actual['functions'];bb=baseline['functions']
 for identity,f in aa.items():
  if f['level'] not in LEVELS:errors.append(f'{identity}: unknown level');continue
  if identity not in bb and f['level'] not in ('verified','exempt'):errors.append(f'{identity}: new function lacks verified evidence')
  elif identity in bb and LEVELS[f['level']]<LEVELS[bb[identity]['level']]:errors.append(f'{identity}: coverage regressed')
  if f.get('grandfathered') and (identity not in bb or not bb[identity].get('grandfathered')):errors.append(f'{identity}: grandfathering may not increase')
  if f['level']=='exempt' and (not isinstance(f.get('reason'),str) or not f['reason'].strip()):errors.append(f'{identity}: exemption needs a reason')
  if f['level']=='verified' and (not f['required_returns_mapped'] or not set(f['required_mutants']).issubset(f['killed_mutants']) or not f['fields'] or not f['runs'] or any(v['observations']<=0 or v['missing'] or not v['nondefault'] or v['distinct']<2 or v['recorded_comparisons']<v['observations'] for v in f['fields'].values()) or not set(f['required_environments'])<=set(f.get('wasm_compared_environments',[]))):errors.append(f'{identity}: invalid verification claim')
 for identity in bb.keys()-aa.keys():errors.append(f'{identity}: stale ledger identity')
 return errors

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--root',type=Path,default=ROOT);p.add_argument('--run',type=Path,action='append',default=[])
 p.add_argument('--native-audit-log',type=Path,action='append',default=[]);p.add_argument('--baseline',type=Path,default=ROOT/'test/native_api_parity/parity_ledger.json');p.add_argument('--output',type=Path,required=True);p.add_argument('--gate',action='store_true')
 a=p.parse_args();model=json.loads((a.root/'rts/wasm/generated/model.json').read_text());ledger=inventory(a.root,model)
 # A reviewed exemption is the baseline's decision, not evidence a run produces: carry it over.
 for identity,entry in json.loads(a.baseline.read_text())['functions'].items():
  if entry.get('level')=='exempt' and identity in ledger['functions']:ledger['functions'][identity].update(level='exempt',reason=entry.get('reason'),grandfathered=False)
 ledger['counts']=dict(Counter(f['level'] for f in ledger['functions'].values()))
 for run in a.run:observe(ledger,specs(a.root),run.resolve())
 for log in a.native_audit_log:observe_native_audit(ledger,log.resolve())
 a.output.write_text(json.dumps(ledger,indent=2)+'\n')
 errors=gate(ledger,json.loads(a.baseline.read_text())) if a.gate else []
 for e in errors:print(e)
 print(ledger['counts']);return bool(errors)
if __name__=='__main__':raise SystemExit(main())

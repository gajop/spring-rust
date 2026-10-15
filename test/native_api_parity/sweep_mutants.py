#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Run per-function native/Wasm mutants in fresh processes; never count a crash as a kill.

Needs an engine built with NATIVE_API_TEST_HOOKS=ON. Baseline passes are mandatory.
Without a matching test, a function stays untested rather than receiving credit.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys
ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(Path(__file__).resolve().parent))
from parity_ledger import inventory

def run(args, output, tests, mutant=None):
 output.mkdir(parents=True,exist_ok=True)
 command=[sys.executable,str(ROOT/'test/native_api_parity/run_harness.py'),'--mode',args.mode,
  '--cases','3','--tests',','.join(tests),'--timeout',str(args.timeout),'--output-dir',str(output),'--skip-callin-compare']
 if args.mode=='wasm':command+=['--wasm-context',args.context]
 if args.rendering:command+=['--enable-rendering-tests']
 env=os.environ.copy();env.pop('SPRING_NATIVE_API_MUTANT',None)
 if mutant:env['SPRING_NATIVE_API_MUTANT']=mutant
 with (output/'driver.log').open('wb') as log:
  process=subprocess.run(command,env=env,cwd=ROOT,stdout=log,stderr=subprocess.STDOUT)
 runs=sorted(output.glob('*/report.md'))
 return process.returncode,runs[-1].parent if runs else None

def target_failures(run,tests,mode='both'):
 if not run:return []
 if mode=='wasm':
  path=run/'comparison.json'
  if not path.exists():return []
  # A trap, missing row, or missing field is inconclusive. Require the
  # unchanged Lua oracle and the guest to have emitted actual field values.
  result=[]
  for mismatch in json.loads(path.read_text()).get('api_mismatches',[]):
   name=mismatch.get('testName')
   expected=mismatch.get('expected',{});actual=mismatch.get('actual',{})
   if name not in tests or expected.get('status')!='pass' or actual.get('status')!='pass':continue
   ef=expected.get('fields',{});af=actual.get('fields',{})
   if any(key in ef and key in af and ef[key]!=af[key] for key in mismatch.get('differing_fields',[])):result.append(name)
  return sorted(set(result))
 result=[]
 for path in run.rglob('native.jsonl'):
  for line in path.read_text().splitlines():
   row=json.loads(line)
   name=row.get('name','').removeprefix('native_')
   if row.get('status')=='fail' and name in tests:result.append(name)
 return sorted(set(result))

def main():
 p=argparse.ArgumentParser(description=__doc__);p.add_argument('--output',type=Path,required=True)
 p.add_argument('--mode',choices=['both','wasm'],default='both');p.add_argument('--context',default='synced_gadget')
 p.add_argument('--rendering',action='store_true');p.add_argument('--timeout',type=int,default=120)
 p.add_argument('--shard-index',type=int,default=0);p.add_argument('--shard-count',type=int,default=1)
 p.add_argument('--limit',type=int);p.add_argument('--list',action='store_true')
 a=p.parse_args()
 if not 0<=a.shard_index<a.shard_count:p.error('invalid shard')
 model=json.loads((ROOT/'rts/wasm/generated/model.json').read_text());functions=inventory(ROOT,model)['functions']
 selected=sorted(functions)[a.shard_index::a.shard_count]
 if a.limit is not None:selected=selected[:a.limit]
 if a.list:
  for identity in selected:print(identity,','.join(functions[identity]['tests']))
  return 0
 a.output=a.output.resolve();a.output.mkdir(parents=True,exist_ok=True);results={}
 for identity in selected:
  entry=functions[identity];tests=entry['tests'];result={'tests':tests,'mode':a.mode,'context':a.context,'mutants':{}}
  if not tests:result['status']='untested'
  else:
   directory=a.output/identity
   code,baseline=run(a,directory/'baseline',tests)
   if code or not baseline or '- Result: PASS' not in (baseline/'report.md').read_text():result['status']='baseline-failed'
   else:
    for kind in entry['required_mutants']:
     spec=identity+':'+kind
     code,observed=run(a,directory/kind,tests,spec)
     logs=list((directory/kind).rglob('spring.log'))
     activated=any('[NativeApiMutant] '+spec in f.read_text(errors='replace') for f in logs)
     failed=target_failures(observed,tests,a.mode)
     result['mutants'][kind]={'status':'killed' if activated and failed else ('survived' if activated and code==0 else 'inconclusive'),
      'activated':activated,'failed_tests':failed,'exit':code,'run':str(observed) if observed else None}
    result['status']='killed' if all(r['status']=='killed' for r in result['mutants'].values()) else 'incomplete'
  results[identity]=result
  (a.output/'mutants.json').write_text(json.dumps(results,indent=2)+'\n')
  print(identity,result['status'],flush=True)
 return 0 if all(r['status']=='killed' for r in results.values()) else 1
if __name__=='__main__':raise SystemExit(main())

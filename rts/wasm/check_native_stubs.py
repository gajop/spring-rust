#!/usr/bin/env python3
# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
"""Audit native table implementations with libclang; ratchet reviewed findings.

Requires matching clang Python bindings and libclang. Compilation commands must
come from the engine build (Ninja's `-t compdb` is supported), not invented flags.
The gate fails on new findings, stale dispositions, parse errors or missing slots.
"""
from __future__ import annotations
import argparse
from collections import defaultdict
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import sys

ROOT=Path(__file__).resolve().parents[2]
MARKERS=re.compile(r'\b(stub\w*|TODO|FIXME|not implemented|simplif\w*|for now|would need|not (?:easily|directly) (?:available|accessible)|placeholder|assume)\b',re.I)

def children(cursor):
    # Do not recurse through the standard library (newer clangs can add cursor
    # kinds unknown to an older Python binding). All analysis is source-local.
    return list(cursor.get_children())

def descendants(cursor):
    yield cursor
    for child in children(cursor): yield from descendants(child)

def compile_args(entry,root,build):
    args=entry.get('arguments') or shlex.split(entry['command'])
    if Path(args[0]).name=='ccache': args=args[1:]
    result=[];skip=False
    for arg in args[1:]:
        if skip:skip=False;continue
        if arg in ('-c','-o','-MF','-MT','-MQ'):skip=True;continue
        if arg in ('-MD','-MMD','-fsingle-precision-constant','-mieee-fp'):continue
        if '/build/spring-static-libs/' in arg:
            if result and result[-1]=='-isystem':result.pop()
            continue
        result.append(arg.replace('/build/src',str(root)).replace('/build/out',str(build)))
    return result

def analyze_cursor(cursor,CK):
    nodes=list(descendants(cursor))
    query=next((n.spelling for n in nodes if n.kind==CK.PARM_DECL and n.type.spelling.endswith('Query *')), 'query')
    read_fields=set();forwarded_fields=set();query_forwarded=False;result_forwarded=False;effect_call=False;effect_write=False;assignments=defaultdict(list)
    for n in nodes:
        tokens=[t.spelling for t in n.get_tokens()]
        if n.kind==CK.MEMBER_REF_EXPR and len(tokens)>=3 and tokens[0]==query and tokens[1]=='->':read_fields.add(tokens[2])
        if n.kind==CK.CALL_EXPR:
            for arg in n.get_arguments():
                arg_tokens=[t.spelling for t in arg.get_tokens()]
                if query in arg_tokens and '->' not in arg_tokens:query_forwarded=True
                if 'result' in arg_tokens:
                    if '->' not in arg_tokens:result_forwarded=True
                    else:
                        j=arg_tokens.index('->');forwarded_fields.add(''.join(arg_tokens[j+1:]))
            name=n.spelling
            if name and not re.match(r'^(?:Get|Find|Parse|Is|Has|Can|Check|size|empty|begin|end|data|c_str|static_cast|dynamic_cast)',name):effect_call=True
        if n.kind==CK.VAR_DECL and query in tokens and '->' not in tokens:query_forwarded=True
        if n.kind==CK.COMPOUND_ASSIGNMENT_OPERATOR or (n.kind==CK.UNARY_OPERATOR and any(t in ('++','--') for t in tokens)):
            effect_write=True
            if tokens[:2]==['result','->']:
                forwarded_fields.add(tokens[2])
        if n.kind==CK.BINARY_OPERATOR and '=' in tokens:
            i=tokens.index('=');lhs=tokens[:i];rhs=tokens[i+1:]
            if '->' in lhs and lhs[0]!='result':effect_write=True
            if lhs==['*','result']:
                assignments['*'].append(rhs)
            if lhs[:2]==['result','->'] and len(lhs)>=3:
                field=''.join(lhs[2:])
                assignments[field].append(rhs)
    return (read_fields,forwarded_fields,query_forwarded,result_forwarded,effect_call,effect_write,assignments, ' '.join(t.spelling for t in cursor.get_tokens()))

def audit(root,commands,build,only=None):
    from clang import cindex as clang
    if not clang.Config.loaded and os.environ.get('LIBCLANG_FILE'):
        clang.Config.set_library_file(os.environ['LIBCLANG_FILE'])
    elif not clang.Config.loaded and os.environ.get('LIBCLANG_PATH'):
        clang.Config.set_library_path(os.environ['LIBCLANG_PATH'])
    index=clang.Index.create()
    CK=clang.CursorKind
    functions={};slots={};errors=[]
    entries={}
    for e in sorted(commands,key=lambda e: '/legacy/' in e.get('output','')):
        if '/NativeInterface/api/' in e['file'] and e['file'].endswith('.cpp'):entries[Path(e['file']).name]=e
    model=json.loads((root/'rts/wasm/generated/model.json').read_text())
    manifest=(root/'rust/crates/spring-native-codegen/src/manifest.rs').read_text()
    definitions=re.findall(r'module: "([^"]+)",\s*header: "([^"]+)",\s*api_struct: "([^"]*)"',manifest)
    api_modules={api:module for module,header,api in definitions if api}
    for name,entry in sorted(entries.items()):
        if only and name not in only:continue
        source=root/'rts/NativeInterface/api'/name
        tu=index.parse(str(source),args=compile_args(entry,root,build))
        bad=[str(d) for d in tu.diagnostics if d.severity>=clang.Diagnostic.Error]
        if bad:errors.extend(bad);continue
        def visit(cursor):
            if cursor.location.file and Path(cursor.location.file.name)!=source:return
            if cursor.kind==CK.FUNCTION_DECL and cursor.is_definition():
                functions[(name,cursor.spelling)]=analyze_cursor(cursor,CK)
                return
            if cursor.kind==CK.VAR_DECL:
                api=cursor.type.spelling.replace('const ','').strip()
                if api in api_modules:
                    tokens=[t.spelling for t in cursor.get_tokens()]
                    for i,t in enumerate(tokens[:-3]):
                        if t=='.' and tokens[i+2]=='=':
                            slots[(api_modules[api],tokens[i+1])]=(name,tokens[i+3])
            for child in children(cursor):visit(child)
        visit(tu.cursor)
    findings=[]
    for module in model['modules']:
        for f in module['functions']:
            identity=(module['name'],f['name'])
            if identity not in slots:
                if not only: findings.append(dict(module=identity[0],function=identity[1],rule='S1',field='',detail='No initialized native API slot'))
                continue
            source,impl=slots[identity];cursor=functions.get((source,impl))
            if cursor is None:
                findings.append(dict(module=identity[0],function=identity[1],rule='S1',field='',detail=f'Cannot resolve definition of {impl}'));continue
            (read_fields,forwarded_fields,query_forwarded,result_forwarded,effect_call,effect_write,assignments,body)=cursor
            def add(rule,field,detail):
                findings.append(dict(module=identity[0],function=identity[1],rule=rule,field=field,detail=detail,source=f'rts/NativeInterface/api/{source}', source_sha256=hashlib.sha256((root/'rts/NativeInterface/api'/source).read_bytes()).hexdigest()))
            if not query_forwarded:
                for field in f['inputs']:
                    if field['name']!='_unused' and field['name'] not in read_fields:add('S3',field['name'],'Named query field is not read or forwarded')
            literal=lambda rhs: bool(rhs) and all(re.fullmatch(r'(?:true|false|nullptr|NULL|[-+]?\d[\w.eE+-]*|".*"|\'.*\'|[{}(),-])',t) for t in rhs)
            constant=[]
            for field,rhs in assignments.items():
                if field=='error' or field in forwarded_fields:continue
                if not result_forwarded and all(literal(r) for r in rhs) and len({tuple(r) for r in rhs})==1:
                    add('S4',field,'Only one literal is assigned to this output');constant.append(field)
            meaningful=[v for k,v in assignments.items() if k!='error']
            if meaningful and not effect_call and not effect_write and not result_forwarded and all(all(literal(r) for r in v) for v in meaningful):add('S5','','Only literal result assignments found; review forwarded work and validity guards')
            if re.search(r'\{\s*\*\s*result\s*=\s*(?:\w+\s*)?\{\s*\}\s*;\s*\}\s*$',body):add('S5','','Whole result is unconditionally zero-initialized')
            for marker in sorted(set(m.group(0).lower() for m in MARKERS.finditer(body))):add('S6',marker,'Marker in implementation')
            error_values=assignments.get('error',[])
            if error_values and not meaningful and not result_forwarded and all(r and r[0]=='&' for r in error_values):add('S2','','No successful result path or forwarded result detected')
    return findings,errors

def key(row): return tuple(row[k] for k in ('module','function','rule','field'))

def gate(findings,baseline):
    actual={key(r):r for r in findings};known={key(r):r for r in baseline}
    errors=[f'new finding: {k}: {actual[k]["detail"]}' for k in sorted(actual.keys()-known.keys())]
    errors += [f'stale disposition: {k}' for k in sorted(known.keys()-actual.keys())]
    errors += [f'missing reason: {key(r)}' for r in baseline if not r.get('reason','').strip()]
    if len(known)!=len(baseline): errors.append('duplicate dispositions')
    errors += [f'changed reviewed source: {k}' for k in sorted(known.keys() & actual.keys()) if known[k].get('source_sha256') != actual[k].get('source_sha256')]
    return errors

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--root',type=Path,default=ROOT)
    p.add_argument('--commands',type=Path,required=True);p.add_argument('--build',type=Path)
    p.add_argument('--baseline',type=Path,default=ROOT/'rts/wasm/native_stub_dispositions.json')
    p.add_argument('--output',type=Path,default=ROOT/'rts/wasm/docs/generated/native_stub_audit.json')
    p.add_argument('--inventory',action='store_true',help='write findings without accepting them')
    p.add_argument('--only',action='append',help='source basename, diagnostic scan only')
    a=p.parse_args();root=a.root.resolve()
    findings,errors=audit(root,json.loads(a.commands.read_text()),(a.build or root/'build-amd64-linux').resolve(),a.only)
    report={'schema':1,'findings':sorted(findings,key=key),'parse_errors':errors}
    a.output.parent.mkdir(parents=True,exist_ok=True);a.output.write_text(json.dumps(report,indent=2)+'\n')
    if not a.inventory and not a.only:
        baseline=json.loads(a.baseline.read_text());errors+=gate(findings,baseline)
        exemptions=(root/'rts/NativeInterface/api/ParityExemptions.def').read_text()
        declared={}
        for line in exemptions.splitlines():
            if not line.startswith('EXEMPT('):continue
            parts=line[7:-1].split(',',4)
            if len(parts)!=5 or not parts[4].strip():errors.append('invalid semantic exemption');continue
            module,function,rule,field,reason=map(str.strip,parts)
            declared[(module,function,rule,'' if field=='function' else field)]=reason
        for row in baseline:
            if declared.get(key(row))!=row.get('reason'):errors.append(f'exemption reason drift: {key(row)}')
        for identity in declared.keys()-{key(r) for r in baseline}:errors.append(f'stale exemption: {identity}')
    for error in errors:print(error,file=sys.stderr)
    print(f'{len(findings)} findings; {len(errors)} errors; report: {a.output}')
    return bool(errors)
if __name__=='__main__':raise SystemExit(main())

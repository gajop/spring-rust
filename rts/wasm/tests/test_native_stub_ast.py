# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
import importlib.util
import json
import os
from pathlib import Path
import tempfile
import unittest
p=Path(__file__).resolve().parents[1]/'check_native_stubs.py'
s=importlib.util.spec_from_file_location('audit',p);audit=importlib.util.module_from_spec(s);s.loader.exec_module(audit)
class Ast(unittest.TestCase):
 def scan(self,body,slot='Impl'):
  with tempfile.TemporaryDirectory() as directory:
   root=Path(directory);api=root/'rts/NativeInterface/api';api.mkdir(parents=True)
   manifest=root/'rust/crates/spring-native-codegen/src';manifest.mkdir(parents=True)
   (manifest/'manifest.rs').write_text('module: "demo", header: "Demo.h", api_struct: "DemoApi"')
   model=root/'rts/wasm/generated';model.mkdir(parents=True)
   (model/'model.json').write_text(json.dumps({'modules':[{'name':'demo','functions':[{'name':'Run','inputs':[{'name':'x'},{'name':'y'}]}]}]}))
   cpp=api/'Demo.cpp';cpp.write_text('struct Error {}; struct Query { int x,y; }; struct Result { const Error* error; int value; }; struct DemoApi { void (*Run)(const Query*,Result*); }; const Error unavailable{}; void Impl(const Query* query,Result* result) {'+body+'}\nconst DemoApi DEMO_API={.Run='+slot+'};')
   findings,errors=audit.audit(root,[{'file':str(cpp),'arguments':['clang++','-std=c++20','-c',str(cpp)]}],root)
   self.assertFalse(errors);return {x['rule'] for x in findings}
 def test_real_input_dependent_result(self):self.assertFalse(self.scan('result->error=nullptr; result->value=query->x+query->y;'))
 def test_missing_slot(self):self.assertIn('S1',self.scan('','nullptr'))
 def test_unconditional_error(self):self.assertIn('S2',self.scan('result->error=&unavailable;'))
 def test_ignored_field(self):self.assertIn('S3',self.scan('result->value=query->x;'))
 def test_constant_output(self):self.assertIn('S4',self.scan('result->value=0;'))
 def test_zero_result(self):self.assertIn('S5',self.scan('*result={};'))
 def test_marker(self):self.assertIn('S6',self.scan('/* TODO finish */ result->value=query->x+query->y;'))
if __name__=='__main__':unittest.main()

# This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
import importlib.util
from pathlib import Path
import unittest

p=Path(__file__).resolve().parents[1]/'check_native_stubs.py'
s=importlib.util.spec_from_file_location('audit',p);audit=importlib.util.module_from_spec(s);s.loader.exec_module(audit)
class Gate(unittest.TestCase):
 def row(self,**kw):return dict(module='utils',function='TestMoveOrder',rule='S5',field='',detail='constant',source_sha256='abc',reason='readiness guard',**kw)
 def test_new_stub_fails(self):self.assertTrue(audit.gate([self.row()],[]))
 def test_stale_disposition_fails(self):self.assertTrue(audit.gate([],[self.row()]))
 def test_changed_source_requires_review(self):
  a=self.row();b=self.row();b['source_sha256']='def';self.assertTrue(audit.gate([a],[b]))
 def test_reason_required(self):
  a=self.row();a['reason']='';self.assertTrue(audit.gate([a],[a]))
 def test_reviewed_finding_passes(self):self.assertFalse(audit.gate([self.row()],[self.row()]))
if __name__=='__main__':unittest.main()

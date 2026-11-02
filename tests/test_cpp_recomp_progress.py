import importlib.util
import hashlib
import json
import struct
import tempfile
import unittest
from pathlib import Path
spec=importlib.util.spec_from_file_location('recomp',Path(__file__).resolve().parents[1]/'tools/progress/cpp_recomp_progress.py')
p=importlib.util.module_from_spec(spec);spec.loader.exec_module(p)
class RecompilationProgressTests(unittest.TestCase):
    def test_original_bytes_not_routine_count(self):
        s=p.summarise([{'bytes':90,'recompiled':False},{'bytes':10,'recompiled':True}])
        self.assertEqual(s['percent'],10);self.assertEqual(s['recompiled_routines'],1)
    def test_never_round_remaining_bytes_to_100(self):
        s=p.summarise([{'bytes':12307572,'recompiled':True},{'bytes':4,'recompiled':False}])
        self.assertEqual(s['display_percent'],'99.99');self.assertGreater(s['percent'],99.999)
    def test_100_requires_no_remaining_bytes(self):
        s=p.summarise([{'bytes':100,'recompiled':True}]);self.assertEqual(s['display_percent'],'100.00')
    def test_prototypes_and_helpers_do_not_count(self):
        code='void F_00000010(CPUState *c);\nvoid helper(CPUState *c) {}\nvoid F_00000020(CPUState *c) { return; }'
        self.assertEqual(set(p.definitions(code)),{'F_00000020'})
    def test_unsupported_original_body_is_detected(self):
        code='void F_00000010_orig(CPUState *c) { rt_unhandled(c,0,"op"); }\nvoid F_00000010(CPUState *c) {}'
        defs=p.definitions(code);self.assertTrue(defs['F_00000010_orig']['unhandled']);self.assertFalse(defs['F_00000010']['unhandled'])
    def test_reject_bad_sizes_and_empty_inventory(self):
        for records in [[],[{'bytes':0,'recompiled':True}]]:
            with self.assertRaises(ValueError):p.summarise(records)
    def test_stale_object_or_source_is_not_green(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp);base=root/'work/cpp-recomp-feasibility/data';base.mkdir(parents=True)
            config=root/'work/cpp-mainline';config.mkdir(parents=True)
            source=root/'generated.cpp';source.write_text('void F_00000010(CPUState *c) { return; }',encoding='utf-8')
            obj=root/'generated.obj';obj.write_bytes(struct.pack('<HH',0x8664,1)+bytes(56))
            (base/'functions.json').write_text(json.dumps({'functions':[{'start':16,'end':20}],'stats':{'code_bytes':4},'overlays':[]}))
            recipe={'units':[{'generated':True,'name':'generated','source':'generated.cpp','object':'generated.obj','source_sha256':p.digest(source),'object_sha256':p.digest(obj)}]}
            (config/'build-config.json').write_text(json.dumps(recipe))
            result,_=p.measure(root);self.assertEqual(result['recompiled_routines'],1)
            obj.write_bytes(obj.read_bytes()+b'changed')
            result,_=p.measure(root);self.assertEqual(result['recompiled_routines'],0)
            obj.write_bytes(struct.pack('<HH',0x8664,1)+bytes(56));source.write_text(source.read_text()+'\n// changed')
            result,_=p.measure(root);self.assertEqual(result['recompiled_routines'],0)
if __name__=='__main__':unittest.main()

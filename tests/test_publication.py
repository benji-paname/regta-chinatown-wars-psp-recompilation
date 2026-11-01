import hashlib, importlib.util, unittest
from pathlib import Path
spec=importlib.util.spec_from_file_location('gate',Path(__file__).resolve().parents[1]/'tools/maintenance/check_publication.py')
gate=importlib.util.module_from_spec(spec);spec.loader.exec_module(gate)
class PublicationTests(unittest.TestCase):
    def test_private_directories(self):
        self.assertTrue(gate.inspect('work/game.txt',b'text',{}))
        self.assertTrue(gate.inspect('docs/ms0/save.txt',b'text',{}))
    def test_case_insensitive_extensions(self):self.assertTrue(gate.inspect('docs/Game.ISO',b'text',{}))
    def test_mislabeled_executable(self):self.assertTrue(gate.inspect('runtime/readme.txt',b'MZ test',{}))
    def test_generated_source(self):self.assertTrue(gate.inspect('runtime/recomp_001.cpp',b'code',{}))
    def test_capture_not_allowed(self):self.assertTrue(gate.inspect('docs/assets/test.png',b'fake capture',{}))
    def test_exact_reviewed_brand(self):
        d=b'brand';p='docs/assets/logo.png'
        self.assertFalse(gate.inspect(p,d,{p:hashlib.sha256(d).hexdigest()}))
        self.assertTrue(gate.inspect(p,d+b'change',{p:hashlib.sha256(d).hexdigest()}))
    def test_public_code(self):self.assertFalse(gate.inspect('runtime/src/kernel_thread.cpp',b'// source',{}))
    def test_embedded_payload(self):self.assertTrue(gate.inspect('docs/assets/progress.svg',b'<image href="data:payload"/>',{}))
if __name__=='__main__':unittest.main()

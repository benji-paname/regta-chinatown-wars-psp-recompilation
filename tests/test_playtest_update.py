import importlib.util
import struct
import tempfile
import unittest
from pathlib import Path

spec = importlib.util.spec_from_file_location('updater', Path(__file__).resolve().parents[1] / 'tools/update_playtest.py')
u = importlib.util.module_from_spec(spec)
spec.loader.exec_module(u)

class UpdateTests(unittest.TestCase):
    def test_backup_and_private_files(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            package = root / 'package'; package.mkdir()
            source = root / 'new.exe'
            blob = bytearray(140); blob[:2] = b'MZ'; struct.pack_into('<I', blob, 60, 128); blob[128:134] = b'PE\0\0\x64\x86'
            source.write_bytes(blob)
            (package / 'ctw_game.exe').write_bytes(b'old executable')
            (package / 'ctw.ini').write_bytes(b'user preferences')
            (package / 'ms0').mkdir(); save = package / 'ms0/save'; save.write_bytes(b'user save')
            original = {str(p.relative_to(package)): p.read_bytes() for p in package.rglob('*') if p.is_file()}
            self.assertFalse(u.update(source, package, check=True)['updated'])
            self.assertEqual(original, {str(p.relative_to(package)): p.read_bytes() for p in package.rglob('*') if p.is_file()})
            result = u.update(source, package)
            self.assertEqual(Path(result['backup']).read_bytes(), b'old executable')
            self.assertEqual((package/'ctw_game.exe').read_bytes(), blob)
            self.assertEqual(save.read_bytes(), b'user save')
            self.assertEqual((package/'ctw.ini').read_bytes(), b'user preferences')
            self.assertFalse(u.update(source, package)['updated'])
    def test_reject_non_executable(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory); (root/'ctw_game.exe').write_bytes(b'preserve')
            bad = root/'bad'; bad.write_bytes(b'not executable')
            with self.assertRaises(ValueError): u.update(bad, root)
            self.assertEqual((root/'ctw_game.exe').read_bytes(), b'preserve')

if __name__ == '__main__': unittest.main()

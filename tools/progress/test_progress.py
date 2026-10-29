"""Integrity checks for the native-only status inventory."""
import tempfile
import unittest
from pathlib import Path
import progress


class InventoryTests(unittest.TestCase):
    def test_file_level_test_gate_keeps_staged_physics_out_of_runtime_count(self):
        self.assertEqual(progress.source_functions('#![cfg(test)]\nfn kernel() {}\n#[test] fn check() {}'), (0, 2))

    def test_comments_strings_and_trait_declarations_are_not_functions(self):
        source = '''// fn invented() {}
        /* nested /* fn nope() {} */ comment */
        trait Example { fn declaration(&self); }
        fn real() { let s = r###"fn fake() {}"###; let t = "fn wrong() {}"; }
        fn vector(p: [f32; 3]) -> [f32; 3] { p }
        '''
        self.assertEqual(progress.source_functions(source), (2, 0))

    def test_test_module_and_standalone_test_do_not_inflate_runtime(self):
        source = '''fn runtime() {}
        #[cfg(test)] mod tests { fn helper() {} #[test] fn regression() {} }
        #[test] fn standalone() {}
        #[cfg(test)] fn fixture() {}
        fn after_tests() {}
        '''
        self.assertEqual(progress.source_functions(source), (2, 4))

    def test_inventory_uses_only_rust_source_and_no_reference_totals(self):
        temp_base = progress.ROOT / 'target'
        temp_base.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(dir=temp_base) as d:
            root = Path(d)
            assert root.resolve().is_relative_to(temp_base.resolve())
            (root/'sub').mkdir()
            (root/'handling.rs').write_text('fn drive() {}')
            (root/'sub/helper.rs').write_text('fn helper() {}')
            (root/'reference.json').write_text('{"routines": 9999999}')
            (root/'vehicle.cpp').write_text('void external() {}')
            result = progress.measure_rust(root)
            self.assertEqual(result['runtimeFunctions'], 2)
            self.assertEqual(result['modules'], 2)
            self.assertIsNone(result['pspCompletionPercent'])
            self.assertFalse(result['pspParityVerified'])


if __name__ == '__main__':
    unittest.main()

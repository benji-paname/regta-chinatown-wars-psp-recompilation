import importlib.util
import unittest
import xml.etree.ElementTree as ET
from pathlib import Path
spec=importlib.util.spec_from_file_location('progress',Path(__file__).resolve().parents[1]/'tools/progress/cpp_treemap.py')
p=importlib.util.module_from_spec(spec);spec.loader.exec_module(p)
class ProgressTests(unittest.TestCase):
    def test_duplicate_stubs_use_last_record(self):
        records=[{'stub':1,'library':'old','name':'old'},{'stub':1,'library':'new','name':'new'}]
        result=p.measure(records,{('old','old')})
        self.assertEqual(result['raw_records'],2);self.assertEqual(result['selected_imports'],1)
        self.assertEqual(result['mapped'],0)
    def test_exact_library_and_name(self):
        records=[{'stub':1,'library':'A','name':'same'},{'stub':2,'library':'B','name':'same'}]
        result=p.measure(records,{('A','same')});self.assertEqual(result['percent'],50)
    def test_comments_do_not_add_fake_providers(self):
        code='// HLE(A, fake)\n/* HLE(B, fake) */\nHLE(A, real) {}\ninstall_hle_by_name("B", "other", fn);'
        self.assertEqual(p.providers(code),{('A','real'),('B','other')})
    def test_import_selection_and_hex_suffix(self):
        text='const ImportEntry g_import_list[] = {\n{ 0x1u, "A", 0xFFu, "one" },\n{ 0x1, "B", 0xAB, "two" },\n};'
        data=p.parse_imports(text);self.assertEqual(data[-1]['nid'],171)
        self.assertEqual(p.measure(data,{('B','two')})['mapped'],1)
    def test_tile_colors_match_counts(self):
        result=p.measure([{'stub':n,'library':'sceAudio','name':str(n)} for n in range(7)],{('sceAudio',str(n)) for n in range(4)})
        svg=ET.fromstring(p.treemap(result));tiles=[e.get('class') for e in svg.iter() if e.get('class')]
        self.assertEqual(tiles.count('mapped'),4);self.assertEqual(tiles.count('missing'),3)
        self.assertIn('not a percentage of gameplay',p.treemap(result))
    def test_empty_inventory_is_rejected(self):
        with self.assertRaises(ValueError):p.measure([],set())
if __name__=='__main__':unittest.main()

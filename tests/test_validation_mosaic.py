import copy,importlib.util,json,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('validation_mosaic',ROOT/'tools/progress/validation_mosaic.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)

class ValidationMosaicTests(unittest.TestCase):
    def setUp(self):
        self.groups=json.loads((ROOT/'docs/PLAYABLE_VALIDATION.json').read_text(encoding='utf-8'))['scenario_groups']
    def test_real_green_records_have_existing_evidence_sources(self):
        m.validate(self.groups)
        for group in self.groups:
            for check in group['checks']:
                if check['state'] in {'tested','observed'}:
                    self.assertTrue((ROOT/check['source']).is_file())
    def test_green_without_source_is_rejected(self):
        groups=copy.deepcopy(self.groups);groups[0]['checks'][0].pop('source')
        with self.assertRaises(ValueError):m.render(groups)
    def test_duplicate_or_invalid_state_is_rejected(self):
        for bad in ('duplicate','state'):
            groups=copy.deepcopy(self.groups)
            if bad=='duplicate':groups[0]['checks'][1]['id']=groups[0]['checks'][0]['id']
            else:groups[0]['checks'][0]['state']='compiled'
            with self.assertRaises(ValueError):m.render(groups)
    def test_mosaic_keeps_unknowns_and_issues_without_codegen_score(self):
        svg=m.render(self.groups)
        self.assertIn('class="pending"',svg);self.assertIn('class="issue"',svg)
        self.assertIn('#3fa34d',svg);self.assertIn('#a83d43',svg)
        self.assertNotIn('%',svg);self.assertNotIn('34,800',svg)
        self.assertIn('not a complete feature inventory',svg)

if __name__=='__main__':unittest.main()

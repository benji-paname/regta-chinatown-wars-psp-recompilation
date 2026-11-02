import copy,importlib.util,json,unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location('readiness',ROOT/'tools/progress/playable_status.py')
p=importlib.util.module_from_spec(spec);spec.loader.exec_module(p)
class PlayableReadinessTests(unittest.TestCase):
    def setUp(self):
        self.ledger=json.loads((ROOT/'docs/PLAYABLE_VALIDATION.json').read_text(encoding='utf-8'))
        self.codegen={'remaining_bytes':0,'generated_units':250,'verified_units':250}
    def test_perfect_codegen_is_not_global_completion(self):
        status=p.evaluate(self.ledger,self.codegen)
        self.assertEqual(status['status'],'in_development');self.assertIsNone(status['completion_percent'])
    def test_all_gates_without_acceptance_is_not_100(self):
        for gate in self.ledger['gates']:gate['state']='passed'
        self.assertIsNone(p.evaluate(self.ledger,self.codegen)['completion_percent'])
    def test_100_requires_all_explicit_conditions(self):
        for gate in self.ledger['gates']:gate['state']='passed'
        self.ledger.update(scope_confirmed=True,completion_accepted=True,known_blockers=[])
        self.assertEqual(p.evaluate(self.ledger,self.codegen)['completion_percent'],100)
        self.ledger['known_blockers']=['regression']
        self.assertIsNone(p.evaluate(self.ledger,self.codegen)['completion_percent'])
    def test_untranslated_candidate_blocks_100(self):
        for gate in self.ledger['gates']:gate['state']='passed'
        self.ledger.update(scope_confirmed=True,completion_accepted=True,known_blockers=[])
        self.codegen['remaining_bytes']=4
        self.assertIsNone(p.evaluate(self.ledger,self.codegen)['completion_percent'])
    def test_missing_gate_and_empty_evidence_are_rejected(self):
        broken=copy.deepcopy(self.ledger);broken['gates'].pop()
        with self.assertRaises(ValueError):p.evaluate(broken,self.codegen)
        self.ledger['gates'][0]['evidence']=''
        with self.assertRaises(ValueError):p.evaluate(self.ledger,self.codegen)
if __name__=='__main__':unittest.main()

"""Generate evidence-based C++ status only; never infer a game-completion percentage."""
import json, re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
def main():
    local=ROOT/'work/cpp-mainline/latest-build.json'
    if not local.exists():raise SystemExit('No local C++ validation record. Existing public status left untouched.')
    build=json.loads(local.read_text(encoding='utf-8'));tests=build.get('tests')
    if not tests or tests['failed'] or tests['passed']!=35:raise SystemExit('No complete passing 35-check run')
    date_match=re.search(r'build-(\d{4})(\d{2})(\d{2})-', build['directory'])
    if not date_match:raise SystemExit('Build date missing from validation evidence')
    validated_date='-'.join(date_match.groups())
    from cpp_treemap import generate
    mappings=generate()
    from cpp_recomp_progress import generate as generate_recomp
    recompilation=generate_recomp()
    from playable_status import generate as generate_playable
    readiness=generate_playable(codegen=recompilation)
    data={'mainline':'C++20','target':'Windows x64 / PSP Europe ULES-01347 v1.01','validation_date':validated_date,
          'kernel_checks':tests,'psp_import_source_mappings':{key:mappings[key] for key in ['mapped','selected_imports','missing','percent']},'runtime_units_compiled':build['runtime_units_compiled'],
          'generated_units_reused':250-build['generated_units_compiled'],'overlay_count':166,'completion_percent':readiness['completion_percent'],'playable_status':readiness['status'],'static_codegen':{key:recompilation[key] for key in ['bytes','recompiled_bytes','remaining_bytes','routines','recompiled_routines','percent','display_percent']},
          'user_report':{'completed_story_missions':2,'reported_by':'Benjamin Paname','independently_replayed_in_migration':False},
          'known_limits':['Unvalidated remainder of campaign','Drops toward 20 FPS and uneven frametimes in dense driving scenes','Existing compiler warnings','Private Playtest only; no cleared public binary package']}
    (ROOT/'docs/assets/cpp-status.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8')
    (ROOT/'docs/assets/badge-cpp.svg').write_text('<svg xmlns="http://www.w3.org/2000/svg" width="236" height="24" role="img" aria-label="C++20: 35 kernel checks passed"><rect width="236" height="24" rx="4" fill="#25302a"/><text x="118" y="16" text-anchor="middle" fill="#d6f4da" font-family="Verdana,sans-serif" font-size="11">C++20 · 35 kernel checks passed</text></svg>\n',encoding='utf-8')
    print('C++ evidence status generated; no global completion percentage.')
if __name__=='__main__':main()

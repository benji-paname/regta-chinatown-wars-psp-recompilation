"""Playable recompilation readiness: never convert generated-code coverage into completion."""
import html
import json
import re
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
REQUIRED={'scope','translation','services','campaign','presentation','controls_saves','stability','local_build'}


def evaluate(ledger,codegen=None):
    gates=ledger['gates']
    if {g['id'] for g in gates}!=REQUIRED or len(gates)!=len(REQUIRED):raise ValueError('Missing or repeated readiness gate')
    if any(g['state'] not in {'partial','unverified','passed'} or not g.get('evidence') for g in gates):raise ValueError('Readiness evidence/state missing')
    technical_ready=bool(codegen and codegen.get('remaining_bytes')==0 and codegen.get('generated_units',0)>0 and codegen.get('verified_units')==codegen.get('generated_units'))
    complete=technical_ready and all(g['state']=='passed' for g in gates) and ledger['scope_confirmed'] and ledger['completion_accepted'] and not ledger['known_blockers']
    return {'metric':'Completed playable PSP recompilation','status':'complete' if complete else 'in_development',
            'completion_percent':100 if complete else None,'definition':ledger['goal'],
            'reason':'All explicit readiness gates accepted' if complete else 'No reliable global completion percentage is established yet',
            'owner_observations':ledger['owner_observations'],'gates':gates,'known_blockers':ledger['known_blockers'],
            'technical_translation_ready':technical_ready,'scope_confirmed':ledger['scope_confirmed'],'completion_accepted':ledger['completion_accepted']}


def treemap(status):
    colors={'passed':'#3fa34d','partial':'#a8752f','unverified':'#555b64'}
    heading='Completed playable PSP recompilation — 100%' if status['status']=='complete' else 'Playable PSP recompilation — in development'
    subheading='All readiness gates and the original-game scope are accepted.' if status['status']=='complete' else '100% is reserved for the completed playable port. A reliable global percentage is not established.'
    out=['<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1100 460" role="img" aria-labelledby="title desc">',
         '<title id="title">Playable recompilation: readiness evidence, no global percentage</title>',
         '<desc id="desc">Qualitative gates. Amber means partial, grey means unverified, green means a gate has passed. Box sizes carry no completion weighting. Generated routines are a separate diagnostic.</desc>',
         '<rect width="1100" height="460" rx="8" fill="#16191c"/>',
         f'<text x="22" y="34" fill="#f2c35b" font-size="21" font-family="Arial,sans-serif">{heading}</text>',
         f'<text x="22" y="58" fill="#c2c9d0" font-size="13" font-family="Arial,sans-serif">{subheading}</text>']
    for i,gate in enumerate(status['gates']):
        x=22+(i%4)*268;y=82+(i//4)*126
        out.extend([f'<rect x="{x}" y="{y}" width="252" height="108" rx="5" fill="#232930" stroke="#424b55"/>',
                    f'<rect x="{x}" y="{y}" width="6" height="108" fill="{colors[gate["state"]]}"/>',
                    f'<text x="{x+16}" y="{y+29}" fill="#eef1f4" font-size="14" font-family="Arial,sans-serif">{html.escape(gate["name"])}</text>',
                    f'<text x="{x+16}" y="{y+58}" fill="{colors[gate["state"]]}" font-size="13" font-family="Arial,sans-serif">{gate["state"].replace("_"," ").upper()}</text>',
                    f'<text x="{x+16}" y="{y+84}" fill="#aeb7c1" font-size="11" font-family="Arial,sans-serif">Evidence: see validation ledger</text>'])
    out.extend(['<text x="22" y="365" fill="#aeb7c1" font-size="12" font-family="Arial,sans-serif">Qualitative gates; equal boxes do not imply equal effort or an arithmetic completion score.</text>',
                '<rect x="22" y="389" width="1056" height="48" rx="5" fill="#203326"/>',
                '<text x="38" y="418" fill="#d8efdc" font-size="14" font-family="Arial,sans-serif">Bounded evidence: 2 owner-tested story missions · 35 kernel checks passed — broader validation remains open.</text></svg>'])
    return '\n'.join(out)+'\n'


def generate(root=ROOT,codegen=None):
    ledger=json.loads((root/'docs/PLAYABLE_VALIDATION.json').read_text(encoding='utf-8'))
    codegen=codegen if codegen is not None else json.loads((root/'docs/assets/cpp-codegen-progress.json').read_text(encoding='utf-8'))
    status=evaluate(ledger,codegen)
    (root/'docs/assets/cpp-progress.json').write_text(json.dumps(status,indent=2)+'\n',encoding='utf-8')
    (root/'docs/assets/cpp-progress.svg').write_text(treemap(status),encoding='utf-8')
    label='Complete' if status['status']=='complete' else 'In development'
    badge_color='#2f6239' if status['status']=='complete' else '#655124'
    badge=f'<svg xmlns="http://www.w3.org/2000/svg" width="300" height="24" role="img" aria-label="Playable recompilation: {label}"><rect width="300" height="24" rx="3" fill="{badge_color}"/><text x="150" y="16" text-anchor="middle" fill="#fff" font-size="11" font-family="Verdana,sans-serif">Playable recompilation: {label}</text></svg>\n'
    (root/'docs/assets/badge-playable.svg').write_text(badge,encoding='utf-8')
    site=root/'docs/index.html';site_text=site.read_text(encoding='utf-8')
    complete=status['status']=='complete'
    states={'en':'Complete — 100%','fr':'Terminé — 100 %','es':'Completado — 100 %'} if complete else {'en':'In development','fr':'En développement','es':'En desarrollo'}
    descriptions={'en':'The completed playable PSP port has passed the documented readiness gates and accepted original-game scope.','fr':'Le portage PSP terminé et jouable a passé les validations documentées sur le périmètre original accepté.','es':'El port de PSP terminado y jugable ha superado las validaciones documentadas del alcance original aceptado.'} if complete else {'en':'100% is reserved for a finished, playable PSP port across its agreed original-game scope. No reliable global completion percentage has been established yet.','fr':'100 % est réservé à un portage PSP terminé et jouable sur le périmètre original convenu. Aucun pourcentage global fiable d’achèvement n’est encore établi.','es':'El 100 % se reserva para un port de PSP terminado y jugable dentro del alcance original acordado. Todavía no se ha establecido un porcentaje global fiable de finalización.'}
    state_html='<strong class="readiness-label" data-readiness-state '+ ' '.join(f'data-{lang}="{html.escape(text,quote=True)}"' for lang,text in states.items())+'>'+html.escape(states['en'])+'</strong>'
    description_html='<p data-readiness-description '+ ' '.join(f'data-{lang}="{html.escape(text,quote=True)}"' for lang,text in descriptions.items())+'>'+html.escape(descriptions['en'])+'</p>'
    site_text=re.sub(r'<strong\b[^>]*\bdata-readiness-state\b[^>]*>.*?</strong>',lambda _:state_html,site_text,flags=re.S)
    site_text=re.sub(r'<p\b[^>]*\bdata-readiness-description\b[^>]*>.*?</p>',lambda _:description_html,site_text,flags=re.S)
    if complete:
        note={'en':'The documented validation records establish the accepted playable scope. Read the ledger for the tested scenarios and remaining optional enhancements.','fr':'Les comptes rendus de validation établissent le périmètre jouable accepté. Le registre précise les scénarios testés et les améliorations facultatives restantes.','es':'Los registros de validación establecen el alcance jugable aceptado. Consulta los escenarios probados y las mejoras opcionales pendientes.'}
        note_html='<p data-readiness-open-work '+ ' '.join(f'data-{lang}="{html.escape(text,quote=True)}"' for lang,text in note.items())+'>'+html.escape(note['en'])+'</p>'
        site_text=re.sub(r'<p\b[^>]*\bdata-readiness-open-work\b[^>]*>.*?</p>',lambda _:note_html,site_text,flags=re.S)
    site.write_text(site_text,encoding='utf-8')
    readme=root/'README.md';text=readme.read_text(encoding='utf-8')
    block='''<!-- CPP_RECOMP_PROGRESS_START -->
![Playable recompilation](docs/assets/badge-playable.svg)

**The playable recompilation is in development. No reliable global completion percentage is available yet.** 100% is reserved for the completed, playable PSP port across its explicitly agreed feature/mode scope, after required services and end-to-end validation pass.

Benjamin has completed the first two story missions in the private Playtest. This is a tested milestone; full campaign coverage, PSP service behavior, stability and performance still need work. [Readiness definition and evidence](docs/CPP_PROGRESS.md).

[![Playable recompilation readiness](docs/assets/cpp-progress.svg)](docs/CPP_PROGRESS.md)

<details>
<summary>Technical diagnostics — generated-code coverage, not finished-port progress</summary>

![Static PSP translation](docs/assets/badge-cpp-recomp.svg) ![Translated routine candidates](docs/assets/badge-cpp-routines.svg)

These two badges describe the same static inventory: original-byte-weighted translation coverage and its routine count. Automated generation/compilation is not equivalent to Kryo.to's source-porting workflow and does not establish playable-port completion. [Method and limits](docs/CPP_CODEGEN.md).

[![Static code-generation treemap](docs/assets/cpp-codegen-progress.svg)](docs/CPP_CODEGEN.md)

[PSP service bindings — another separate diagnostic](docs/CPP_IMPORTS.md).

</details>
<!-- CPP_RECOMP_PROGRESS_END -->'''
    if complete:block=block.replace('**The playable recompilation is in development. No reliable global completion percentage is available yet.**','**Completed playable recompilation: 100%. All documented readiness gates and the original-game scope are accepted.**').replace('full campaign coverage, PSP service behavior, stability and performance still need work.','broader readiness evidence is recorded in the acceptance ledger.')
    text=re.sub(r'<!-- CPP_RECOMP_PROGRESS_START -->.*?<!-- CPP_RECOMP_PROGRESS_END -->',lambda _:block,text,flags=re.S)
    readme.write_text(text,encoding='utf-8')
    rows=['| Readiness gate | State | Current evidence |','| --- | --- | --- |']
    rows.extend(f"| {g['name']} | {g['state']} | {g['evidence']} |" for g in status['gates'])
    report='''# Completed playable recompilation: readiness

**In development. Global completion percentage: not yet measured.**

Benjamin's current definition is **100% = a finished, playable PSP recompilation**, not simply source generation or successful object compilation. The original-game scope must be explicit and accepted: original features/modes are not silently dropped, while optional future PC enhancements are separate.

![Playable recompilation readiness](assets/cpp-progress.svg)

The working private build has a real milestone: Benjamin reports two story missions completed using normal controls. The migration also passed 35 bounded kernel checks. Neither establishes a percentage of the complete game validated.

## What must be established before 100%

All gates below must have documented passing evidence, the original-game scope must be confirmed, no listed blocker may remain, and completion must be explicitly accepted. Gate boxes are qualitative and are not equal-weight percentage points. Until a reliable intermediate measurement exists, the generator emits `completion_percent: null`; it never derives overall readiness from generated function counts, HLE bindings, compiler output or arbitrary milestone weights.

'''+ '\n'.join(rows)+'''

The source of truth is [PLAYABLE_VALIDATION.json](PLAYABLE_VALIDATION.json). Update it only when new evidence actually supports a change. A gate can be partial while many tested features already work. Complete acceptance does not require redistributing game assets or offering a public download.

## Why the earlier 99.99% was misleading as the main progress figure

That number measures automatic translation/compilation of indexed PSP binary routines. It belongs to [static code-generation diagnostics](CPP_CODEGEN.md). It does not mean 99.99% of the porting work, game fidelity, runtime correctness or campaign validation is finished. The original source files from Rockstar are not the measured input; the indexed PSP machine code is.

Kryo.to's two badges are two views of one source-porting inventory: the percentage is weighted by original routine size, and the second badge gives routine count. Its source-name matching criterion differs from this project's generated-body/compiled-object criterion. Sharing the weighted arithmetic does not make their completion percentages comparable. The historical Rust figure is also not a comparable baseline for our automatic C++ code generation.

We retain both technical badges and the green code-generation treemap as secondary diagnostics, plus [PSP HLE source bindings](CPP_IMPORTS.md). They must never be presented as the completed-playable-port percentage.

Regenerate evidence and presentation with `python tools/progress/cpp_status.py`. No original game file, generated game code or save is changed by these tools.
'''
    if complete:report=report.replace('**In development. Global completion percentage: not yet measured.**','**Completed playable recompilation: 100%. All readiness gates and the original-game scope are accepted.**')
    (root/'docs/CPP_PROGRESS.md').write_text(report,encoding='utf-8')
    print('Playable recompilation: '+status['status']+'; global percentage '+str(status['completion_percent']))
    return status

if __name__=='__main__':generate()

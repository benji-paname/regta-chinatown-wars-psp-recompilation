# Completed playable recompilation: readiness

**In development. Global completion percentage: not yet measured.**

Benjamin's current definition is **100% = a finished, playable PSP recompilation**, not simply source generation or successful object compilation. The original-game scope must be explicit and accepted: original features/modes are not silently dropped, while optional future PC enhancements are separate.

![Visual status map](assets/cpp-status-map.svg)

A second, always-visible [dense PSP code mosaic](assets/cpp-code-map.svg) is shown in README and on the site. Its blue tiles describe generated/compiled routines and its areas follow original PSP byte sizes. It does not display a completion percentage or establish gameplay validation. The green cells in the functional map above describe bounded test observations.

The working private build has a real milestone: Benjamin reports two story missions completed using normal controls. The migration also passed 35 bounded kernel checks. Neither establishes a percentage of the complete game validated.

## What must be established before 100%

All gates below must have documented passing evidence, the original-game scope must be confirmed, no listed blocker may remain, and completion must be explicitly accepted. Gate boxes are qualitative and are not equal-weight percentage points. Until a reliable intermediate measurement exists, the generator emits `completion_percent: null`; it never derives overall readiness from generated function counts, HLE bindings, compiler output or arbitrary milestone weights.

| Readiness gate | State | Current evidence |
| --- | --- | --- |
| Original-game scope | unverified | A complete original PSP feature/mode checklist is not yet signed off. |
| Native translation | partial | 250 generated units verified; one unsupported candidate remains excluded. This only establishes static coverage. |
| PSP runtime services | partial | 35 bounded kernel checks pass. 203/252 source bindings are not proof of complete required PSP service behavior. |
| Campaign and activities | partial | Benjamin reports the first two story missions completed. Remaining missions, activities and mini-games lack comprehensive validation. |
| Rendering and audio | partial | Observed in tested sequences; broader PSP rendering/audio comparisons remain open. |
| Controls and saves | partial | Controller and savedata worked in bounded tests. Wider input and save/load regression coverage remains open. |
| Stability and performance | partial | Boot synchronization fixes are integrated; dense-driving frametimes remain irregular and repeatability needs broader testing. |
| Reproducible local setup | partial | The prepared Windows Playtest works; a complete documented fresh local setup from the owner ISO remains to be packaged. |

The source of truth is [PLAYABLE_VALIDATION.json](PLAYABLE_VALIDATION.json). Update it only when new evidence actually supports a change. A gate can be partial while many tested features already work. Complete acceptance does not require redistributing game assets or offering a public download.

## Why the earlier 99.99% was misleading as the main progress figure

That number measures automatic translation/compilation of indexed PSP binary routines. It belongs to [static code-generation diagnostics](CPP_CODEGEN.md). It does not mean 99.99% of the porting work, game fidelity, runtime correctness or campaign validation is finished. The original source files from Rockstar are not the measured input; the indexed PSP machine code is.

Kryo.to's two badges are two views of one source-porting inventory: the percentage is weighted by original routine size, and the second badge gives routine count. Its source-name matching criterion differs from this project's generated-body/compiled-object criterion. Sharing the weighted arithmetic does not make their completion percentages comparable. The historical Rust figure is also not a comparable baseline for our automatic C++ code generation.

We retain both technical badges and the green code-generation treemap as secondary diagnostics, plus [PSP HLE source bindings](CPP_IMPORTS.md). They must never be presented as the completed-playable-port percentage.

Regenerate evidence and presentation with `python tools/progress/cpp_status.py`. No original game file, generated game code or save is changed by these tools.

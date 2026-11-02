# C++ mainline status — 2 November 2026

The active implementation is native C++20 for Windows x64, targeting PSP Europe ULES-01347 v1.01.

| Evidence | What it establishes | Limit |
|---|---|---|
| Benjamin completed two story missions with normal controls | A playable local build in those sequences, including swimming, driving, mini-games, HUD and controller use | Owner-reported; not a full campaign test or independent replay during migration |
| 29 runtime components rebuilt and linked | Current canonical source builds with the prepared Clang/MSVC environment | Existing compiler warnings retained; no fresh-machine setup claim |
| 35 deterministic kernel checks passed | Covered mutex ownership, FIFO/priority, mailbox and StateIO cases | Not a complete PSP scheduler/hardware equivalence proof |
| 250 generated units and 166 overlays retained locally | Existing game-code compilation assets preserved | Generated code and all game data remain private |

Known: drops toward 20 FPS and uneven frametimes in dense driving scenes. Remaining campaign, compatibility and performance require testing. Unverified USA hooks/cheats must not be enabled on Europe. Mouse-driven mini-games are a future option, not implemented.

No public download or completion percentage. No gameplay captures published. [Migration evidence](CPP_MAINLINE_MIGRATION.md) · [Historical Rust inventory](legacy/progress.html).

## Completed playable recompilation readiness

**In development. A reliable global completion percentage is not yet established.** Benjamin's definition is 100% = a finished, playable PSP port across its agreed original feature/mode scope. See [CPP_PROGRESS.md](CPP_PROGRESS.md) and the evidence ledger [PLAYABLE_VALIDATION.json](PLAYABLE_VALIDATION.json).

The first two missions were completed by Benjamin; 35 bounded kernel checks passed. Required PSP service behavior, broader campaign/features, stability, performance and reproducible setup remain open. No arbitrary intermediate percentage is assigned from these milestones.

The former main 99.99% figure now appears only in [static code-generation diagnostics](CPP_CODEGEN.md), alongside its 34,800/34,801 routine count. It measures automatic translation and object compilation, not functional port completion. Kryo.to's code-porting badges use different inclusion criteria, despite similar original-byte-size weighting. The historical Rust figure is not an equivalent baseline either. [HLE source bindings](CPP_IMPORTS.md) remain separate.

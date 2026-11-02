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

## Original PSP code recompilation progress

The [main C++ treemap](CPP_PROGRESS.md) now measures **original PSP code recompiled, weighted by original routine size**, as requested by Benjamin. Current evidence: **34,800/34,801 indexed routines**, **12,307,572/12,307,576 original bytes**. It displays **99.99%**, truncated to avoid rounding known remaining work to 100.00%. One four-byte mapapp candidate still contains an unsupported-operation trap; earlier evidence suggests data, but it remains excluded pending reviewed classification.

This measures automatic static translation and verified object compilation across the indexed European executable and 166 overlays. It does not establish gameplay fidelity, campaign completion, all indirect callback targets or perfect code/data boundaries. The high figure reflects the automated recompilation process; runtime compatibility, performance and gameplay validation remain in progress.

[PSP HLE source mappings](CPP_IMPORTS.md), 203/252 = 80.56%, remain a separate diagnostic. The old Rust statistics remain historical and are not mixed into either C++ metric.

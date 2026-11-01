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

## Visual source-mapping progress

The [C++ treemap](CPP_PROGRESS.md) is restored: **80.56% (203/252)** selected PSP imports have exact source providers. Green means a provider declaration exists, including potentially partial implementations/stubs; grey means absent. This does not certify those services or score completion of the game. Counts regenerate from the current canonical runtime and private Europe import inventory. Historical Rust metrics remain separate.

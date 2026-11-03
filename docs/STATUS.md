# C++ mainline status — 3 November 2026

The active implementation is native C++20 for Windows x64, targeting PSP Europe ULES-01347 v1.01.

| Evidence | What it establishes | Limit |
|---|---|---|
| Benjamin completed five story missions and taxi activity with normal controls | Playable local build up to 'The Wheelman', with functional save/reload, swimming, driving, mini-games, HUD and controller | Owner-reported; not a full campaign test or independent replay during migration |
| 32 runtime components rebuilt and linked | Current canonical source builds with the prepared Clang/MSVC environment | Existing compiler warnings retained; no fresh-machine setup claim |
| 35 deterministic kernel checks passed | Covered mutex ownership, FIFO/priority, mailbox and StateIO cases | Not a complete PSP scheduler/hardware equivalence proof |
| 250 generated units and 166 overlays retained locally | Existing game-code compilation assets preserved | Generated code and all game data remain private |

Known: drops toward 20 FPS and uneven frametimes in dense driving scenes. Remaining campaign, compatibility and performance require testing. Unverified USA hooks/cheats must not be enabled on Europe. Mouse-driven mini-games are a future option, not implemented.

No public download or completion percentage. No gameplay captures published. [Migration evidence](CPP_MAINLINE_MIGRATION.md) · [Historical Rust inventory](legacy/progress.html).

## Completed playable recompilation readiness

**In development. A reliable global completion percentage is not yet established.** Benjamin's definition is 100% = a finished, playable PSP port across its agreed original feature/mode scope. See [CPP_PROGRESS.md](CPP_PROGRESS.md) and the evidence ledger [PLAYABLE_VALIDATION.json](PLAYABLE_VALIDATION.json).

Five story missions and a taxi activity were completed by Benjamin; save/load persistence is verified; 35 bounded kernel checks passed. Required PSP service behavior, broader campaign/features, stability, performance and reproducible setup remain open. No arbitrary intermediate percentage is assigned from these milestones.

The former main 99.99% figure now appears only in [static code-generation diagnostics](CPP_CODEGEN.md), alongside its 34,800/34,801 routine count. It measures automatic translation and object compilation, not functional port completion. Kryo.to's code-porting badges use different inclusion criteria, despite similar original-byte-size weighting. The historical Rust figure is not an equivalent baseline either. [HLE source bindings](CPP_IMPORTS.md) remain separate.


## HD replacement milestone — Validated in Gameplay (v0.2.0-alpha)

On 3 November 2026, Benjamin officially validated the HD texture replacement system in the native C++20 Playtest (`outputs/ReGTA-CTW-Playtest/ctw_game.exe`, SHA256: `6B48A3B29DD4DE510FD2A38E6DDD5BCB16BE2D1DAAF196781264DD96E5C1A7DA`). Three tests were confirmed in gameplay:
1. **Live toggle without crash**: Original PSP → HD Upscaled → Original PSP via the in-game PC options menu (F4) — **PASS**.
2. **Visible enhancements**: HD improvements confirmed on the minimap, texts, and HUD — **PASS**.
3. **Persistence**: Configuration (`hd_textures = 1`) preserved across game restarts — **PASS**.

**Limits**: Not all game textures are improved; individual pack asset verification, fallback behavior when the pack is absent, PSP lighting fidelity, and 60 FPS flickering remain under active development. Under the Alpha / Beta / Stable roadmap, the project remains at `v0.2.0-alpha`, with `v0.3.0-alpha` reserved for the completed graphics milestone once PSP lighting is validated. [Detailed validation report](HD_VALIDATION.md).

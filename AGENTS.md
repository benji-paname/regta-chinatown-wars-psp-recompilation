# ReGTA CTW — C++20 mainline instructions

## Canonical implementation (Benjamin's decision, 2026-11-02)

Work on `runtime/` by default: native C++20 PSP Europe ULES-01347 v1.01 recompilation for Windows x64. Do not restart gameplay reconstruction in Rust. `legacy/rust/src/` is the preserved historical implementation; its archive branch is `legacy/rust-pre-cpp-migration`. Root `Compiler.ps1` now invokes C++.

Benjamin reports completing the first two story missions in the private C++ Playtest using normal controls. This is a user playtest report, not exhaustive campaign validation. Drops toward 20 FPS and irregular frametimes in dense traffic remain a priority.

## Before every task

Run `python tools/maintenance/check_upstreams.py --task "Description"` from the root. Review BOTH official-kryo-to/ctw-native (Android reference) and highcmd/chinatown-wars-psp-recomp (PSP USA reference), then record relevant findings in `docs/UPSTREAM_REVIEW.md`. A fetch alone is not review. If DNS/network fails, record it; connected GitHub reads may independently verify remote revisions. Do not import upstream changes automatically. Mirrors must have their own .git and expected origin. Never fetch from `work/upstream-ctw`.

## Absolute private-data preservation

Keep ALL original game files and local Playtest data on Benjamin's PC, including ignored files, extracted data, textures, videos, music, executables and saves. Never delete these as cleanup, overwrite saves/preferences during updates, or force a locked directory rename. Reference mirrors, ISO/ELF and Ghidra projects remain untouched. Preserve uncommitted changes before reorganizing.

Publish NO proprietary game content: no ISO, PSP executable, generated/recompiled game translation units, extracted resources, texture packs, game audio/video, saves or test captures. All captures stay local per Benjamin's explicit instruction. Licensed project wordmarks may remain public; no font files. `work/`, `outputs/ReGTA-CTW-Playtest/`, and private build recipes stay ignored. `.gitignore` does not protect tracked files: check the index/tree AND outgoing Git objects before every push. Remove private files from tracking only, never from disk. Do not rewrite history destructively.

Publication gate: `python tools/maintenance/check_publication.py --tree HEAD --outgoing`. Install the versioned pre-push hook with `git config core.hooksPath .githooks`. Review actual content/provenance as well; an extension filter is not legal clearance. No commit/push unless authorized by the current user mission. The official repository and Pages URLs must not be renamed.

## Active layout and commands

- `runtime/src/`, `runtime/vfpu/`, `runtime/sas/`: canonical licensed runtime, provenance in `runtime/provenance.json` and `THIRD_PARTY.md`.
- `tests/kernel/mutex_scheduler.cpp`: 35-check real-runtime fixture; no game boot.
- `tools/build/cpp.py`, `tools/build/Cpp.cmd`, root `Compiler.ps1`: validated Clang 22.1.3/MSVC x64 C++20 build. No fast-math, no silent ABI/flag changes.
- `work/cpp-mainline/build-config.json`: private relocatable local recipe, generated from existing evidence, not a distributable game package.
- `work/cpp-recomp-feasibility/build/gen/`: 250 PRIVATE generated units; do not upload them or regenerate unnecessarily. Earlier reports/objects remain evidence.
- `outputs/ReGTA-CTW-Playtest/ReGTA-CTW-Playtest.exe`: normal local launcher; `ctw_game.exe`, image, DLLs, ms0 and ctw.ini stay together.
- `tools/update_playtest.py`: explicit local update; preserve saves, preferences and previous binary.
- `legacy/rust/src/`: byte-preserved Rust history. Old Cargo manifest remains under outputs/CTW-Rust for compatibility.

Build/check: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Check`.
Compile 29 runtime units, link locally and test: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test`.
This does not launch the game or replace the Playtest. `-FullGame` additionally recompiles the 250 private units only when needed. See `docs/BUILD_CPP.md` for local prerequisites; do not install/download dependencies automatically.

Keep compiler diagnostics. Current C++ builds have inherited warnings: do not claim zero warnings. Do not equate passing 35 tests with full PSP fidelity. Bink owner-exit cleanup and mutex/mailbox FIFO/priority selection are integrated; preserve them. USA hooks/cheats not verified for Europe must not be activated or remapped by guessed offsets. No fake HLE stubs or undocumented fallback changes.

## Status, website and handoff

`python tools/progress/cpp_status.py` generates C++ evidence status from actual local validation, without a completion percentage. Rust's `tools/progress/progress.py` writes only `docs/legacy/`; never put Rust metrics back on the main site or README. Never hand-edit generated status SVGs.

Public name: ReGTA CTW — Native PC Recomp; credit: by Benji Paname. README English; site English/French/Spanish. Keep translations consistent. No public download until redistribution rights/package are verified. Preserve GitHub, Discord and support links. Owner reports and automated test results must be separately labelled.

Read/update `docs/AI_HANDOFF.md` and `docs/CPP_MAINLINE_MIGRATION.md`. Historical reports can overstate fidelity: current evidence takes precedence. Preserve private reports rather than rewriting them. A new game run needs current authorization and a bounded supervisor; never reuse consumed-session markers.

## Ghidra

Use the existing CTW_Working project only with `-noanalysis -readOnly`, exact European function entries, and compare decompiler output with Allegrex instructions. No project writes, memory dumps, permission bypass or guessed USA-to-Europe offsets. See `docs/REVERSE_ENGINEERING.md` for reference paths.

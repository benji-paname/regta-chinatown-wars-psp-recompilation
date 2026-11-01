# ReGTA CTW PC Recomp — working instructions

## Before every new user task

Run from the repository root:

```powershell
python tools/maintenance/check_upstreams.py --task "Short description of the current task"
```

This is Benjamin's explicit requirement: consult BOTH official-kryo-to/ctw-native and highcmd/chinatown-wars-psp-recomp before starting a task. It also applies to branding, documentation and tooling tasks; do not force unrelated upstream changes into them. The report is docs/upstream-status.json. Read changed files / recent commits relevant to the task, then record the conclusion in docs/UPSTREAM_REVIEW.md. Fetching alone is not a review.

- If network access or a mirror is unavailable, record that limitation; do not claim a current upstream check succeeded. Continue independent work, but defer decisions that require unavailable evidence.
- The mirrors in references/ must have their own Git roots and exact expected origins. work/upstream-ctw is a historical file snapshot, NOT a Git mirror. Never run fetch/pull there: Git can fall back to this project's parent repository.
- Do not merge/pull upstream into our runtime automatically. Compare findings, record a pinned source commit and license, adapt compatible algorithms to Rust and verify against our PSP Europe ULES-01347 1.01 evidence. Android and PSP USA are references, not interchangeable data/addresses.
- Respect upstream license/attribution. Do not send messages, open issues or submit upstream PRs unless Benjamin authorizes that communication.

## Mandatory: Keep Visual Port Status & Treemap Up To Date After Every Significant Progress

> **BENJAMIN'S CORE MANDATE FOR ALL AI MODELS (GPT, Claude, Gemini, Copilot, Cursor, etc.)**:
> After **every significant progress, new gameplay system, module expansion, or refactoring**, the visual status treemap, SVG badges, and documentation dashboards **MUST be updated and kept current**.
> Never leave the repository with stale status visuals.

### Rules for Port Progress & Treemap Maintenance:
1. **Automated Synchronization**:
   - Running `.\Compiler.ps1 -Test` automatically re-measures the codebase and synchronizes the treemap, badges, `README.md`, and `docs/index.html`.
   - You can also run the generator directly:
     ```powershell
     python tools/progress/progress.py
     ```
2. **100% Native Rust Data Only (Absolute Integrity Rule)**:
   - The visual treemap (`docs/assets/progress.svg`), the SVG badges (`docs/assets/badge-overall.svg`, `docs/assets/badge-routines.svg`), and the subsystems breakdown table in `README.md` and `docs/index.html` **MUST reflect strictly and exclusively our native Rust codebase (`src/**/*.rs`)**.
   - **NEVER** scan, borrow, or mix in third-party C++ code (e.g. from `references/ctw-native`). Every single green block in the treemap must correspond to verified, real functions implemented in pure Rust in `src/`.
3. **Updating `RUST_MODULE_MAP`**:
   - When creating a new Rust file or adding new gameplay systems in `src/`, make sure `RUST_MODULE_MAP` in `tools/progress/progress.py` associates your module with the relevant Chinatown Wars classes.
   - The generator will immediately count your new Rust functions and allocate the corresponding routines and bytes in the treemap and badges.
4. **Never Hand-Edit Generated SVGs**:
   - `docs/assets/progress.svg`, `docs/assets/badge-overall.svg`, `docs/assets/badge-routines.svg`, and `docs/assets/progress.json` are generated artifacts. Always generate them through `python tools/progress/progress.py` or `.\Compiler.ps1 -Test`.

## Handoff

Keep outputs/CTW-Rust/AI_HANDOFF_GUIDE.md current. Preserve private game data and the latest film/logos. Public branding: ReGTA CTW PC Recomp; website/README default language: English. No emulated CPU in our Rust runtime; references may use different approaches.

---

## Project Objective & Scope

**ReGTA CTW — PC Recomp** is a clean-room, native Rust (Windows x64) port and reverse-engineering project for **Grand Theft Auto: Chinatown Wars** targeting the **European PSP release (`ULES-01347`, version 1.01)**.
- **Zero Emulated CPU at Runtime**: The executable (`outputs/CTW-Rust/CTW-Rust.exe`) executes 100% native x86_64 Rust code and reads assets directly from the user's legally owned PSP ISO (`USRDIR/GAME.PAK`, `USRDIR/WAD.TOC`, `USRDIR/OVERLAYS.PAK`, `USRDIR/SOUND/`).
- **Primary Reference Hierarchy**:
  1. **PSP Europe (`ULES-01347` v1.01)**: Primary ground truth for constants, memory offsets, bitflags, fixed-point Q12 math, and gameplay behavior.
  2. **`references/ctw-native` (`official-kryo-to/ctw-native`, Android 4.4.243 C++, GPL-3.0-only)** & `work/upstream-ctw` (local snapshot): Structural reference for class names, state machines, and high-level algorithms. Always verify constants and data formats against PSP Europe before adopting.
  3. **`references/chinatown-wars-psp-recomp` (`highcmd/chinatown-wars-psp-recomp`, PSP USA `ULUS-10490` v1.01)**: Reference for PSP GE display lists, overlay structure, and MIPS calling conventions (addresses differ from `ULES-01347`).

---

## Current Architecture & Key Files / Directories

### 1. Repository Layout
- `src/` — Single canonical source tree for the native Rust runtime and unit tests (~45 modules):
  - `src/main.rs` & `src/play.rs`: CLI entry point, application loop, fixed 30 Hz simulation step (`1.0 / 30.0`), input routing, and diagnostic/slice modes.
  - `src/iso.rs`, `src/psp.rs`, `src/wad.rs`, `src/pak.rs`, `src/overlays.rs`: ISO9660 reader, PSP archive (`WAD.TOC`, `GAME.PAK`, `OVERLAYS.PAK`), LZSS decompression, and 166 code overlays catalog.
  - `src/world.rs`, `src/model.rs`, `src/texture.rs`, `src/texture_pack.rs`, `src/timecycle.rs`: 3D city sectors, vertex normals, swizzled GE textures, HD pack overrides, and 55-channel PSP `timecycle.dat` interpolation.
  - `src/collision.rs`, `src/pavement.rs`, `src/ped_paths.rs`, `src/gps.rs`: Spatial grid collision (boxes, cylinders, triangles), 13,530 pavement nodes, road spline network (`ai.bin`), and A* GPS routing.
  - `src/actors.rs`, `src/ped.rs`, `src/roll.rs`, `src/vault.rs`: Scene coordinator, player health/armour (`damage_player`, `reduce_armour`, `MAX_ARMOUR = 127`), pedestrian AI, PSP combat roll, and obstacle vaulting.
  - `src/vehicle.rs`, `src/handling.rs`, `src/traffic.rs`, `src/driving_audit.rs`: PAK 29 vehicle kinematics, Q12 suspension sway/pitch, on-rail spline traffic, junction curves, and 166-route real-map driving audit.
  - `src/camera.rs` (`src/car_camera.rs`): On-foot (`FollowPedCam`) and vehicle (`FollowCarCam2`) cameras supporting PSP, DS, and Mobile profiles plus PSP cardinal-axis steering assist.
  - `src/weapons.rs`, `src/explosions.rs`, `src/police.rs`, `src/props.rs`, `src/psp_prop_physics.rs`: Weapons, projectiles, fire patches, explosion shockwaves, LCPD cop takedowns/roadblocks/helicopters, breakable/loose street furniture, and the 512-case PSP rigid-body force oracle (`#[cfg(test)]`).
  - `src/missions.rs`, `src/jobs.rs`, `src/drugs.rs`, `src/safehouse.rs`, `src/collectibles.rs`, `src/races.rs`: Story mission engine, emergency/odd jobs & perks (`JobPerks`, including `double_armour` & `armour_half_tick`), 81-dealer drug economy, 21 safehouses, cameras/rampages/stunt jumps, and street races.
  - `src/gpu.rs`, `src/hud.rs`, `src/radar.rs`, `src/pda.rs`, `src/ui.rs`, `src/locale.rs`, `src/audio.rs`, `src/radio.rs`, `src/weather.rs`, `src/stats.rs`: OpenGL renderer, planar world shadows, PSP radar/HUD sprites, PDA apps, FR/EN localization, audio mixer, 11 radio stations, weather cycle, and bitfield statistics.
- `outputs/CTW-Rust/` — Cargo package manifest (`outputs/CTW-Rust/Cargo.toml`), delivered executable (`CTW-Rust.exe`), launchers, and `AI_HANDOFF_GUIDE.md`.
- `tools/` — Build (`tools/build/Compile.ps1`), maintenance (`check_upstreams.py`, `check_repository.py`), progress generator (`tools/progress/progress.py`), research scripts (`tools/research/`), and versioned Ghidra scripts (`tools/ghidra/CtwQuery.java`).
- `docs/` — Technical handoff (`docs/AI_HANDOFF.md`), reverse-engineering manual (`docs/REVERSE_ENGINEERING.md`), upstream logs (`docs/UPSTREAM_REVIEW.md`, `docs/upstream-status.json`), and subsystem evidence (`docs/reference/`).
- `work/` & `target/` — Ignored private directories containing the local PSP ISO (`work/Chinatown-Wars-PSP.iso`), decrypted ELF (`work/psp-analysis/CTW-PSP.elf`), Android snapshot (`work/upstream-ctw`), and Cargo build cache (`target/`).

---

## Build, Test & Validation Commands

### 1. Canonical Full Build + Test + Dashboard Sync
From the repository root in PowerShell:
```powershell
powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test
```
- Automatically locates the system Cargo or the local portable Rust 1.90.0 GNU toolchain (`..\2026-10-05\en\work\rust-portable`).
- Enforces `RUSTFLAGS="-D warnings"` (zero compiler warnings allowed).
- Runs all deterministic unit tests (currently **248 tests**), copies `CTW-Rust.exe` to `outputs/CTW-Rust/CTW-Rust.exe`, writes `target/validation/build-report.json`, and updates the progress treemap/badges.

### 2. Targeted / Code-Only Build & Test (Without Modifying `CTW-Rust.exe` or SVGs)
When performing localized code verification without touching delivered binaries or generated SVG dashboards:
```powershell
$repoRoot = "C:\Users\BENJAMIN\Documents\Codex\2026-10-17\tu-voit-le-projet-vcs-rust"
$toolRoot = "C:\Users\BENJAMIN\Documents\Codex\2026-10-05\en\work\rust-portable"
$env:PATH = "$toolRoot\bin;$toolRoot\lib\rustlib\x86_64-pc-windows-gnu\bin\self-contained;" + $env:PATH
$env:CARGO_HOME = "C:\Users\BENJAMIN\Documents\Codex\2026-10-05\en\work\cargo-home"
$env:CARGO_TARGET_DIR = "$repoRoot\target"
$env:RUSTFLAGS = "-D warnings"
& "$toolRoot\bin\cargo.exe" test --release --locked --offline --manifest-path "$repoRoot\Cargo.toml"
```

### 3. Real-ISO Smoke & Driving Audit
```powershell
.\target\release\ctw-rust.exe --iso work/Chinatown-Wars-PSP.iso --driving-audit --output target/driving-audit
```

---

## Ghidra 12.2 + `ghidra-allegrex` v21.5 Infrastructure

To inspect original PSP Europe (`ULES-01347` v1.01) functions, always query the analyzed Ghidra project (`C:\Tools\ghidra_projects\CTW_Working.gpr`, program `/CTW-PSP.elf`, **11,228 functions**, **94,152 symbols**, **327,185 xrefs**, **1,111 VFPU candidate words**) in **read-only, no-analysis mode** using `CtwQuery.java` (`C:\Tools\ghidra_scripts\CtwQuery.java`, versioned identically in `tools/ghidra/CtwQuery.java`):

```powershell
& "C:\Tools\ghidra_12.2_PUBLIC\support\analyzeHeadless.bat" `
  "C:\Tools\ghidra_projects" "CTW_Working" `
  -process "CTW-PSP.elf" `
  -noanalysis -readOnly `
  -scriptPath "C:\Users\BENJAMIN\Documents\Codex\2026-10-17\tu-voit-le-projet-vcs-rust\tools\ghidra" `
  -postScript CtwQuery.java 0x089c967c 0x089c7c50
```
- **Critical Precaution**: Always pass `-noanalysis -readOnly` so `CTW_Working.gpr` and `CTW-PSP.elf` are never modified or locked.
- Always pass exact function entry points (verify prologues `addiu sp, sp, -N` if following raw offsets) and verify decompiled pseudocode against the Allegrex disassembly output (especially for delay slots, untyped `$v0` return registers, bitfield shifts, and VFPU instructions).
- Full details on Ghidra, REA 6.8.0, and FreeLLMAPI are documented in `docs/REVERSE_ENGINEERING.md`.

---

## Mandatory Precautions & Anti-Regression Rules

1. **Preflight Upstream Check**: Always run `python tools/maintenance/check_upstreams.py --task "..."` and update `docs/UPSTREAM_REVIEW.md` before starting work.
2. **Never Commit or Push Without Explicit User Authorization**: Do not run `git commit`, `git push`, or `git reset` unless Benjamin explicitly asks for it.
3. **Never Add Proprietary Binaries or Secrets to Git**: Never stage `.iso`, `.elf`, `.pak`, `.wad`, `.bin`, `.mp4`, or any API keys/tokens.
4. **Localized, Evidence-Backed Changes**: Preserve all existing working subsystems. Do not rewrite unrelated modules or replace verified PSP behavior with unverified heuristics.
5. **Zero Warnings & Full Test Pass**: Every Rust modification must compile with `-D warnings` (remember `#[cfg_attr(not(test), allow(dead_code))]` for helpers only called in `#[cfg(test)]` inside this binary crate) and pass the entire unit test suite.
6. **Honest Status Reporting**: Distinguish clearly between PSP-verified behavior, Android reference adaptations, and prototype approximations. See `docs/AI_HANDOFF.md` for the complete current state.
7. **Strategic Horizon (Future C++17 Static Recompilation)**: Our long-term strategic plan is a future transition toward a C++17 static recompilation of the European PSP binary (`ULES-01347` v1.01) and its 166 overlays based on `highcmd/chinatown-wars-psp-recomp` (see `docs/CPP_TRANSITION_PLAN.md`). **This transition is NOT yet authorized for execution**: preserve 100% of the Rust port, never perform automatic Rust-to-C++ conversion, and do not start the C++ prototype without explicit authorization from Benjamin.


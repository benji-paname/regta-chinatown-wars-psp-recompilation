# ReGTA Chinatown Wars PC — AI Technical Handoff (`docs/AI_HANDOFF.md`)

> **Current isolated C++ experiment (2026-11-01):** Approved Bink owner-exit fix validated in an experimental copy: 27 actual C++ checks pass; three known FIFO/priority-policy differences remain. Three bounded boots reached autosave and first-mission intro, including one direct cleanup308-to-audio309 recovery of the formerly blocked interleaving. All3 runs consumed; STOP pending new validation. Read `work/cpp-recomp-feasibility/reports/BINK_MUTEX_FIX_VALIDATION.md` and `outputs/CTW-Rust/AI_HANDOFF_GUIDE.md`. GO recommendation for a future controlled integration only; originals intact, no full PSP-fidelity claim. Rust status below is historical and was not revalidated by this C++ mission.

**Last Updated**: 2026-10-31  
**Target Release**: Grand Theft Auto: Chinatown Wars — PSP Europe (`ULES-01347`, v1.01)  
**Runtime Architecture**: 100% Native Rust (`x86_64-pc-windows-gnu`), OpenGL, Zero Emulated CPU at Runtime  
**Current Validation**: **248 passing unit tests, 0 failures, 0 ignored, 0 compiler warnings (`-D warnings`)**

---

## 1. Real State of the Rust Port

ReGTA CTW is an independent native PC port and reverse-engineering project reading assets directly from the user's legally owned European PSP ISO (`work/Chinatown-Wars-PSP.iso`).

> **Honesty & Provenance Rule**: The codebase combines **PSP-verified subsystems** (verified directly against `ULES-01347` binary code/data via Ghidra 12.2 + `ghidra-allegrex` v21.5 and Unicorn differential tests), **structural adaptations** from the GPL-3.0-only Android reference (`references/ctw-native`), and **playable prototype implementations**. Passing unit tests (248) and source function counts measure local engineering progress, **not** a percentage of full PSP game equivalence.

---

## 2. Implemented Features (By Subsystem)

### A. Archive, World & Asset Pipeline (PSP-Verified)
- **ISO9660 & Archive Readers** (`src/iso.rs`, `src/psp.rs`, `src/wad.rs`, `src/pak.rs`, `src/overlays.rs`):
  - Direct parsing of `USRDIR/WAD.TOC`, `USRDIR/GAME.PAK`, `USRDIR/OVERLAYS.PAK` (all **166 PSP code overlays** indexed), and LZSS decompression.
- **World Geometry, Models & Textures** (`src/world.rs`, `src/model.rs`, `src/texture.rs`, `src/texture_pack.rs`):
  - Full Liberty City sector streaming, 368 street furniture prop meshes (PAK 1866), 271 prop textures/animations (PAK 1869..2163), PSP GE swizzled texture decoding, signed-byte vertex normal decoding (`bytes +2..4 / 128`), and optional HD texture pack replacement.
- **Collision & Navigation Graphs** (`src/collision.rs`, `src/pavement.rs`, `src/ped_paths.rs`, `src/gps.rs`):
  - Spatial cell collision (boxes, cylinders, spheres, triangles), solid street prop collision (`Collision::populate_prop_shapes`, `set_prop_solid`), **13,530 PSP pavement nodes** with **9,654 sector bridges**, road spline network (`ai.bin`), and A* GPS pathfinding.

### B. Player Locomotion, Cameras & Combat Vitals (PSP-Verified + Prototype Combat)
- **Locomotion, Roll & Vault** (`src/ped.rs`, `src/roll.rs`, `src/vault.rs`):
  - Fixed 30 Hz sub-stepped walker (`Walker::step`), PSP directional combat roll impulse (`src/roll.rs`), and contextual obstacle vaulting (`src/vault.rs`).
- **On-Foot & Vehicle Cameras** (`src/camera.rs`, `src/car_camera.rs`):
  - `FollowPedCam` and `FollowCarCam2` supporting PSP, Nintendo DS, and Mobile framing profiles (`F3` / right-stick click), progressive reverse orbit after 31 ticks of sustained backward velocity, and PSP cardinal-axis steering assistance (`FillInDrivingYoke` at `0x089cb264..0x089cb720`, toggled via `F6`).
- **Health, Armour & Double Armour Bonus** (`src/actors.rs`, `src/jobs.rs`, `src/radar.rs`):
  - Verified against PSP `cPlayer::ReduceArmour` (`0x089c967c`), `cPed::AddArmour` (`0x089c7c50`), `cPed::SetArmour` (`0x089c7d08`), `cPlayer::FixDamageForArmour` (`0x089c8f20`), `cPlayer::SetHasDoubleArmour` (`0x089c8930`), and `cPlayer::Reset` (`0x089c8ff4`):
    - Hardware/gameplay ceiling fixed at **`MAX_ARMOUR = 127` (`0x7f`)**.
    - With Double Armour active (`JobPerks::double_armour`, bit `0x2000` of `flags_f14`, unlocked at Vigilante Wave 5):
      - Incoming damage $\ge 2$ is halved via unsigned integer shift (`damage >>= 1`) without modifying the alternation bit.
      - Incoming damage of `1` point alternates via `JobPerks::armour_half_tick` (bit `0x4000` of `flags_f14`), producing the exact sequence **`0, 1, 0, 1, 0, 1...`** (first 1-point hit absorbed for 0 damage).
      - Halving and alternation occur before comparing against remaining armour, so overflow damage to health and hits when `armour == 0` are also halved/alternated.
      - `JobPerks::set_double_armour` and `JobPerks::reset_damage_state` clear `armour_half_tick` (`0x4000`).

### C. Vehicles, Handling & Ambient Traffic (`PAK 29` + `ctw-native` Adaptation)
- **Drivetrain, Tyres & Suspension** (`src/handling.rs`, `src/vehicle.rs`):
  - Live PSP `PAK 29` handling parameters (mass, drag, tyre grip, drive bias, 6 suspension sway constants `sway_k`), Q12 body roll and braking/acceleration pitch (`cWheeledVehicle::UpdateSuspension`), standing burnout + release launch impulse, handbrake drift, and watercraft buoyancy.
- **Spline Traffic & Avoidance** (`src/traffic.rs`, `src/gps.rs`, `src/driving_audit.rs`):
  - On-rail spline progression, traffic light stop/go cycles, `AISpline::computeIntercept` junction curve clamping with cubic Hermite fallback, `rail_body_clear` (ignoring decorative sidewalk tree cylinders), lateral obstacle avoidance, proximity horn alerting, gunfire panic flee, and frustum-guarded off-screen streaming.
  - Validated across **92 vehicles and 166 real-map routes with 0 world obstructions** (`--driving-audit`).

### D. Street Furniture Props, Weapons, Police & World Systems
- **Street Furniture & Prop Physics** (`src/props.rs`, `src/psp_prop_physics.rs`):
  - Synchronization and OpenGL rendering of all city props, broken mesh swapping, fire hydrant geysers, toppling lampposts, exploding gas pumps/propane barrels, shattering phone booths/bus shelters, and loose tumbling mailboxes/newspaper dispensers/dumpsters with loot drops.
  - Offline/test-only exact transcription of the PSP rigid-body force/damping core (`0x089b5960`) in `src/psp_prop_physics.rs`, verified across 512 deterministic states against Unicorn execution of the original PSP ELF.
- **Weapons, Projectiles & Explosions** (`src/weapons.rs`, `src/explosions.rs`):
  - Melee, firearms, sniper scope with breath hold, flamethrower & fire patches (`FireManager`), grenades/Molotovs/flashbangs/mines/RPG (`ProjectileManager`), shockwave blasts (`ExplosionManager`), and world/dropped pickups (`PickupManager`).
- **Police & Wanted System** (`src/police.rs`):
  - 1–6 wanted stars, cop cruiser ramming takedowns, roadblocks with tire-puncturing spike strips (3+ stars), and LCPD helicopter with tracking searchlight and shootdown spiral.
- **Gameplay Content & UI** (`src/missions.rs`, `src/jobs.rs`, `src/drugs.rs`, `src/safehouse.rs`, `src/collectibles.rs`, `src/races.rs`, `src/pda.rs`, `src/hud.rs`, `src/gpu.rs`, `src/timecycle.rs`, `src/audio.rs`, `src/radio.rs`, `src/weather.rs`, `src/stats.rs`):
  - 8 Boss contact mission frameworks, 5 Odd Jobs (Taxi, Vigilante, Firefighter, Paramedic, Noodle Delivery), 81 Drug Wars dealers across 6 commodities, 21 safehouses, 100 security cameras / 35 rampages / 30 stunt jumps, 16 street races, contextual minigames, PSP radar & HUD sprites, PSP 55-channel `timecycle.dat` loader, first-pass planar world/prop stencil shadows, and 11 synthesized/streamed radio stations.

---

## 3. Features Still Missing or Incomplete

1. **Player Respawn Wiring & Wasted/Busted Flow (`src/play.rs`, `src/missions.rs`)**:
   - Currently, all 4 active player damage sources in `src/actors.rs` (single-bike crash, bike-vs-car collision, hostile pedestrian melee, and explosion blasts) use the verified `damage_player` Double Armour logic.
   - However, when player health reaches `0` (`"HORS COMBAT"` in `src/gpu.rs:2795`) and the player presses `R` / `LB+Y` in `src/play.rs:996-1029`, `play.rs` instantiates a fresh `Scene::new(...)` without carrying over `prev_scene.jobs.perks.double_armour` via `s.jobs.perks.set_double_armour(had_double_armour)` (which in PSP `0x08897e78` preserves bit `0x2000` and resets bit `0x4000`). Similarly, `MissionEngine::trigger_wasted` and `trigger_busted` (`src/missions.rs:789-831`) are tested in isolation but not yet automatically triggered when `scene.health <= 0.0` in `play.rs`.
2. **Additional Player Damage Sources (NPC Gunfire, Fire Patches, Drowning)**:
   - Hostile NPCs currently attack the player via melee (`src/actors.rs:2310`); NPC-to-player firearm shooting and player burn damage from `FireManager` patches (`src/weapons.rs:1002`) are not yet wired to `damage_player`.
3. **PSP Visual & Lighting Parity (`src/gpu.rs`, `src/timecycle.rs`)**:
   - While `src/timecycle.rs` reads the real 42,240-byte PSP `timecycle.dat` and `src/gpu.rs` renders first-pass planar stencil shadows for world meshes and standing props, full PSP GE lighting parity (exact GE ambient/diffuse material coefficients, second directional light, dynamic car/ped/loose-prop shadows, and warm sunlight balance) is **not yet achieved**. See `docs/reference/PSP-VISUAL-PARITY.md`.
4. **Coupled PSP Rigid-Body Contact & Quaternion Solver (`src/props.rs`, `src/psp_prop_physics.rs`)**:
   - The PSP velocity/damping force core (`0x089b5960`) is 100% verified in `src/psp_prop_physics.rs` (`#[cfg(test)]`), but runtime loose props in `src/props.rs` still use the repaired native approximation because the coupled PSP position/quaternion integration and `FullCollision`/`FullSprings` contact solver have not yet been fully decompiled and connected. See `docs/reference/PSP-PROP-PHYSICS.md`.
5. **Original PSP Overlay Scripting & Full Campaign / Save Compatibility**:
   - Story missions, PDA menus, and odd jobs are clean-room Rust state machines rather than decompiled executions of the 166 `OVERLAYS.PAK` binaries.
   - Four optional lookup tables in `data/tables/` (weapons, radio, weather, sound) still originate from the Android reference installation and await replacement with direct PSP binary reads.

---

## 4. Latest Modifications & Discoveries (31 October 2026)

1. **Validated Ghidra 12.2 + `ghidra-allegrex` v21.5 + `CtwQuery.java` Pipeline**:
   - Full analysis of `CTW-PSP.elf` in `C:\Tools\ghidra_projects\CTW_Working.gpr` (**11,228 functions, 94,152 symbols, 327,185 xrefs, 1,111 VFPU candidate words**).
   - Versioned `CtwQuery.java` in `tools/ghidra/CtwQuery.java` (identical to `C:\Tools\ghidra_scripts\CtwQuery.java`).
2. **Faithful PSP Double Armour Implementation (`src/jobs.rs`, `src/actors.rs`)**:
   - Decompiled and verified `cPlayer::ReduceArmour` (`0x089c967c`), `cPed::AddArmour` (`0x089c7c50`), `cPed::SetArmour` (`0x089c7d08`), `cPlayer::FixDamageForArmour` (`0x089c8f20`), `cPlayer::SetHasDoubleArmour` (`0x089c8930`), `cPlayer::Reset` (`0x089c8ff4`), and hospital/arrest respawn (`0x08897e78`).
   - Implemented `MAX_ARMOUR = 127`, `set_armour`, `add_armour`, `reduce_armour`, updated `damage_player`, and added `JobPerks::armour_half_tick`, `JobPerks::set_double_armour`, and `JobPerks::reset_damage_state`.
   - Added 10 non-regression test scenarios in `src/actors.rs` and `src/jobs.rs` (**248 total tests passing, 0 warnings**).

---

## 5. Strategic Horizon: Future Transition Toward C++17 Static PSP Recompilation (`docs/CPP_TRANSITION_PLAN.md`)

> [!IMPORTANT]
> **Current State vs. Future Strategic Objective**  
> - **Current Active Codebase**: The **Native Rust Port (`src/`)** is the active, working baseline of the repository (**248 passing tests, 0 warnings**). It must be **preserved in full** (code, history, tests, and build pipeline).  
> - **Future Strategic Target**: We are preparing a future transition toward a **C++17 static recompilation architecture** of the European PSP binary (`ULES-01347` v1.01) and its 166 overlays, modeled on [`highcmd/chinatown-wars-psp-recomp`](https://github.com/highcmd/chinatown-wars-psp-recomp) (`ULUS-10490` v1.01).  
> - **Strict Guardrails**:  
>   1. This transition is **NOT YET AUTHORIZED FOR EXECUTION**. Do **not** begin coding the C++ prototype unless explicitly instructed by Benjamin.  
>   2. Never perform an automated Rust-to-C++ code conversion.  
>   3. Before any migration is considered, it must undergo a **feasibility study and an isolated C++ prototype** evaluated against strict criteria (boot, intro, world rendering, gameplay, and full completion of the first mission compared against PPSSPP and our Rust version).  
>   4. Read the complete plan in **[`docs/CPP_TRANSITION_PLAN.md`](CPP_TRANSITION_PLAN.md)**.

---

## 6. Development Priorities (Next Recommended Tasks)

### A. On the Active Native Rust Port (`src/`)
1. **Respawn & Player Damage Completeness**:
   - Wire `src/play.rs` (`R` / `LB+Y` respawn and automatic Wasted/Busted hospital/station respawn) so that `had_double_armour` is preserved via `s.jobs.perks.set_double_armour(had_double_armour)` (keeping `0x2000` and clearing `0x4000`), and route player fire/NPC gunshot damage through `damage_player`.
2. **PSP GE World Lighting & Shadow Recovery**:
   - Use Ghidra (`CtwQuery.java`) on `CTW_Working.gpr` to locate the exact PSP GE world material and light setup routines (avoiding allocator poison `0x0885fde0` and non-world pass `0x08982090`) and original shadow caster/receiver rules.
3. **Coupled PSP Prop Physics Solver**:
   - Decompile the callers/callees of `0x089b5960` (`FullCollision`, `FullSprings`, quaternion integration) via Ghidra, extend `tools/research/psp_prop_oracle.py`, and enable the full PSP solver in `src/props.rs`.
4. **PSP Table Provenance**:
   - Replace the 4 optional Android reference tables (`weapons`, `radio`, `weather`, `sound`) with direct parsers of `CTW-PSP.elf` / `GAME.PAK`.

### B. On the Strategic C++17 Static Recompilation Track (When Authorized — See `docs/CPP_TRANSITION_PLAN.md`)
1. **Feasibility Audit (`ULUS-10490` vs. `ULES-01347`)**:
   - Use Ghidra (`CTW_Working.gpr` + `CtwQuery.java`) to map `highcmd`'s `ULUS-10490` hook addresses (`game_hooks.cpp`, `data/cheats.json`) and `.overlays` slot boundaries to `ULES-01347` v1.01.
2. **Isolated Prototype (Only Upon Explicit Authorization)**:
   - Adapt `highcmd`'s `setup_from_iso.py`, `analyze.py`, and `recomp.py` in an isolated prototype folder (`work/`) for `ULES-01347` (`CTW_PSP_UMD_MASTER_SCEE/OVERLAYS.PAK`), without touching the Rust port.

---

## 7. Immediate Onboarding Procedure for Any AI Agent

When opening this repository in a new session (ChatGPT/Codex, Gemini, Claude, Cursor, or Antigravity), execute the following sequence:

### Step 1: Read Project Instructions & Handoff State
```powershell
Get-Content AGENTS.md
Get-Content docs/AI_HANDOFF.md
Get-Content docs/REVERSE_ENGINEERING.md
git status -s
```

### Step 2: Run the Mandatory Upstream Preflight
```powershell
python tools/maintenance/check_upstreams.py --task "Brief description of your task"
```
Inspect any new commits in `references/ctw-native` and `references/chinatown-wars-psp-recomp`, and log your findings in `docs/UPSTREAM_REVIEW.md`.

### Step 3: Query PSP Functions in Read-Only Mode via Ghidra
Whenever your task involves verifying original PSP behavior, run `CtwQuery.java` against `CTW_Working.gpr` with `-noanalysis -readOnly`:
```powershell
& "C:\Tools\ghidra_12.2_PUBLIC\support\analyzeHeadless.bat" `
  "C:\Tools\ghidra_projects" "CTW_Working" `
  -process "CTW-PSP.elf" `
  -noanalysis -readOnly `
  -scriptPath "C:\Users\BENJAMIN\Documents\Codex\2026-10-17\tu-voit-le-projet-vcs-rust\tools\ghidra" `
  -postScript CtwQuery.java 0x089c967c
```

### Step 4: Compile & Run the Rust Test Suite (`-D warnings`)
- For code + test verification without overwriting delivered artifacts/SVGs:
  ```powershell
  $repoRoot = "C:\Users\BENJAMIN\Documents\Codex\2026-10-17\tu-voit-le-projet-vcs-rust"
  $toolRoot = "C:\Users\BENJAMIN\Documents\Codex\2026-10-05\en\work\rust-portable"
  $env:PATH = "$toolRoot\bin;$toolRoot\lib\rustlib\x86_64-pc-windows-gnu\bin\self-contained;" + $env:PATH
  $env:CARGO_HOME = "C:\Users\BENJAMIN\Documents\Codex\2026-10-05\en\work\cargo-home"
  $env:CARGO_TARGET_DIR = "$repoRoot\target"
  $env:RUSTFLAGS = "-D warnings"
  & "$toolRoot\bin\cargo.exe" test --release --locked --offline --manifest-path "$repoRoot\Cargo.toml"
  ```
- For a full deliverable build + test + treemap/badge update:
  ```powershell
  powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test
  ```

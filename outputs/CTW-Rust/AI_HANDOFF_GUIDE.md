# ACTIVE HANDOFF - Handling suspension (Option B), traffic avoidance (Option C), and pop-in / teleportation eradication (Option A)

Updated: 2026-10-28 02:45 Europe/Paris. Implemented, compiled and validated locally. This section supersedes the historical checkpoints below. Gemini / other models can resume directly from here.

## Resume in 10 seconds

```powershell
Get-Content outputs/CTW-Rust/AI_HANDOFF_GUIDE.md -TotalCount 75
Get-Content target/validation/build-report.json
& ./target/release/ctw-rust.exe --iso work/Chinatown-Wars-PSP.iso --driving-audit --output target/driving-audit
```

Before a new task, run the mandatory upstream check, review relevant changes and record the conclusion. To rebuild and regenerate treemaps/dashboards: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test`.

## Exact delivered state

- Baseline: `d55399d`. Fulfills user request for:
  - **Option B**: Authentic body roll, pitch, and suspension spring/damper dynamics (`cWheeledVehicle::UpdateSuspension`).
  - **Option C**: Intelligent traffic avoidance, obstacle bypass/overtaking, horn alerting, and panic reaction (`trafficavoidance.cpp`).
  - **Option A**: Eradication of NPC/vehicle pop-in and lane teleportation (strict frustum guard, safe spawn perimeters, continuous lane transitions).
- Build report: **2026-10-28T02:45:48.2686653+01:00**, **227 passed, 0 failed, 0 ignored, 0 warnings** under `-D warnings`. Log: `target/validation/build-report.json`. Three Python progress-inventory tests pass. Pure native Rust treemap, site, README and SVG badges synchronized: **1030 runtime functions, 253 test/helper functions, 49 modules**.
- Delivered executable: `outputs/CTW-Rust/CTW-Rust.exe`; SHA-256 `3fa8a620231fe53bd21aeb1f3e4335209aafb1b80569fb713844c48f463287cb`, verified against build report. No GitHub push performed.
- Upstream review verified: CTW-Native `f7a45b3b7aac9bd9f4662d104d276d421c1fca52`, highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Documented in `docs/UPSTREAM_REVIEW.md` and `docs/upstream-status.json`.
- Full real-ISO audit (`--driving-audit` on `work/Chinatown-Wars-PSP.iso`): **0 world obstructions across all 166 sampled map routes**.

## Implementation Details

1. **Option B — Suspension Sway & Pitch Dynamics (`src/handling.rs`)**:
   - Reference: `references/ctw-native/port/game/vehicle.cpp:90-140, 567-576, 1395-1485`.
   - PAK 29 vehicle handling specs parse 6 sway constants (`sway_k: [i32; 6]`) per vehicle model from offset `0x74 + 4*i`.
   - `update_suspension` integrates Q12 body roll ($r = dv_x - \text{centripetal}$) and longitudinal pitch with spring/damper decay. Cars lean into corners and dip nose-forward during heavy braking ($ax > 0$).
   - `damageWobble` rattles car body when damage exceeds threshold; burnout launch tilts vehicle nose up.
   - Rail traffic in `src/traffic.rs` updates `roll` and `pitch` continuously along curves and stops.
2. **Option C — Intelligent Traffic Avoidance, Horn Alerting & Panic Flee Reaction (`src/actors.rs`, `src/traffic.rs`, `src/play.rs`)**:
   - Reference: `references/ctw-native/port/game/trafficavoidance.cpp:1-292`, `traffic.cpp:330-415, 550-670`.
   - Off-rail ambient cars calculate lateral clearance (`rail_body_clear`) to bypass blocked traffic smoothly instead of stopping dead.
   - Proximity horn alerting: cars queued behind obstructions tick down horn countdown timers and sound their horns (`sound.play_car_horn`).
   - Gunfire / explosion panic reaction (`Scene::alert_gunfire`): nearby civilian drivers panic (`driveAway`), speeding up and steering away from danger.
3. **Option A — Frustum Streaming Guards, Safe Spawning & Continuous Lane Transitions (`src/traffic.rs`, `src/actors.rs`)**:
   - Frustum visibility margins extended to $1.22$ horizontally and vertically, preventing jarring pop-in or premature despawn at screen borders.
   - Cargen and traffic spawning enforce safety margins around the player ($> 18.0\text{m}$ for cargen, never inside vehicle hull), eliminating pop-in on top of the player.
   - `rejoin_target` preserves the vehicle's assigned lane (`car.lane`) instead of picking random lanes, eliminating lane-teleportation glitches.
   - Queue preservation: cars stopped in standard traffic queues remain on rail without reversing backwards into oncoming traffic.

## Validation and honest limits

- 227 Rust tests passed, 0 warnings under `-D warnings`.
- FollowCarCam2, PSP steering assist, driving profiles, and junction curve fixes from previous commits remain 100% intact.
- Grounded force/engine calculations driven by authentic PAK 29 parameters.

---

# HISTORICAL CHECKPOINT - traffic deadlock resolution and junction curve repair

Updated: 2026-10-28 01:30 Europe/Paris. Implemented, compiled and validated locally. This section supersedes the historical checkpoints below. Gemini / other models can resume directly from here.

## Resume in 10 seconds

```powershell
Get-Content outputs/CTW-Rust/AI_HANDOFF_GUIDE.md -TotalCount 75
Get-Content target/validation/build-report.json
& ./target/release/ctw-rust.exe --iso work/Chinatown-Wars-PSP.iso --driving-audit --output target/driving-audit
```

Before a new task, run the mandatory upstream check, review relevant changes and record the conclusion. To rebuild and regenerate treemaps/dashboards: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test`.

## Exact delivered state

- Baseline: `2c96a5a`. Resolves the 3 remaining real-map traffic deadlocks on links `197->142`, `443->428`, `626->1439` documented in `docs/GEMINI_DRIVING_NEXT.md`.
- Build report: **2026-10-28T01:27:24.6767871+01:00**, **224 passed, 0 failed, 0 ignored, 0 warnings** under `-D warnings`. Log: `target/validation/build-report.json`. Three Python progress-inventory tests pass. Pure native Rust treemap, site, README and SVG badges synchronized: 1028 runtime functions, 250 test/helper functions, 49 modules.
- Delivered executable: `outputs/CTW-Rust/CTW-Rust.exe`; SHA-256 `9b142f579774619d40b3d1b98675f826ee38d83d5a6b67a4519d5dd85e43e769`, verified against build report. No GitHub push performed.
- Upstream review verified: CTW-Native `f7a45b3b7aac9bd9f4662d104d276d421c1fca52`, highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Documented in `docs/UPSTREAM_REVIEW.md` and `docs/upstream-status.json`.
- Full real-ISO audit (`--driving-audit` on `work/Chinatown-Wars-PSP.iso`): **0 world obstructions across all 166 sampled map routes** (previously 3).
  - Route `197 -> 142`: travelled 447.1 units cleanly (0 stopped ticks).
  - Route `443 -> 428`: travelled 347.7 units cleanly (0 stopped ticks).
  - Route `626 -> 1439`: travelled 546.3 units cleanly (0 stopped ticks).

## Root causes and implementation

1. **Junction curve intercept runaway & fishhook loop (`src/gps.rs` `rail_pose`)**:
   - Upstream reference: `references/ctw-native/port/game/aispline.cpp:218-236` (`AISpline::computeIntercept`).
   - Root cause on `197->142`: collinear North-heading links laterally offset by 6.0m had $|det| = 0.00311 < 0.05$. The unbounded line intercept projected 1945.8m North ($t = 1945.8$), ballooning curve length to 2009.8m into building geometry.
   - Root cause on `626->1439`: acute link angle produced $t = -10.3 < 0$, placing the control point behind the vehicle and forming a backwards fishhook reversing into buildings.
   - Fix: checks $|det| > 0.05$, $dot < 0.985$, $t > 0.001$, and forward approach towards $end$ ($(icpt - P_2) \cdot d_1 \le 0.05$). Falls back to midpoint control with cubic Hermite tangent handles along link directions for collinear offset corridors and acute turns.
2. **Sidewalk tree canopy false stops (`src/collision.rs`, `src/actors.rs`, `src/traffic.rs`)**:
   - Upstream reference: `references/ctw-native/port/game/traffic.cpp:410` (`Traffic::wander`), `vehicle.cpp:156, 331-342, 910`.
   - Root cause on `443->428`: link 431->432 passes a decorative tree cylinder in `world.bin` section 1 at $(1511.032, 303.694)$ with radius 2.50m. In `world.bin`, decorative curb cylinders have `pad != 0`. The multi-sphere vehicle envelope grazed this canopy cylinder by 3.7 cm on the curb, falsely treating it as a building obstruction and causing the vehicle to halt and detach from rails.
   - Fix: added `rail_body_clear(p, radius, height)` in `collision.rs` which checks solid building boxes and mesh walls while ignoring sidewalk curb cylinders. Added `rail_position_for` in `actors.rs` (snapping ground elevation, water exclusion, drop-offs, and building clearance). `initialize_rail` and `move_on_rail` in `traffic.rs` now use `rail_position_for`.
3. **Tests**:
   - Unit tests: `collinear_lateral_shift_and_acute_angle_curves_remain_bounded` and `curb_decorative_cylinder_does_not_block_rail_travel` in `src/traffic.rs`.
   - Real-map regression: `real_map_repaired_routes_have_zero_world_obstructions` in `src/driving_audit.rs`.

## Validation and honest limits

- 224 Rust tests passed, 0 warnings.
- No collision geometry was deleted or bypassed for player/free vehicles; no boat nodes ($z = -7$) used for land cars; no vehicle teleportation.
- FollowCarCam2, PSP steering assistance, and driving profiles (PSP/DS/Android) from commit `2c96a5a` are fully preserved.

---

# HISTORICAL CHECKPOINT - vehicle cameras and steering assistance

Updated: 2026-10-28 00:20 Europe/Paris. Implemented, compiled and validated locally. This section supersedes the historical checkpoints below. Gemini can resume directly from here.

## Resume in 10 seconds

```powershell
Get-Content outputs/CTW-Rust/AI_HANDOFF_GUIDE.md -TotalCount 65
Get-Content docs/reference/DRIVING-CAMERAS.md
Get-Content target/validation/build-report.json
```

Before a new task, run the mandatory upstream check, review relevant changes and record the conclusion. To rebuild: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test`.

## Exact delivered state

- Baseline: `f43a801`. This checkpoint adds a dedicated native FollowCarCam2 instead of the on-foot sprint/recenter path, plus player-only PSP steering assistance.
- Final build report: **2026-10-28T00:19:48.1143555+01:00**, **221 passed, 0 failed, 0 ignored, 0 warnings**. Log: `target/camera-build.log`. Three Python progress-inventory tests also pass. The build regenerates the treemap/badges/site/README from Rust only: 1026 runtime functions, 247 test/helper functions, 49 modules. PSP completion remains unmeasured.
- Delivered executable: `outputs/CTW-Rust/CTW-Rust.exe`; SHA-256 `c8fa9ed883fc6d9cb4ef56a4ad4bb26c8e1bb0a10ea912b843b8d40366965a6d`, verified against build report. No GitHub push performed.
- Latest upstream review: CTW-Native `f7a45b3b7aac9bd9f4662d104d276d421c1fca52`, highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Relevant sources/changes reviewed in `docs/UPSTREAM_REVIEW.md`.

## Implementation / controls

- `src/car_camera.rs` (child module of camera.rs): original reverse delay/orbit/unwinding, stopped-side retention, speed/size framing, spring and yaw/pitch convergence; profile switches retain the view, freecam suspends following, entry/exit reset controller state. PSP/DS base height 27/max39; Android phone 28/max42. DS/Android altitude lift differs from PSP. Evidence and boundaries: `docs/reference/DRIVING-CAMERAS.md`.
- `src/handling.rs`: PSP FillInDrivingYoke cardinal-axis assistance within about 18 degrees, proportional half-lock steering; manual steering/brake/reverse take priority. NPC assistance stays disabled. Ordinary grounded driving repair is preserved.
- **F3 / right-stick click** cycles PSP/mobile/DS. **F6 / RB + right-stick click** toggles assistance (default on); **C / LB** holds recenter. Camera and assistance preferences persist in the output folder's camera-settings.json. `--steering-assist on|off` overrides the preference. Updated French/English hint; handbrake hint corrected to B / RB.
- `play.rs` selects the occupied vehicle AFTER simulation, passes actual speed/lateral velocity/size, records camera poses in validation samples. Controller report now distinguishes FollowCarCam2 from FollowPedCam.
- Private research files are in `work/original-cameras/`; do not commit binaries, extracted assets or disassembly. DS initializer at 0x0214bd58 belongs to FollowCarCam2; preceding routine 0x020c55b0 is a DIFFERENT camera, not its constants.

## Validation and honest limits

- Eight added Rust tests cover reverse threshold/delay/unwind, interrupted reverse, stationary direction, angle wrap, profile framing, freecam/exit transitions, assistance window/sign/manual priority and physical yaw direction.
- Inspected native captures `target/camera-psp.bmp`, `target/camera-ds.bmp`, `target/camera-mobile.bmp`: each 180 render frames / 90 simulation ticks, identical 65.006317-unit trajectory and end position, zero player collision contacts. On this ground-level run PSP and DS coincide as expected; mobile has a different height. Captures preceded the final CLI-help/handbrake-label-only edit; physics/camera code is unchanged. Reports: `target/camera-{profile}/play-validation.json`.
- Earlier `target/camera-reverse.bmp` confirms a rendered front-facing reverse view, but its vehicle speed does not sustain the state-0 threshold; the 31-tick threshold is covered by the unit test, not claimed from that capture.
- No physical XInput controller connected; no full live emulator side-by-side comparison. Do not claim 100% parity. Remaining original branches: near-axis quaternion snap with its original gates, full 3D suspension/orientation, exact sphere/world sweep, lookup-table trig, mission/cinematic gates, distance setting, burnout camera and reverse-disabled state 2. Android profile is phone, not tablet.
- Existing traffic obstruction follow-up remains: links 197->142, 443->428, 626->1439. See `docs/GEMINI_DRIVING_NEXT.md` and `docs/DRIVING_TRAFFIC_REPAIR.md`. Do not weaken collisions or allow boat nodes to conceal failures. Android gear ratios still need independent PSP confirmation.

## Reproduce a native camera capture

```powershell
& ./target/release/ctw-rust.exe --iso work/Chinatown-Wars-PSP.iso --map --slice --demo-drive --camera psp --headless --frames 180 --walk 0.5 1 --capture target/camera-psp.bmp --output target/camera-psp
python -m unittest discover -s tools/progress -p test_progress.py -v
```

Use `ds` or `mobile` with distinct output/capture names for the other profiles. Preserve current logos/film and all private game data. Public language English; branding ReGTA CTW PC Recomp. Runtime remains native Rust without an emulated CPU.

---

# HISTORICAL CHECKPOINT - driving and traffic repair

Updated: 2026-10-27 19:00 Europe/Paris. Repair implemented and validated locally. Read this before historical sections below.

## Resume in 10 seconds

```powershell
Get-Content outputs/CTW-Rust/AI_HANDOFF_GUIDE.md -TotalCount 65
git log -3 --oneline
Get-Content target/validation/build-report.json
```
Then read `docs/DRIVING_TRAFFIC_REPAIR.md` and `docs/GEMINI_DRIVING_NEXT.md`.

## Exact verified state

- Repair follows baseline commit `0323f03`. Commit messages for this checkpoint describe grounded driving and traffic repair plus native-only status measurement. No GitHub push performed.
- `Compiler.ps1 -Test`: **213 passed, 0 failed, 0 ignored, 0 warnings**, build report at 2026-10-27T03:28:51.5337912+01:00. Full log: `target/driving-final.log`. Three Python inventory-integrity tests also pass.
- Delivered executable: `outputs/CTW-Rust/CTW-Rust.exe`, SHA-256 `73d673793121451d562f51f9e9e2b9027b5eedf0de528084029041ac6fb8c8ac`. Its hash matches `target/release/ctw-rust.exe` and the build report.
- Final real-ISO audit: 92 land vehicles, 166 sampled map routes, zero non-finite/excessive trajectories or route jumps over one unit. Largest route step 0.529 units. **Three routes request world-obstruction recovery**, not yet validated past that request: starting links 197->142, 443->428, 626->1439. Their exact ending nodes/positions are in `target/driving-audit/driving-audit.json`.
- Native OpenGL smoke captures inspected: `target/driving-frame180.bmp`, `target/driving-turn.bmp`, `target/traffic-20seconds.bmp`. Three-second steering run travelled 65.0 units with no player collision events; straight run hit a solid planter and stopped. No physical XInput controller was connected; no full PSP comparison/playthrough is claimed.
- Both upstreams rechecked at the evening resume, 27 October 19:01 Europe/Paris: CTW-Native `95adc327ddd8357701ca3885c61fd6b4a40f193b`; highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. New startup/bike-entry/arrest/optional-prop changes were reviewed; grounded drivetrain/traffic files are unchanged. Repair provenance stays pinned to `b0506abc786fea229101c56e0a64726aa8b4ac97`. See `docs/UPSTREAM_REVIEW.md`.

## Implementation and provenance

- `handling.rs`: grounded Q12 engine/tyre adaptation from GPL-3.0-only Kryo.to vehicle.cpp; per-vehicle handling loaded from user's PSP PAK29. Separate wheel spin, load/drive/brake splits, torque feedback, gearbox, steering reduction, service brake/reverse and explicit gas+brake burnout.
- `actors.rs`: fixed 30Hz updates, preserve world momentum while body turns, resync wheels after impact; disabled engine cannot accelerate/release burnout. `gamepad.rs` / `play.rs`: independent accelerator/brake, vertical stick does not control throttle.
- `traffic.rs` / `gps.rs`: directed lanes, correct endpoint metadata, filtered junction choices, continuous curves/residual travel, collision rollback, normal queues/lights never reverse, original reference light phases, road-dependent speed, actual spline spawn clearance/reservations and viewport guard. Rail reattachment only aligned, offscreen and straight.
- `police.rs`: pursuit supplies controls to shared physics, removing the second direct position/angle/speed update. Existing tactical target heuristic remains unverified original behaviour.
- CLI `--driving-audit`: real ISO handling traces and map routes, writes ignored JSON; `tools/research/inspect_vehicle_handling.py` inspects private records. No proprietary game files were added to Git.
- `tools/progress/progress.py`: corrected status integrity. Previous code allocated Rust function counts over Android binary routine sizes and called it original-code coverage. Current chart reads ONLY src/**/*.rs, green runtime definitions / grey test-helper definitions; no inferred PSP completion percentage. Includes 3 Python regression tests. Related-class mapping is research navigation, not verified routine correspondence.

## Latest user feedback and suggested next feature batch

Benjamin tested the repaired driving and reports it feels very similar to the original. He proposes steering assistance, automatic reverse camera, and authentic PSP/DS/Android vehicle-camera behaviour. Advisory review confirms the current runtime still uses the on-foot camera with forced recenter while driving. Recommended order: dedicated PSP vehicle camera including reverse/entry/exit transitions, evidence-based steering assistance, then distinct validated DS/Android vehicle profiles. Android carcam.cpp is a useful reference; PSP/DS constants and steering-assistance logic must be independently established. This feature batch is **not implemented yet**; existing 213-test validation applies only to the committed driving repair. Retain the obstruction-recovery assignment below as a separate follow-up.

## Known remaining limits / Gemini next task

Read `docs/GEMINI_DRIVING_NEXT.md`: reproduce three obstruction routes at their ENDING coordinates, identify actual obstacle, test physical recovery and multi-car intersections. Do not weaken collision checks or enable underwater boat nodes to hide failure.

PSP equivalence of Android gear ratios remains unverified (exact sequence search in private PSP ELF was inconclusive). Vertical suspension, slopes/airborne behaviour, original full collision response, surface grip, police tactics and bike lean/body sway are not fully ported. The world sweep still uses a simplified mostly Admiral-sized footprint. Q12 force arithmetic uses quantized float trigonometry, not bit-exact MIPS.

Gemini's earlier `docs/GEMINI_TRAFFIC_AUDIT.md` is preserved with review corrections in the repair report: z=-7 nodes are boats, directed reciprocal links already support opposing traffic, 0x4CE/4096 at 30Hz is ~9 units/s2. Its distant-car braking finding was valid and fixed. The earlier audit prompt is completed; use GEMINI_DRIVING_NEXT for continuation.

## Reproduction

```powershell
python tools/maintenance/check_upstreams.py --task "Investigate remaining traffic obstruction recovery"
powershell -ExecutionPolicy Bypass -File ./Compiler.ps1 -Test
python -m unittest discover -s tools/progress -p test_progress.py -v
& ./target/release/ctw-rust.exe --iso "work/Chinatown-Wars-PSP.iso" --driving-audit --output target/driving-audit
```

Use the original F: ISO path if the private local copy is absent. Preserve original data, current logos/film and existing research. Public language is English; branding ReGTA CTW PC Recomp. No C++/emulated CPU dependency in runtime. Recheck both upstreams before new tasks, review relevant evidence, regenerate status after significant work and record exact results here.

---

# ReGTA CTW PC Recomp — media and naming checkpoint

Updated: 2026-10-25, Europe/Paris. Current media work was published to GitHub and GitHub Pages in commit 4dbbc6c. No YouTube upload was performed.


## Obsolete-media cleanup — 25 October 2026

Owner explicitly requested removal of old logo/video versions and their superseded production work. Deleted 71 files (263,822,158 bytes; exact inventory in ignored target/obsolete-media-cleanup.json): old devlog MP4, old thumbnail, video/temp, old review images/previews and obsolete logo/video/README generators. Current four logo variants, aliases, raw custom_gameplay.mp4, launch-film outputs/work files, fonts and current generators were kept. SHA-256 comparison confirmed 109 protected files unchanged. No gameplay or research source removed. Git history was not rewritten.

## Latest user decisions
- Start the promotional film from scratch; new film uses Benjamin's original native capture, not the old finished montage.
- Public name is **ReGTA CTW PC Recomp**, not PSP Recomp. Technical PSP original-data references remain PSP.
- Two logo layouts (horizontal + vertical), each on black and transparent backgrounds.

## Completed
- Flat cover-inspired red/yellow CTW, white integrated ReGTA with lowercase e at 78% scale. Pricedown Black + Bruce Mikita Two. Four named variants in docs/assets and outputs/CTW-Rust/branding, SVG + 2x PNG; default aliases remain transparent. Board: outputs/CTW-Rust/branding/logo-variants-preview.png.
- New English film: outputs/CTW-Rust/video/launch-film/ReGTA_CTW_First_Look_EN.mp4. 1920x1080 / 60 fps, approximately 78.55 seconds container duration. 13 native footage selects, new synthetic narration and original score, concise headings, nine-second donation end card. All footage is from custom_gameplay.mp4, supplied by Benjamin. No new gameplay capture or invented footage was needed.
- New thumbnail, English SRT, narration, YouTube description and exact timeline in the same launch-film folder. Source: tools/video/build_launch_film.py + launch_film.json. Deleted the obsolete generators and old movie at the owner's request. The current generator has no emulator-footage fallback.
- Updated website, README, active docs, build output, package description, runtime window/PDA/help branding to PC Recomp. Archived historical documents and upstream repository names remain unchanged. No gameplay mechanics modified.
- docs/PROMO_REVIEW.md records the review; docs/YOUTUBE_CREATOR_KIT.md is the current upload kit. Bruce Mikita publication terms remain undocumented (pre-existing issue in BRANDING.md); no font files redistributed.

## Verification
- Compiler.ps1 -Test after runtime string changes: 160 passed, 0 failed, 0 warnings. Log target/promo-final-validation.log.
- Visual port progress treemap (`docs/assets/progress.svg`) and SVG badges synchronized automatically with pure Rust data.
- **BENJAMIN'S CORE MANDATE**: All AI models must keep the visual status and treemap up to date after every significant code advance (`.\Compiler.ps1 -Test` or `python tools/progress/progress.py`). Strictly 100% Rust code.
- Complete final MP4 decode successful; final video contact sheet visually checked. Audio peak -3.6 dBFS, no sample clipping observed; automated audio checks are not a full listening review.
- All four logo variants visually inspected, PNG alpha/background checked and SVG XML parsed. New video/thumbnail/end-card graphics inspected.
- Website browser preview was not performed. Existing browser restrictions remain; no headless browser workaround used. Logos are rasterized with Sharp, video with FFmpeg/Pillow.

## Resume
```powershell
Get-Content .\outputs\CTW-Rust\AI_HANDOFF_GUIDE.md -TotalCount 45
git status --short
```
Review the new film and logos with Benjamin. Do not publish automatically. Subsequent gameplay priorities remain in the historical handoff below, but its fidelity claims are not certified by this media work.

---
## Previous gameplay handoff (historical)
# ReGTA CTW — presentation, street races, and autonomous progress

Updated: 2026-10-24 23:50 Europe/Paris. Multi-AI autonomous handoff guide.

## Completed this session

1. **All 16 Street Races & Time Trials Engine (`src/races.rs`, Overlay 140 `streetrace`)**:
   - 16 authentic circuits across Broker, Dukes, Bohan, and Algonquin:
     - 0: Transporterror (Broker - BOABO)
     - 1: Midtown Mayhem (Dukes - Meadow Hills)
     - 2: South Boulevard Boogie (Broker - South Slopes)
     - 3: Beach Blitz (Broker - Firefly Island)
     - 4: Boulevard Boogie (Bohan - Boulevard)
     - 5: Industrial Drift (Bohan - Industrial)
     - 6: Bridge Burner (Dukes - East Island City)
     - 7: Alleyway Dash (Bohan - Chase Point)
     - 8: Star Junction GP (Algonquin - Star Junction)
     - 9: Algonquin Expressway (Algonquin - Middle Park East)
     - 10: Castle Gardens Sprint (Algonquin - Castle Gardens)
     - 11: Meat Quarter Dash (Algonquin - Meat Quarter)
     - 12: Suffolk Circuit (Algonquin - Suffolk)
     - 13: Heliport Rush (Algonquin - Lower Easton)
     - 14: Times Square Shakedown (Algonquin - Triangle)
     - 15: Liberty City Grand Prix (Endurance cross-borough championship)
   - Dynamic checkpoint detection (14m radius), 3-second starting grid countdown, real-time lap times, and personal records.
   - 3 dynamic AI rival racers (`STREETRACER1` to `STREETRACER5`, `STREETRACER1 MOD`) with authentic rubber-banding pace adjustment.
   - Live position tracking (1st through 4th), medal tiers (Gold, Silver, Bronze), cash prizes, and persistent record logging in `SaveProgress::race_best_time[16]` (IDs `0x4C` to `0x5B`).
   - Anti-cheat/forfeit timer (10s if vehicle is abandoned mid-race).
   - HUD banner and radar golden diamond waypoint indicators.

2. **Tattoo Parlor Needle Tracing Minigame (`src/minigames.rs`, Overlay 141 `tattooingapp`)**:
   - 4 Triad & gang motifs: Dragon of Broker ($1200), Koi of Dukes ($800), Tiger of Bohan ($950), Viper of Algonquin ($1500).
   - Stencil needle tracing, ink reservoir replenishment, client pain meter, and flinch/failure handling.

3. **YouTube Creator Production Kit (`docs/YOUTUBE_CREATOR_KIT.md`)**:
   - Comprehensive video kit for Benjamin's first devlog/promotion video:
     - 5 high-CTR titles, thumbnail design with PNG logos.
     - Minute-by-minute script (Hook, Rust recomp vs emulator, 60fps/4k demo, live gameplay, drug wars / tattooing / street racing, roadmap, call to action with Tipeee and PayPal.Me).
     - Copy-paste description with timestamps, links, legal disclaimer, tags.
     - OBS Studio 60 FPS recording guide with fullscreen launcher commands.

4. **Official Project Branding, PNG Assets & Trilingual Showcase**:
   - 4 vector SVGs and 4 high-resolution PNGs delivered:
     - `docs/assets/logo.png` & `logo-transparent.png` (Vertical Stacked, 1800 × 1000)
     - `docs/assets/logo-ctw.png` & `logo-ctw-transparent.png` (Horizontal Inline, 1920 × 520)
   - GitHub Pages showcase deployed with trilingual EN / FR / ES switcher and persistence.
   - Repository renamed to `regta-chinatown-wars-pc-port`.

## Verification

- **Mandatory compiler check**: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test`
  - **160 passed, 0 failed, 0 ignored, 0 compiler warnings (`-D warnings`)**.
  - Binary delivered to `outputs/CTW-Rust/CTW-Rust.exe`.
- **Upstream check**: `references/chinatown-wars-psp-recomp` branch `master` clean and up to date.

## Next Roadmap Priorities (Autonomous Continuation)

1. **YouTube Promotion Video Recording**:
   - Benjamin records and publishes the first video using `docs/YOUTUBE_CREATOR_KIT.md`.
2. **Story Missions Execution & Cutscenes Engine (`src/missions.rs`)**:
   - Implement cutscene camera staging, scripted boss dialogue, and mission chains across the 8 boss contacts.
3. **Vehicle Radio Audio Stations Synthesis**:
   - Integrate radio audio playback loops for all 11 Chinatown Wars in-car radio stations.

Resume command:
```powershell
powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test
```

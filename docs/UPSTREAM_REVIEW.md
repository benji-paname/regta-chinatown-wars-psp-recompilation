# Upstream Repositories Review — 25 October 2026

## 2026-11-01 — Authorized isolated Bink fix validation

Preflight task: `Isolated Bink mutex owner-exit fix validation with deterministic tests and at most three bounded boots`. Exit 1; GitHub DNS fails for all configured mirrors. No current remote revision check claimed. Local highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7` (GPL-3.0-or-later): re-reviewed kernel_exit_current_thread/terminate, mutex ownership, best_waiter/wake, process_timers and fiber exit paths. The approved cleanup addresses owner-exit lifetime but retains priority/UID wake ordering. Local Android `b0506abc786fea229101c56e0a64726aa8b4ac97` (GPL-3.0-only): loading-screen and recent Porter/death-app changes provide no PSP normal-mutex implementation to substitute. No upstream source changed or imported automatically. The prior PPSSPP source comparison remains reference evidence, not a PSP hardware test. Rust progress unchanged by this C++ experiment. Separately, PPSSPP primary mutex source was retrieved through the web reader again; this does not repair the Git DNS preflight. Final isolated validation:27 checks pass,3 known queue-policy discrepancies,3/3 bounded boots reach autosave/first-mission intro, including one observed owner-exit handoff. No upstream integration performed.

## 2026-11-01 — Bink mutex root-cause investigation

Mandatory `check_upstreams.py --task "Prove European Bink mutex lifecycle and prepare an isolated faithful fix proposal"` exited 1: all configured Git remotes failed DNS resolution for github.com. No current remote revision verification claimed. Local highcmd pin `6562163b6f504a1db93e1be5ce580d8cd6f024a7` (GPL-3.0-or-later) was reviewed, particularly kernel_thread.cpp mutex ownership, waits and exit; it does not release owned mutexes in kernel_exit_current_thread. Local Android pin `b0506abc786fea229101c56e0a64726aa8b4ac97` (GPL-3.0-only), recent WIP/loading/death-app/Porter changes reviewed: no applicable PSP ThreadMan mutex-lifetime implementation established, nothing imported.

Separately, the web reader could retrieve PPSSPP's primary `Core/HLE/sceKernelMutex.cpp` source on master, including `__KernelMutexThreadEnd` and ownership validation; this is technical reference evidence, not a successful Git mirror refresh or a pinned hardware test. It explicitly releases held normal mutexes at thread end. PSPAutotests pages could not be retrieved; no hardware test result is claimed. European original ELF disassembly and isolated diagnostic logging are used to confront that difference with the observed Bink chain. Rust progress visuals unchanged because this is C++ diagnostics only.

## 1. Upstream Mirrors & Roles

| Upstream Mirror | Origin / Author | Platform / Target | Current Revision | Role & Provenance Policy |
|---|---|---|---|---|
| `references/ctw-native` | [`official-kryo-to/ctw-native`](https://github.com/official-kryo-to/ctw-native.git) | Android 4.4.243 / C++ | `b0506abc` (2026-10-22) | **Game systems reference**: Class names, state machines, math formulas, dealer/commodity tables. Useful for understanding decompiled game logic, but Android-specific adaptations must be checked against PSP Europe (`ULES-01347`). |
| `references/chinatown-wars-psp-recomp` | [`highcmd/chinatown-wars-psp-recomp`](https://github.com/highcmd/chinatown-wars-psp-recomp.git) | PSP USA `ULUS-10490` v1.01 / C++ + PSP HLE | `6562163b` (2026-10-24) | **PSP executable & MIPS recompilation reference**: Overlay structure (`OVERLAYS.PAK`), MIPS register conventions, PSP memory layout, and HLE wrapper patterns. |
| **ReGTA CTW** *(Our Target)* | [`benji-paname/regta-chinatown-wars-pc-port`](https://github.com/benji-paname/regta-chinatown-wars-pc-port) | PSP Europe `ULES-01347` v1.01 / **Native Rust (Windows x64)** | Local workspace | **Clean-room Native Rust PC Port**: Independent implementation with zero emulated CPU at runtime, reading original game assets from legal PSP discs. |

---

## 2. Recent Upstream Advances

### A. `ctw-native` (Android / C++)
- `b0506ab` wip
- `a31e94b` Release Porter 0.3.0
- `48f20b5` Port the death app: the shattering screen, the Wasted jingle and WASTED! text
- `9642011` Add sirens: the toggle in siren vehicles and their looping sound
- `17e51d9` Hold thrown peds' sprites until they land; add the landing thud and pain comments
- `ed73363` Get in by the nearer door: port the player's seat plans

**Analysis & Applicability to ReGTA CTW**:
- The pedestrian throwing physics and landing sound, vehicle siren sound loop, and nearest-door seat determination in `ctw-native` provide valuable structural logic for our pedestrian AI (`src/actors.rs`) and vehicle interaction (`src/vehicle.rs`).
- However, `ctw-native`'s percentage metrics reflect decompiled symbols from Android binaries. They do not translate directly to PSP Europe MIPS function offsets.

### B. `chinatown-wars-psp-recomp` (highcmd PSP USA)
- `6562163` Launcher 0.1.5 (rebuilds the game: HUD size on F3)

**Analysis & Applicability to ReGTA CTW**:
- highcmd continues work on launcher and HUD scaling for the USA release (`ULUS-10490`).
- ReGTA CTW already supports dynamic multi-resolution HUD rendering, camera profiles (PSP / Mobile / DS), and responsive scaling directly in native Rust OpenGL (`src/gpu.rs`).

---

## 3. Provenance & Coverage Policy

- **No borrowed coverage percentages**: The progress percentage seen in Android decompilation charts measures Android ELF symbol coverage, not PSP MIPS completion.
- ReGTA CTW does not display an unverified percentage until an exact build-specific inventory of European PSP routines (`docs/progress/psp-routines.json`) is audited.
- All progress is tracked through **verifiable, testable counts** in our codebase (unit tests, overlays, subsystems, and game features).


## 26 October 2026 — Model choice discussion

Both mirrors were fetched for this task: CTW-Native `b0506abc786fea229101c56e0a64726aa8b4ac97`; PSP recomp `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Reviewed the recent-commit snapshot against the previous review above; neither remote tip changed. These repositories do not provide a controlled Opus 5.5 versus GPT-6 Astra comparison for PSP reverse engineering or Rust recompilation. No upstream integration or gameplay change is relevant to this advisory task. Model recommendations must distinguish vendor benchmark evidence from project-specific evaluation.

## 26 October 2026 — Driving and traffic repair (active)

Both remotes were fetched successfully after a restricted-network failure; pinned revisions remain b0506abc786fea229101c56e0a64726aa8b4ac97 and 6562163b6f504a1db93e1be5ce580d8cd6f024a7. Reviewed CTW-Native vehicle.cpp (SetPropertiesFromVehicleInfo, Act, UpdateSteering, CEngine, CTyre, RecalcKinematics), aispline.cpp (metadata/lanes, randomNode, fixUpTuplet), roads.cpp and traffic.cpp (forwardSpeed, wander, afterPhysics, rejoin). GPL-3.0-only, Copyright 2026 Kryo.to. The highcmd README/NOTES confirm original 30 Hz logic, PSP USA data and a different HLE runtime; no engine integration is appropriate.

Confirmed local regressions: body-frame momentum is not reprojected on turning; artificial drivetrain and grip scales replace source calculations; reverse also triggers burnout; red-light waits trigger reverse recovery; rail progress can advance without a successful world move; filtered-link selection uses an unfiltered count. Repair will adapt grounded force/engine algorithms with the user's PSP PAK 29 parameters. Android gear constants are reference data, not yet verified Europe PSP constants (exact sequences were not found in the private ELF). GPS curve arc-length adaptation and existing collision/presentation are not certified PSP equivalence. See the active AI_HANDOFF_GUIDE and forthcoming repair report for validation.

## 27 October 2026 - Driving repair resumed and verified

The mandatory preflight was repeated successfully after restricted DNS initially failed. CTW-Native remains `b0506abc786fea229101c56e0a64726aa8b4ac97`; highcmd remains `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Re-read vehicle.cpp engine/tyre/integrator paths, traffic.cpp roadSpeed/forwardSpeed, roads.cpp lane metadata and boat classification, aispline.cpp curve/light rules, and trafficavoidance.cpp cache/clearance semantics. No remote change requires another integration. Licenses and target differences remain as recorded above.

Adaptations are now in native handling.rs/traffic.rs and the host integration. Local validation: 213 Rust tests, zero warnings; ISO diagnostics and native captures are detailed in DRIVING_TRAFFIC_REPAIR.md. Gemini's audit was considered, but its underwater-bridge and missing-reverse-lane claims contradict the inspected sources/current graph representation; corrections are recorded in that report. Original police targeting, complete collision/suspension and exact PSP gear constants remain unverified.

Status integrity correction: the inherited generator allocated real Rust function counts over Android reference routine sizes and labeled the result original-code coverage. That was not an audited PSP correspondence. The generator now reads only src/**/*.rs, separates tests/helpers, and displays a source inventory with PSP completion explicitly unmeasured. No C++ routine totals or sizes contribute to the chart.

### Evening resume, 27 October (19:01 Europe/Paris)

A fresh successful preflight found CTW-Native advanced to `95adc327ddd8357701ca3885c61fd6b4a40f193b` (WIP, 15:54 +01:00). Highcmd remains `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Reviewed the full changed-file list and diffs for STATUS.md, game.cpp, peds.cpp, police.cpp and propdynamics.cpp. Changes include startup/intro video/loading, wanted radar tint, bike mounting from the nearer side, swept contact/hold logic for on-foot arrest, and optional modded loose-prop physics. The grounded vehicle.cpp, roads.cpp, aispline.cpp, traffic.cpp, driving.cpp and trafficavoidance.cpp used by this repair are unchanged. Keep the algorithm provenance pinned to b0506abc; no automatic integration needed. Do not mistake the optional prop enhancement for verified original PSP physics. Bike entry and on-foot arrest improvements are follow-up candidates, outside this driving repair.

## 27 October 2026 - Next driving priorities (advisory review)

Preflight succeeded after the restricted-network attempt: CTW-Native 95adc327ddd8357701ca3885c61fd6b4a40f193b; highcmd 6562163b6f504a1db93e1be5ce580d8cd6f024a7. Reviewed carcam.cpp (FollowCarCam::inherit/update/setBehind), game.cpp vehicle-camera dispatch, vehicle.cpp steering references, and highcmd README platform/runtime scope. Native Rust camera.rs remains FollowPedCam-derived; play.rs feeds driving into the on-foot controller and forces recenter above speed 3. Thus a dedicated vehicle-camera path is the next concrete fidelity improvement.

Android FollowCarCam2 already has speed/vehicle-size distance and height, a spring, entry transition, stopped-view retention, and a reverse state triggered after sustained signed backward velocity; this provides a GPL-3.0-only algorithm reference, not proof of PSP or DS constants. Recommendation: establish PSP driving camera including automatic reverse, then identify original steering-assistance logic in PSP evidence, then implement/validate distinct DS and Android vehicle profiles. Steering assistance has not yet been located/verified by this review. No gameplay changes or new test run made for this advisory task.


## 28 October 2026 - Driving cameras and steering assistance implemented

Mandatory preflight completed after a restricted-network failure. CTW-Native tip: `f7a45b3b7aac9bd9f4662d104d276d421c1fca52`; highcmd remains `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Reviewed the changed-file list since 95adc327, STATUS changes and vehicle.cpp changes: new multiplayer pose plumbing does not alter the grounded steering/engine path. carcam.cpp and driving.cpp are unchanged. Re-read highcmd README scope (PSP USA/HLE); its addresses/runtime are not interchangeable with our Europe PSP data. No upstream merge or communication performed.

Kryo.to GPL-3.0-only carcam.cpp at 95adc327 provides the structural reference, with original PSP/DS/APK binaries supplying independent camera constants and the reverse orbit state machine. The immediate reverse-offset flip in the reference port is incomplete compared with all three originals. Located PSP steering assistance in FillInDrivingYoke at 0x089cb264..0x089cb720: cardinal-axis correction, not lane following. Android Act/UpdateSteering identifies the proportional rack path. Implemented native Rust car_camera.rs and player-only assistance in handling.rs; see [driving camera evidence](reference/DRIVING-CAMERAS.md) for exact addresses, provenance and remaining limits.

Final build: 221 Rust tests passed, zero warnings; three Python source-inventory tests passed. Native PSP/DS/mobile captures were inspected; the three runs retain identical vehicle trajectories. This is not a full emulator comparison or a claim of bit-exact original parity. No proprietary inputs were added to Git.


## 28 October 2026 - Traffic deadlocks resolved on routes 197->142, 443->428, 626->1439

Mandatory preflight completed via `python tools/maintenance/check_upstreams.py`: CTW-Native checked at `f7a45b3b7aac9bd9f4662d104d276d421c1fca52`, highcmd at `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Reviewed `aispline.cpp`, `traffic.cpp`, `vehicle.cpp`, and `roads.cpp`.

Root cause analysis and resolution:
1. `197->142` (ended at 142->215) and `626->1439` (ended at 1439->1467):
   - Investigated `src/gps.rs:rail_pose` line-intercept construction. On route `197->142`, nearly collinear North-bound links with lateral offset produced a tiny determinant ($|det| = 0.00311 < 0.05$), ballooning the intersection parameter $t$ to 1945.8m and the Bezier curve to 2009.8m North into building collision boxes. On route `626->1439`, an acute outgoing angle produced $t = -10.3m < 0$ (behind start), placing the control point backwards and creating a fishhook reversing loop into a building.
   - Adapted upstream `aispline.cpp:218-236` (`AISpline::computeIntercept`): intercept requires $|det| > 0.05$, $dot < 0.985$, forward progress along incoming tangent ($t > 0$), and an approach from behind the outgoing link start ($(icpt - P_2) \cdot d_1 \le 0.05$). Invalid intercepts and collinear offset links fall back to midpoint control with cubic Hermite handles along $d_0$ and $d_1$. Route 1 curve length drops from 2009.8m to ~10m (cleanly on road), and Route 3 curves forward smoothly without backwards reversal.
2. `443->428` (ended on two-way street link 431->432):
   - Traced real ISO road geometry and collision data. Link 431->432 travels North in lane 0 at $X = 1508.50$. A static decorative cylinder at $(1511.032, 303.694)$ with radius $2.50m$ sits on the sidewalk curb ($0.53m$ from curb edge), overhanging into the road corridor.
   - In upstream Chinatown Wars (`traffic.cpp:410`, `vehicle.cpp:185, 910`), on-rail traffic (`cWanderRoads`) follows road splines via `placeOnRail` without per-frame static map box/cylinder sweeps because `ai.bin` links are designed corridors. Furthermore, vehicle collision in `vehicle.cpp:156` uses centerline-aligned spheres (`sphereStep_[0] = 0`), and `vehicle.cpp:910` skips decorative prop cylinders. In contrast, our `move_on_rail` used `car_position_for`, which checked `body_clear` against decorative curb cylinders with a 6-sphere envelope that grazed the canopy by 3.7 centimeters, causing false stops on clear roads and trigger-happy off-rail recovery.
   - Introduced `rail_position_for` and `rail_body_clear`: preserves strict checks for ground elevation, water exclusion, vertical ledges/drop-offs, solid building boxes (`BoxShape`), and mesh geometry (`triangles`), but avoids false stops against decorative sidewalk canopy cylinders.

Validation:
- `Compiler.ps1 -Test`: **224 passed, 0 failed, 0 warnings**. 1028 native Rust runtime functions, 250 test/helper functions, 49 modules.
- Three Python inventory-integrity tests pass.
- `--driving-audit` on `work/Chinatown-Wars-PSP.iso`: 92 vehicles, 166 real-map routes, **zero world obstructions across all 166 routes** (previously 3). Max step: 0.529 units. Route `197->142` travelled 447.1 units, `443->428` travelled 347.7 units (0 stopped ticks), `626->1439` travelled 546.3 units. No push performed.

## 28 October 2026 - Vehicle Body Suspension (Option B), Traffic Avoidance (Option C), and Frustum / Pop-in Guards (Option A)

Preflight and upstream review verified against CTW-Native `f7a45b3b7aac9bd9f4662d104d276d421c1fca52` and highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7`.

Implementation summary:
1. **Option B — Suspension Sway & Pitch Dynamics (`src/handling.rs`)**:
   - Upstream reference: `references/ctw-native/port/game/vehicle.cpp:90-140` (`swayK_` parsed from PAK 29 record at offset 0x74 + 4*i), `1395-1485` (`cWheeledVehicle::UpdateSuspension`), and `567-576` (`damageWobble`).
   - Added `sway_k: [i32; 6]` to `handling::Parameters`.
   - Dynamic body roll (`sway_roll`, `sway_roll_v`) in Q12 ($4096 = 1.0\text{ rad}$) responding to lateral centrifugal acceleration ($r = dv_x - \text{centripetal}$) with spring/damper decay ($k_1, k_0$). Cars lean realistically into turns and evasive swerves.
   - Dynamic body pitch (`sway_pitch`, `sway_pitch_v`) diving nose-down under heavy braking ($ax > 0$) and squatting under hard launch ($ax < 0$) using parameters $k_3, k_2$.
   - Authentic damage rattle / wheel wobble kick (`damageWobble`) when vehicle health drops below threshold, and launch burnout tilt ($k_5, k_4$).
   - Synchronized on-rail traffic roll/pitch calculation in `src/traffic.rs` so ambient cars exhibit authentic body dynamics while navigating curves and junctions.
   - Unit tests: `body_suspension_sway_reacts_to_turning_acceleration_and_braking`, `rail_traffic_updates_body_roll_in_curves_and_pitch_during_braking`.

2. **Option C — Intelligent Traffic Avoidance, Horn Alerting & Panic Flee Reaction (`src/actors.rs`, `src/traffic.rs`, `src/play.rs`)**:
   - Upstream reference: `references/ctw-native/port/game/trafficavoidance.cpp:1-292` (`TrafficAvoidance::compute`), `traffic.cpp:330-415` (`wander`, horn countdown), `550-670` (`rejoin`, `driveAway`).
   - Ambient off-rail bypass steering: when encountering blocked traffic, cars assess side clearance via `rail_body_clear` and steer smoothly around obstructions rather than stopping dead or jumping tracks.
   - Proximity horn alerting: ambient vehicles queued behind stopped cars sound their horns with authentic countdown timers before attempting avoidance maneuvers (`sound.play_car_horn` hooked in `src/play.rs`).
   - Gunfire & explosion panic reaction (`Scene::alert_gunfire`): vehicles within earshot of shooting or explosions trigger panic flee reaction (`driveAway`), accelerating away with evasive steering.
   - Unit tests: `vehicle_traffic_ai_obstacle_avoidance_overtaking_and_gunfire_alert`, `vehicle_traffic_ai_recovery_horn_and_panic_reaction`.

3. **Option A — Frustum Streaming Guards, Safe Spawning & Continuous Lane Transitions (`src/traffic.rs`, `src/actors.rs`)**:
   - Expanded frustum visibility margins in `traffic.rs:visible` from $1.15$ to $1.22$ horizontally and vertically, preventing vehicles and pedestrians from popping into view or despawning right at the viewport boundaries.
   - Safe spawn placement: cargen parking vehicles and ambient traffic enforce safety clearance perimeters around the player ($> 18.0\text{m}$ for cargen, never inside vehicle hull) preventing cars from spawning on top of the player or in direct line of sight.
   - Continuous lane transitions: `rejoin_target` strictly preserves the vehicle's assigned lane (`car.lane`) instead of randomly selecting arbitrary lanes on the link, completely eliminating unnatural mid-route lane teleportation.
   - Queue preservation: cars in normal queues (`gap == 0.0`) remain strictly on rail without false un-railing or backward reversing into oncoming traffic.

Validation:
- `Compiler.ps1 -Test`: **227 passed, 0 failed, 0 warnings** under `-D warnings`.
- Treemap, site, README and SVG badges synchronized: **1030 runtime functions, 253 test/helper functions, 49 modules**.
- Three Python progress-inventory tests pass cleanly.
- ISO driving audit on `work/Chinatown-Wars-PSP.iso`: **0 failures across all 166 real-map routes**.
- Delivered to `outputs/CTW-Rust/CTW-Rust.exe`.

## 28 October 2026 - City Street Furniture Props, Physical Impacts, 3D Loose Tumbling, and Loot Drops

Preflight and upstream review verified against CTW-Native `f7a45b3b7aac9bd9f4662d104d276d421c1fca52` and highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7`.

Implementation summary:
1. **Citywide Props Synchronization & Rendering (`src/props.rs`, `src/gpu.rs`)**:
   - Upstream reference: `references/ctw-native/port/src/world/props.h`, `props.cpp`, `cDynamicPropManager`, `Collision::setPropSolid`.
   - Removed the filter skipping indestructible/static street props (`smash_force < 0 && uproot_force < 0`) in `PropsManager::sync_area`. All street furniture across Liberty City (solid lampposts, traffic light poles, telephone booths, trees, mailboxes, bus shelters, fences, hydrants, bins) are now synchronized, visible, and rendered.
   - Smashed/broken model support: if a prop definition specifies a broken model (`broken_model_id < 0xFFFE`), the broken mesh is rendered in place of the intact one; if `0xFFFE`, it is cleanly removed.
2. **Comprehensive Vehicle & Explosion Impacts (`src/props.rs`, `src/actors.rs`, `src/play.rs`)**:
   - Vehicle impacts checked across all active moving cars (player vehicle and ambient traffic) via `check_vehicle_impact_with_loot`.
   - Explosion blasts (`Scene::step_explosions`) blow up, tilt, and fling nearby street furniture within blast radii.
   - Solid push-out for pedestrians (`push_out_player`): player on foot can no longer ghost through solid street furniture.
3. **Authentic Physics Reactions & Particle Bursts**:
   - Hydrants (effect 9): tilts by 24 degrees in impact direction, disables collision, launches continuous vertical water spray geyser (`kind: 1`), and triggers looping hydrant audio.
   - Trash cans & dumpsters (effect 35): becomes a `Loose` physical body with 3D ballistic translation ($\mathbf{p} += \mathbf{v} \cdot dt$), gravity ($-9.8\text{ m/s}^2$), angular tumbling rotation, ground restitution bounce ($0.38$), rolling friction ($0.75$), settling to sleep, and emitting garbage debris (`kind: 2`).
   - Wooden fences & barriers (effect 16, 26, 27): shatters into flying wood splinters (`kind: 3`) with wood smash audio.
   - Newspaper stands (effect 14, 22, 42): breaks, releases fluttering paper pages (`kind: 4`), and drops \$25 cash pickup loot on the pavement.
   - Parking meters: snaps, drops \$40 cash pickup loot with authentic chime audio.
   - Streetlights & lampposts (effect 7, 8): tilts, emits electrical arc sparks (`kind: 5`), and topples.
   - Spatial sound dispatch: `drain_pending_smashes` hooked into `sound.play_prop_smash` in `src/play.rs`.

Validation:
- `Compiler.ps1 -Test`: **229 passed, 0 failed, 0 warnings** under `-D warnings`.
- Treemap, site, README and SVG badges synchronized: **1036 runtime functions, 255 test/helper functions, 49 modules**.
- Three Python progress-inventory tests pass cleanly.
- ISO driving audit on `work/Chinatown-Wars-PSP.iso`: **0 failures across all 166 real-map routes**.
- Delivered to `outputs/CTW-Rust/CTW-Rust.exe`.

## 28 October 2026 - City Street Furniture Models & Textures OpenGL Pipeline Integration

Preflight and upstream review verified against CTW-Native `f7a45b3b7aac9bd9f4662d104d276d421c1fca52` and highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7`.

Defect resolved:
- In `CTW-Rust.exe`, street furniture and props were synchronized in memory but invisible on screen because `Renderer::upload` only uploaded map meshes and car models, leaving `self.models.get(&model_to_draw)` returning `None` for every prop.
- Furthermore, prop 3D meshes (368 models from PAK resource 1866) and textures (266 static, 5 animated from PAK resources 1869..2163) were not previously parsed from `GAME.PAK`.

Fix implementation:
1. **Asset Extraction in `PropLibrary::load` (`src/props.rs`)**:
   - Parses all 368 intact and broken prop 3D meshes referenced by prop definitions using `model::parse`.
   - Decodes all 271 unique prop textures and animated texture frame sets using `texture::decode` and `texture::animation`.
2. **OpenGL Display Lists & Texture Upload in `Renderer::upload` (`src/gpu.rs`)**:
   - Added dedicated `prop_models` and `prop_textures` maps to `Renderer`.
   - Uploads all prop textures to GPU memory with mipmapping, and builds display list triangle batches for each prop model with surface vertex normals and illumination shading.
   - Preserves memory safety with clean teardown in `clear_assets`.
3. **Display Rendering & Animation in `Renderer::draw` (`src/gpu.rs`)**:
   - Queries `self.prop_models` (with fallback to `self.models`) for `model_to_draw`.
   - Binds prop textures and animates flashing/neon prop lights using `prop_library.animations`.
   - Extended active streaming area in `sync_area` to $5 \times 5$ cells ($250\text{ m} \times 250\text{ m}$) with 130m retention distance, preventing pop-ins.
4. **Immediate Initial Synchronization (`src/play.rs`)**:
   - Calls `s.props.sync_area` immediately upon scene initialization so all furniture around the spawn is present from frame 0.
   - Added `active_props`, `loaded_prop_models`, and `loaded_prop_textures` telemetry to slice validation.

Validation:
- `Compiler.ps1 -Test`: **229 passed, 0 failed, 0 warnings** under `-D warnings`.
- Runtime verification: 81 active props in initial bubble, 368 prop models loaded, 266 prop textures uploaded to OpenGL.
- Visual frame capture `target/prop-screen.bmp` and `target/prop-drive.bmp` confirms rendered street lamps, green trash cans, food carts, trees, and street furniture.
- Delivered to `outputs/CTW-Rust/CTW-Rust.exe`.

## 28 October 2026 - Street Furniture Solid Collision On Foot & Authentic Falling Physics

Preflight and upstream review verified against CTW-Native `references/ctw-native/port/src/world/collision.cpp` (`Collision::setPropSolid`, `propShapes`), `references/ctw-native/port/src/world/props.cpp` (`PropLibrary::shapes`), and `references/ctw-native/port/game/propdynamics.cpp` (`smashEffect == 7` / `8`, `lampTimer`, toppling physics).

Defects resolved:
1. **On foot (à pied)**: Zero solid collision physics; the walking player ghosted straight through standing street furniture (lamp posts, phone booths, trees, dumpsters, fences, etc.).
2. **In vehicle / Destruction physics**: Smashed street furniture lacked authentic physical behaviors — notably, lamp posts froze in mid-air at a fixed ~30° tilt rather than toppling over and landing flat on the pavement.

Fix implementation:
1. **Solid Prop Collision On Foot (`src/collision.rs`)**:
   - Added `PropShapeRange { box_start, box_count, cyl_start, cyl_count, solid, saved_boxes, saved_cyls }` and `pub prop_shapes: Vec<PropShapeRange>` to `Cell`.
   - Implemented `Collision::populate_prop_shapes(&mut self, library: &PropLibrary)`: transforms and populates all 165 box, 74 cylinder, and 12 sphere primitives from PAK 1866 into each `Cell`'s `boxes` and `cylinders`.
   - `Walker::step` sub-stepping calls `push_out(&mut p, 0.875, 2.0)`, stopping the walking player solidly against standing streetlights, fences, phone booths, bins, and trees.
   - Marked prop boxes with `flags: 0x8000`, cleanly skipped by `rail_body_clear` and `body_clear` so ambient traffic road rails are never obstructed.
   - `car_position_for` uses `rail_body_clear`, allowing vehicles to impact breakable furniture directly into `PropsManager::check_vehicle_impact_with_loot` without premature wall stopping.
2. **Authentic Toppling & Physical Destruction (`src/props.rs`, `propdynamics.cpp:216-224, 489-502`)**:
   - Added `tilt_speed: f32` and `base_z: f32` to `ActiveProp`.
   - Added `pending_disables: Vec<(usize, usize, usize)>` to `PropsManager` and drained in `src/play.rs` to call `collision.set_prop_solid(cx, cy, prop_idx, false)`.
   - Streetlights (`smash_eff == 7`): Impact unanchors solid collision via `set_prop_solid`, emits broken lamp head electric sparks (`kind: 5`), and initiates gravitational topple (`tilt_speed = (impact_speed * 0.15 + 2.8).min(5.5)`, accelerating with `10.0 * dt`). Tilts progressively from $0^\circ$ to $88^\circ$ ($1.54$ rad), triggers ground clatter sound 56 on pavement contact, and rests flat on the ground.
   - Streetlights strictly remain anchored at their base; they never fly into the air or execute barrel rolls/tonneaux at high speed, matching original Chinatown Wars behavior.
   - Large streetlights / signals (`smash_eff == 8`): tilt flat immediately ($1.48$ rad / $85^\circ$) with electric sparks and unanchored collision.
   - Dumpsters & trash bins (`smash_eff == 35`): loose tumbling physics with ground restitution settling at `prop.base_z`.
   - When props are broken/tilted, `Collision::set_prop_solid` parks their shapes below the world (`-100000.0`), allowing pedestrians and vehicles to walk or drive over the fallen debris.

Validation:
- `Compiler.ps1 -Test`: **230 passed, 0 failed, 0 warnings** under `-D warnings`.
- Treemap, site, README and SVG badges synchronized: **1040 runtime functions, 256 test/helper functions, 49 modules**.
- Real-map driving audit on `work/Chinatown-Wars-PSP.iso`: **0 failures across all 166 real-map routes**.
- Delivered to `outputs/CTW-Rust/CTW-Rust.exe`.


## 28 October — paired PSP/Rust prop recordings

Both upstreams checked successfully after restricted-network failure: f7a45b3b7aac9bd9f4662d104d276d421c1fca52 and 6562163b6f504a1db93e1be5ce580d8cd6f024a7, unchanged. Reviewed propdynamics.cpp contactForces/applyForce/smashParticles, rigidbody.cpp, world/props.cpp and PaperEmitter, plus highcmd README target/runtime boundaries. GPL-3.0-only Kryo.to reference adaptation is documented in PROP_CONTACT_REPAIR.md; PSP PAK thresholds independently inspected. No upstream merge or communication. Full PSP dynamics are not claimed.


## 29 October — strict PSP prop fidelity
Both mirrors checked successfully after restricted-network failure; unchanged f7a45b3b7aac9bd9f4662d104d276d421c1fca52 / 6562163b6f504a1db93e1be5ce580d8cd6f024a7. Reviewed rigidbody.cpp RecalcKinematics/integrateStep/damp and highcmd README USA/HLE boundaries. Located independent Europe PSP update/damping code; its angular gates and velocity-cap conversion require more research. Added test-only signed Q12 transcription. Rejected isolated runtime damping replacement after settling regression. No automatic upstream integration. See reference/PSP-PROP-PHYSICS.md.


## 29 October — native force core with original-code differential checks
Upstream preflight succeeded after restricted attempt. Tips remain f7a45b3b7aac9bd9f4662d104d276d421c1fca52 and 6562163b6f504a1db93e1be5ce580d8cd6f024a7. Reviewed rigidbody.h/cpp force integration, inertia, FullCollision and FullSprings, plus highcmd platform boundaries. Independent Europe ELF inspection confirmed the gravity initializer and speed-cap conversion; 512 original kinematics executions match staged native Rust. No runtime CPU emulator or automatic merge. Coupled contact solver remains outstanding.


## 2026-10-29 — PSP HUD and rendering regression repair
Both mirrors fetched successfully after a restricted-network failure. Pins: Android f7a45b3b7aac9bd9f4662d104d276d421c1fca52; PSP USA 6562163b6f504a1db93e1be5ce580d8cd6f024a7. Read Android weaponhud.cpp/weapons.h and world/timecycle.cpp/worldrenderer.cpp, plus PSP runtime/ge_render.cpp/ge_gl.cpp. GPL-3.0-only references; no CPU/HLE code imported.
Android recently added weapon/cash HUD routines, but its sprite IDs and viewport differ. PSP ISO SS_Weapons.bin has 34 sprites (fists ID14), versus Android fists ID13. Use PSP directories and visually identified PSP icons, never Android indices. PSP reference executes GE state; our constant face lighting is still an approximation. Android timecycle data/formulas are research leads, not certified PSP lighting. Removed default artistic grading rather than inventing another filter. Full menus, timecycle, shadow projection and original display timing remain unfinished; see reference/PSP-VISUAL-PARITY.md.


## 2026-10-29 — PSP timecycle runtime integration
Both upstream checks succeeded, same pinned revisions f7a45b3b7aac9bd9f4662d104d276d421c1fca52 / 6562163b6f504a1db93e1be5ce580d8cd6f024a7. Re-read Android timecycle.h/.cpp and worldrenderer lighting; PSP ge_render.cpp explicitly describes emissive + ambient + sum of lights. These GPL-3.0 references inform structure, but PSP Europe analysis revealed incompatible data: 42240 bytes, 55 channels per weather, not Android 29184/38. PSP RGB initialization applies pow(channel/255,0.6)*255, then truncates, unlike Android initialization. Native interpolation follows PSP float half-Q12 fraction, not Android integer rounding. Renderer now consumes PSP ambient/diffuse data; sun-direction convention remains adapted from the sibling renderer, not certified PSP-equivalent. No HLE/emulation imported. See PSP-VISUAL-PARITY.md for original addresses and limits.


## 2026-10-29 — user-rejected visual parity; footage recheck
Both mirrors checked at unchanged pins f7a45b3b7aac9bd9f4662d104d276d421c1fca52 / 6562163b6f504a1db93e1be5ce580d8cd6f024a7. Reviewed PSP ge_render.cpp decode_vtype/rd/light_vertex: signed byte normals divide by128, lighting normalizes transformed normals. Locally inspected PSP models30,7207,17463: bytes+2..4 contain signed normal vectors; +5 is padding. Prior parser discarded these and GPU reconstructed face normals. Preserve original vectors and transform by node rotation (no translation). No upstream code copied into runtime. Rechecked full-resolution user video frames at0/18/54 seconds; original and recorded native lighting/shadows differ substantially. The latest own capture at13:52 still differs. No full rendering-parity claim is justified.


## 2026-10-29 — absent sunlight/shadows follow-up
Both mirrors checked at unchanged f7a45b3b7aac9bd9f4662d104d276d421c1fca52 / 6562163b6f504a1db93e1be5ce580d8cd6f024a7. Re-read Android worldrenderer.cpp lighting (0.4 ambient factor); our PSP GL bridge uses white ambient material, which is unverified and may saturate colours when added to diffuse. Do NOT silently copy Android factor as PSP proof. Original PSP sunny tables contain warm colours at13:00; matching warm input alone is insufficient. Corrected WeatherManager initial hour/minute to match --start-time, avoiding a spurious first-frame hour transition. New13:52 native capture still grey. Projected shadows remain absent. Visual goal NOT achieved.


## 2026-10-30 — initial projected shadows
Both mirrors fetched at unchanged Android f7a45b3b7aac9bd9f4662d104d276d421c1fca52 / PSP6562163b6f504a1db93e1be5ce580d8cd6f024a7. Reviewed sibling model parser and renderer; Android model format differs. Inspected PSP GE command-writing sequences0x088d2450 onward and0x08982090; the latter is a separate pass and must not be assumed world materials. Candidate0x0885fde0 is allocator poison0x55667788, NOT material0x666666; rejected. No verified world material coefficient recovered.
Implemented an explicitly partial native planar shadow pass from opaque ISO mesh geometry, current table-driven sun and channel19 opacity. This is NOT a translation of original shadow code. Stencil limits each ground pixel to one contribution; alpha-cutout foliage excluded. Static world and standing props only; no dynamic car/ped shadow replacement. Existing clock-start correction retained and tested. Native capture confirms lamp silhouettes now visible; sunlight/yellow tone still unresolved.


## 2026-10-31 — PSP Double Armour (`cPlayer::ReduceArmour`) verification and Rust implementation
Preflight completed against CTW-Native (`work/upstream-ctw` at `be18fd3a4f7842ff7b74f4d204207339a2b346f7` / `references/ctw-native` at `f7a45b3b7aac9bd9f4662d104d276d421c1fca52`) and highcmd (`6562163b6f504a1db93e1be5ce580d8cd6f024a7`). Verified PSP Europe (`ULES-01347` v1.01) via Ghidra 12.2 + `ghidra-allegrex` v21.5 + `CtwQuery.java` on `CTW_Working.gpr`: `cPlayer::ReduceArmour` (`0x089c967c`), `cPed::AddArmour` (`0x089c7c50`), `cPed::SetArmour` (`0x089c7d08`), `cPlayer::FixDamageForArmour` (`0x089c8f20`), `cPlayer::SetHasDoubleArmour` (`0x089c892c`/`0x089c8930`), `cPlayer::Reset` (`0x089c8ff4`), `cPlayer::cPlayer` (`0x089c9bf0`), and respawn handler `0x08897e78`. Also compared against Android reference `Player::reduceArmour` (`work/upstream-ctw/port/game/player.cpp`) and `references/ctw-native/tests/armour_test.cpp`.
Confirmed PSP mechanics: bit `0x2000` of `flags_f14` (`+0xf14`) enables Double Armour; bit `0x4000` alternates 1-point hits (`0, 1, 0, 1...`); damage $\ge 2$ is halved via unsigned shift (`srl $a0, $a0, 1`) without touching bit `0x4000`; halving and 1-point alternation occur before comparing against `this->armour` (`+0x247`), so overflow to health and hits at `armour == 0` are also halved/alternated; armour ceiling remains fixed at `127` (`0x7f`); `SetHasDoubleArmour` and `Reset` clear bit `0x4000`. Implemented in `src/jobs.rs` and `src/actors.rs` with unit tests.


## 2026-10-31 — Jalon 0 C++ static recompilation feasibility (`work/cpp-recomp-feasibility`)
Mandatory preflight completed via `python tools/maintenance/check_upstreams.py`: CTW-Native checked at `f7a45b3b7aac9bd9f4662d104d276d421c1fca52`, highcmd at `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Reviewed `highcmd/chinatown-wars-psp-recomp` (`setup_from_iso.py`, `psp_prx.py`, `analyze.py`, `recomp.py`, `NOTES.md`, `docs/BUILDING.md`, `data/hooks.json`, `data/cheats.json`, `runtime/src/game_hooks.cpp`, `runtime/src/main.cpp`).
Executed Jalon 0.1 (SCEE overlay extraction), Jalon 0.2 (bidirectional static analysis of `CTW-PSP.elf` + 166 SCEE overlays), and Jalon 0.3 (C++ source generation) strictly inside the isolated ignored directory `work/cpp-recomp-feasibility/` without modifying `src/` or `references/`. Confirmed 166 SCEE overlays (`0x08b87560` Slot 0 and `0x08bb2580` Slot 1, shifted by `+0x100` vs. SCEA), 12,222 main-ELF functions, 22,579 overlay functions, 204 + 137 jump tables, and complete generation of 82 main C++ files + 166 overlay C++ files (`build/gen/`).



## 2026-10-31 — Milestone 0 independent pre-build validation

Preflight attempted: both remotes failed DNS resolution. Local Android HEAD b0506abc786fea229101c56e0a64726aa8b4ac97; local PSP HEAD 6562163b6f504a1db93e1be5ce580d8cd6f024a7. Reviewed PSP tools/recomp.py overlay signatures, tools/analyze.py discovery, data/hooks.json, data/cheats.json and runtime/src/game_hooks.cpp (GPL-3.0-or-later). Android is not used for EU address proof; no runtime changes. Current remote freshness cannot be certified. Audit isolated in work/cpp-recomp-feasibility; all unproven regional mappings must remain unproven.

Validation outcome: wrapper ABI/categories/buffer repaired; analysis and generated files reproducible, signatures collision-free. NO-GO on regional hook/cheat proof (USA binary unavailable) and import binding qualification. Local Android recent commits reviewed (b0506ab wip, a31e94b porter,48f20b5 death app); no relevant EU mapping evidence. Full report is private work/cpp-recomp-feasibility/reports/VALIDATION_PRE_BUILD.md.


## 2026-10-31 — Jalon 1B.0 runtime audit

Preflight attempted; configured remotes unavailable due to DNS. Local PSP pin6562163b6f504a1db93e1be5ce580d8cd6f024a7, Android HEADb0506abc786fea229101c56e0a64726aa8b4ac97. Reviewed actual PSP CMake/toolchain/win_deps.py, core dispatch/HLE registry, main boot chain, CPU/VFPU headers, kernel/fibers, GE/audio/filesystem and game_hooks.cpp (GPL-3.0-or-later; PPSSPP portions retain GPL-2.0-or-later notices). Android does not determine this ABI. No upstream edits/merge. Existing COFF objects scanned with llvm-nm/readobj:39 residual native symbols; source HLE registration scan203/256 name matches,53 absent matches. Missing-import fallback currently returns0 and must not be mistaken for supported service. Report: work/cpp-recomp-feasibility/reports/JALON1B_RUNTIME_AUDIT.md. No runtime compilation or link/startup.


## 2026-10-31 — Jalon 1B.5 critical import documentary audit

Mandatory preflight attempted; Git fetches failed DNS. Local highcmd pin 6562163b6f504a1db93e1be5ce580d8cd6f024a7 (GPL-3.0-or-later), Android HEAD b0506abc786fea229101c56e0a64726aa8b4ac97. Inspected recent Android log/stat (Porter/gameplay, no applicable PSP HLE ABI contract), highcmd recomp.py import index selection, core.cpp dispatch/registry/fallback, actual kernel/time/display/extra implementations and remaining CMake source/dependency inventory. No upstream import, merge or modification. Primary PPSSPP/PSPSDK documents were readable via web despite Git DNS failure; URLs and qualification recorded in the isolated report. No mirror freshness claimed.

Material correction: 256 raw import records describe 252 stub addresses; four of the prior 53 missing-name records are shadowed by later proper records. Existing generated and indirect dispatch select the later records. Effective selected-stub source coverage is 203/252, with 49 absent (9 non-network, 40 network). No alias, registration or fallback changed. Four existing providers are in still-uncompiled kernel_misc.cpp. The 13 requested records, 14 EU direct sites, criticality and 14 remaining runtime C++ files are documented in work/cpp-recomp-feasibility/reports/JALON1B5_HLE_CRITICAL_AUDIT.md. Ghidra launcher denied settings access before project query; no fresh decompilation claimed. No compilation/link/run/install/commit; stop awaiting validation.
# 2026-11-01 — First playable C++ supervised session

Mandatory preflight ran for this mission; all configured remote fetches failed DNS (`github.com`). No current upstream verification is claimed. Local highcmd remains `6562163b6f504a1db93e1be5ce580d8cd6f024a7`; its actual `hle_display.cpp` input parser and button logging, plus portable `main.cpp` path behavior, were reviewed. `CTW_INPUT` is parsed once and cannot be cancelled after a successful minigame. Local Android reference is `b0506ab` (not the preflight cached last-success revision); its player source explicitly states swimming is not ported. It is not evidence for swimming controls. Local Vita `90809be` loader was searched; no directly reusable sinking/swimming control implementation was found. No upstream code imported. EU overlay 138 update `O138_08BB7D84` reads masks `0x100/0x200`, requires release to rearm byte `+0x13c`, and gates strikes on state `+0x104 == 2` and `+0x12c != 5`.

All experiment code, copied launch files, logs, screenshots and handoff are isolated under `work/cpp-recomp-feasibility/`. No Rust metric regeneration: this experiment adds no Rust progress. Only one native session of at most 300 seconds is authorized; no runtime edits, rebuild or relink are authorized.

# 2026-11-01 — Single 60-second boot reproducibility diagnostic

Preflight task `One 60-second boot reproducibility diagnostic with built-in caller, wait and savestate traces` failed DNS on all configured remotes. No live remote review claimed. Local highcmd `6562163b` runtime `core.cpp`, `kernel_thread.cpp`, `savestate.cpp` reviewed: caller logging runs in the selected one-second window; long-wait logs occur on resumed waits; a scheduled save needs the scheduler idle/service gate. Local Android `b0506ab` recent WIP/Porter/death-app commits provide no demonstrated fix for PSP Bink/kernel synchronization; no code imported. Historical successful 1B.15 and 1B.17 logs both show zero connected gamepads, correcting the preceding report's controller comparison. User authorizes one <=60s run with existing instrumentation only; no runtime changes, interprocess memory read, privilege escalation, build or commit.


## 2026-11-02 — Official C++ mainline migration and publication

Mandatory check_upstreams.py ran and Git fetch failed DNS. A separate GitHub connector check succeeded: highcmd main 6562163b6f504a1db93e1be5ce580d8cd6f024a7; Android main 108320b137c3ca748d697838f86c8919c12b3f2c. Both match the inspected local mirrors with correct independent origins. This does not turn the failed Git fetch into a successful fetch. Reviewed highcmd launcher 0.1.5/HUD scaling and Android 0.5.0, Linux packaging and GCC include changes; no unrelated upstream changes are merged. Canonical runtime copies retain the pinned highcmd revision and six previously validated adaptations, documented per file in runtime/provenance.json. highcmd GPL-3.0-or-later, inherited PPSSPP GPL-2.0-or-later and Android GPL-3.0-only attribution retained. PSP USA and Android addresses/data remain unsuitable as unverified Europe replacements.

## 2026-11-02 — Full-title logo and C++ status visualization

Mandatory preflight failed (network unavailable). Independent GitHub connector reads confirm unchanged highcmd 6562163b6f504a1db93e1be5ce580d8cd6f024a7 and Android 108320b137c3ca748d697838f86c8919c12b3f2c. Reviewed their recent HUD/launcher and GCC/release changes: no related branding or Europe coverage algorithm to import. Local highcmd core.cpp HLE registry/selected-stub behavior and current canonical runtime registrations are the evidence for a clearly labelled PSP-import binding percentage. No gameplay-completion percentage is inferred, no Android code counts mixed with Rust or C++. No upstream modifications.

## 2026-11-02 — Original PSP code recompilation percentage

Mandatory preflight ran; configured mirrors reported unavailable. Independent GitHub reads still pin Android 108320b137c3ca748d697838f86c8919c12b3f2c and highcmd 6562163b6f504a1db93e1be5ce580d8cd6f024a7. Read Android scripts/progress.py measure/percent and CONTRIBUTING.md: sum of original routine machine-code sizes marked ported, divided by the original-size inventory; its declaration/name heuristic is not PSP proof. Read pinned PSP generator FuncGen, traced overlay boundaries, hooks and original/wrapper emission. Adapt only the arithmetic principle and treemap presentation: use Europe's own function-size inventory and source/object SHA256 + COFF checks, not Android classes/25,988 routines or HLE count. No upstream code merged, references modified, runtime rebuilt or game executed.

Measured current evidence: 12,307,572 of 12,307,576 indexed original bytes count; 34,800/34,801 routine candidates, across main ELF and 166 overlays. All 250 generated units match compiled source/object evidence. Four-byte mapapp candidate remains excluded because its body contains rt_unhandled; earlier read-only analysis suggests data, not established removal authorization. Display truncates 99.99996749...% to 99.99%, never rounds known remaining work to 100.00%. This is static indexed-code coverage; gameplay fidelity, HLE services, callback reachability and code/data-boundary completeness remain separate.

## 2026-11-02 — Separate playable readiness from code generation

Mandatory preflight ran; mirrors reported unavailable. Independent GitHub reads verified unchanged highcmd 6562163b6f504a1db93e1be5ce580d8cd6f024a7 and Android 108320b137c3ca748d697838f86c8919c12b3f2c. Re-read Android scripts/progress.py source-name matching and original-size weighting, plus pinned PSP generator/our body-object metric from the preceding audit. The same weighted arithmetic does not imply comparable numerators or finished gameplay: Android source-porting references differ from automatically generated PSP bodies. Corrected the primary dashboard to playable-readiness evidence with no fabricated global percentage. Numeric byte coverage and routine-count badges remain labelled technical diagnostics. No upstream merge, reference edit, runtime build or game run.

## 2026-11-02 — Visible playable status map

Mandatory preflight ran; mirrors reported unavailable. Independent connected GitHub reads confirm Android 108320b137c3ca748d697838f86c8919c12b3f2c and highcmd 6562163b6f504a1db93e1be5ce580d8cd6f024a7 unchanged. Reviewed their already-pinned recent GCC/release and launcher/HUD changes plus the progress/reference distinction from the prior review; no gameplay algorithm is relevant to this presentation fix and none is imported. Browser inspection confirmed that GitHub displayed only the generic readiness cards while the dense green diagram was folded into diagnostics. Replaced the primary cards with a visible evidence map: observed portions and remaining work, with no inferred completion percentage. Runtime, original PSP data and upstream sources remain unchanged.

Benjamin additionally requested a Kryo-style mosaic alongside the functional map. Reused our existing private PSP function/size inventory and verified body/object evidence to render a blue module/routine treemap, without percentages in the visible overview. Android class sizes, addresses and routine counts are not borrowed. All runtime sources and original game data remain untouched.

## 2026-11-02 — Functional scenario mosaic, green/red request

Mandatory preflight rerun; all mirrors remain unavailable. Connected reads verified the same Android main 108320b137c3ca748d697838f86c8919c12b3f2c and highcmd master 6562163b6f504a1db93e1be5ce580d8cd6f024a7. Previously reviewed source-porting/generated-code criteria remain relevant: Kryo-style areas alone cannot certify gameplay. Benjamin clarified that the visible mosaic should represent actual project validation rather than nearly complete generated output. Replaced the default routine mosaic with 54 evidence-scoped scenarios from the existing owner report, migration report and bounded kernel fixture. No new upstream changes are needed for the presentation. Automatic generation stays secondary; no Android progress or guessed weights are used.

# Upstream Repositories Review — 25 October 2026

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

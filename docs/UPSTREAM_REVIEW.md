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

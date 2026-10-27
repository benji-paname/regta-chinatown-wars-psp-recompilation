# Driving and traffic repair — 27 October 2026

The delivered Windows build replaces the previous empirical grounded handling with a Rust adaptation of CTW-Native's engine and tyre calculations, using handling records read from the user's PSP ISO. Ambient traffic now keeps a continuous route through intersections, waits at lights and queues without reversing, and validates its actual spawn position. This is a substantial prototype repair, **not a certification of full PSP driving equivalence**.

## Sources and provenance

- PSP Europe ULES01347 1.01: PAK 29 (106 records of 312 bytes), `ai.bin` road network and `world.bin` collision data, read locally from the owner's ISO. No proprietary resource files are included in this change.
- [CTW-Native, pinned b0506abc](https://github.com/official-kryo-to/ctw-native/tree/b0506abc786fea229101c56e0a64726aa8b4ac97/port/game): `vehicle.cpp`, `traffic.cpp`, `aispline.cpp`, `roads.cpp`, `trafficavoidance.cpp` and `driving.cpp`. GPL-3.0-only, Copyright 2026 Kryo.to. Algorithms are adapted into Rust; there is no C++ runtime dependency or emulated CPU.
- [highcmd PSP recomp, pinned 6562163](https://github.com/highcmd/chinatown-wars-psp-recomp/tree/6562163b6f504a1db93e1be5ce580d8cd6f024a7): reviewed its README/notes and 30 Hz logic context. Its USA release data and MIPS-to-C++/HLE runtime are not interchangeable with this Europe Rust target.
- Both mirrors fetched on 27 October after the initial network-restricted attempt failed. At the evening resume CTW-Native advanced to `95adc327ddd8357701ca3885c61fd6b4a40f193b`; reviewed the new changes. Grounded drivetrain/traffic source files are unchanged, so this adaptation remains pinned to b0506abc. Highcmd is unchanged. Review: [UPSTREAM_REVIEW.md](UPSTREAM_REVIEW.md).

## Repairs

| Area | Corrected behaviour |
| --- | --- |
| Grounded drivetrain | Separate front/rear wheel spin, load distribution, torque curve, wheel force feedback, drive/brake splits, gearbox delay and steering reduction at speed. Per-vehicle parameters come from PAK 29 at runtime. |
| Integration | Fixed 30 Hz vehicle updates, with 136/4096 physical integration interval. Turning the body preserves world momentum and reprojects it into the new body axes; rotating a car no longer rotates its existing velocity artificially. |
| Controls | Accelerator and service brake remain independent. The controller's vertical stick axis no longer cancels trigger/button acceleration. Handbrake, reverse and explicit accelerator+brake burnout are distinct. |
| Damaged vehicles | Disabled/destroyed vehicles cannot accelerate or release a stored burnout impulse. Wheel state is resynchronized after collisions. |
| Traffic routing | Link metadata uses the appropriate endpoint; junction selection filters candidates before choosing. Curve arc length and residual travel keep junction transitions continuous. Failed world moves roll back the route cursor. |
| Traffic waits | Service braking continues moving the vehicle toward a stop. Distant cars do not halt an entire avenue. Normal red lights/queues never trigger reverse recovery. Original reference light phases are shared across approaches. |
| Traffic placement | Initial positions use the selected candidate instead of every vehicle being placed at the edge midpoint. The final spline pose is checked for player/ped/car clearance. Recycling reserves each accepted position and checks the viewport. |
| Recovery | Physical recovery aims toward a directed lane; rail attachment is restricted to aligned, offscreen, straight-lane conditions. It no longer teleports directly to the road or alternates both movement systems in one update. |
| Police | Existing pursuit targets now supply controls to the common drivetrain and world collision path. Police bookkeeping no longer moves cars a second time by directly editing position, angle and speed. |

The speed variation seed uses stable local vehicle attributes in place of upstream entity UIDs. Panic/yield/recovery steering remain host behaviours. The bumper-clearance query is a simplified substitute for the full upstream avoidance system.

## Validation

- `Compiler.ps1 -Test`: **213 passed, 0 failed, 0 ignored, 0 warnings**, with warnings treated as errors. Fourteen new regressions cover frame-rate independence, steering reduction, disabled-engine protection, distinct burnout controls, queues, distant obstacles, red/green restart, spline handover/spawn continuity, rollback, reattachment, opposing lanes/bridge height separation and viewport projection. The police test also rejects direct movement during bookkeeping.
- Three Python inventory-integrity tests pass. Source status now excludes comments/string literals and separates runtime definitions from test/helper definitions. It no longer distributes Rust function counts over Android binary routine sizes to claim a PSP completion percentage.
- Final ISO audit: **92 land vehicles**, each exercised for 900 ticks through acceleration, turning, coasting, braking, reverse, handbrake and burnout; **166 sampled real-map routes**, up to 60 seconds each. No non-finite/excessive trajectories or jumps over one world unit were reported. Largest route step: approximately **0.529** world units; minimum route travel: **99.24** units.
- **Three routes encountered world collision** and requested physical recovery. They start at directed edges `197→142`, `443→428`, `626→1439`. This audit stops when rail recovery is requested, so it does not prove they subsequently recover successfully. The ignored JSON records the ending route/position for follow-up.
- Native OpenGL captures on RTX 5070 Ti: three-second acceleration and turning runs, plus a twenty-second populated street run. Turning run: about **65.0 world units travelled**, zero player collision events. Straight acceleration deliberately reached a solid planter and stopped. These are scripted smoke checks, not a physical-controller playtest or an original-game comparison video.

Reproduce from the repository root:

```powershell
powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test
python -m unittest discover -s tools/progress -p test_progress.py -v
& .\target\release\ctw-rust.exe --iso "work/Chinatown-Wars-PSP.iso" --driving-audit --output target/driving-audit
& .\target\release\ctw-rust.exe --iso "work/Chinatown-Wars-PSP.iso" --map --slice --demo-drive --headless --frames 180 --walk 0.5 1 --capture target/driving-turn.bmp --output target/driving-turn
```

Use the owner's actual ISO path if the local ignored copy is absent. Logs, captures, build report and diagnostics live in ignored `target/`; they are not redistributed. The ready-to-run executable is `outputs/CTW-Rust/CTW-Rust.exe`.

## Remaining fidelity work

1. Verify the Android reference **gear-ratio constants** against Europe PSP code/data. Exact sequence searches in the private PSP ELF did not locate them. Do not label those constants PSP-verified.
2. Port/validate vertical suspension, slopes, airborne motion and the full original collision response. The current world sweep uses a simplified footprint; most car types still share an Admiral-sized world footprint. Bike lean/body sway and impacts remain simplified.
3. Investigate the three blocked route continuations; validate recovery and long multi-vehicle intersection runs. Offscreen streaming is guarded, but complete original population/avoidance parity is unproven.
4. Compare deterministic input traces with original PSP recordings, including joystick steering, braking distance and collisions. Keyboard/script checks do not certify physical XInput hardware.
5. Original police tactics, panic driving and surface-dependent grip are not fully ported. Grounded arithmetic uses Q12 with quantized floating-point trigonometry; it is not bit-exact MIPS execution.

## Review of Gemini's independent audit

[GEMINI_TRAFFIC_AUDIT.md](GEMINI_TRAFFIC_AUDIT.md) is retained as supplied, with these corrections:

- Nodes at `z=-7` are **water/boat routes**, not bridge decks. `RoadNetwork::meta` explicitly branches on raw `n.z < -4` for boats. They must not be enabled for land vehicles.
- This project iterates directed links both ways when both links exist. Lane zero on the reversed link provides the opposing carriageway; forcing lane one without reversing the link would introduce wrong-way traffic. The regression checks actual lane separation.
- The reference `0x4CE/4096` speed increment at 30 Hz is approximately **9.0 units/s²**, not 3.5.
- The audit's distant-obstacle stop finding was valid and repaired. Its claim that the new rail mover discards residual distance was outdated: the implemented transition carries it forward and now has a regression test.
- The C++ algorithms inspected in that report are from the **Android reference port**, not independently recovered PSP engine routines.

Next-agent entry point: [AI_HANDOFF_GUIDE.md](../outputs/CTW-Rust/AI_HANDOFF_GUIDE.md).

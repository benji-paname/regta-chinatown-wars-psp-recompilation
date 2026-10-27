# Original driving cameras and steering assistance

Research and native implementation, 28 October 2026. Target: PSP Europe ULES01347 1.01.
No game binaries, extracted assets or disassembly are distributed with this document.

## Evidence

Private inputs and hashes are recorded in `CAMERAS-ORIGINALES.md`. The relevant functions are:

- PSP ELF: FollowCarCam2 constructor `0x088ca1f8`, placement `0x088ca390`, update `0x088ca5b8`, globals initializer `0x088cc17c`.
- DS YGXP ARM9: constructor around `0x020c56cc`, update `0x020c5778`, placement `0x020c62bc`, globals initializer `0x0214bd58`.
- Supplied Android APK: `cFollowCarCam2::Update` at `0x0072c9f0`, placement `0x0072d584`.
- GPL-3.0-only structural reference: Kryo.to `port/game/carcam.cpp`, commit `95adc327ddd8357701ca3885c61fd6b4a40f193b`. Unchanged at reviewed tip `f7a45b3b7aac9bd9f4662d104d276d421c1fca52`.

**Do not copy the reference port's immediate reverse-offset flip.** All three original binaries contain an orbit state and a signed angular accumulator that the reference port currently simplifies.

## Recovered ordinary-driving parameters

Positions/speeds use Q12; angles use 65536 units per revolution. Update rate is 30 Hz.

| Parameter | PSP | DS | Android phone |
|---|---:|---:|---:|
| Base height | 27 | 27 | 28 |
| Maximum / large-vehicle height | 39 | 39 | 42 |
| Base horizontal distance | 10 | 10 | 10 |
| Small-vehicle half-length threshold | `0x5666` | `0x5666` | `0x5666` |
| Horizontal spring | 400 | 400 | 400 |
| Vertical spring, after Q12 multiplication by 409 | 1475 | 1475 | 400 |
| Previous-velocity retention | 500 | 500 | 500 |
| Pitch target | `0xd4e4` | `0xd4e4` | `0xd4e4` |
| Speed/altitude lift term | disabled | enabled in ordinary play | enabled in ordinary play |

The DS routine at `0x020c55b0` belongs to the preceding camera implementation; its height 28 / distance 6 must **not** be used for FollowCarCam2. Its own initializer confirms 27 / 10.

Small vehicles compute `per_frame = speed * 136 >> 12`, then a squared-speed term multiplied by `0x7800` in Q12. Horizontal distance subtracts half this term. DS/Android additionally use clamped speed above 20, divided by 70, multiplied by altitude above 2; lift is capped at 2, subtracting 12 times lift from height and 6 times lift from distance. Large vehicles use the fixed maximum height.

State 0 requires signed forward speed strictly below -8 for 31 consecutive updates before entering orbit state 1. The transition tick retains the old offset. State 1 rotates by `speed * 60 >> 12`, clamped to a signed half turn; returning forward unwinds it. The state clears only above +8 and with angular error below `0x38f`. A stationary constructor starts in state 1 (speed <= 40 Q12), so an entry from rest is not the same case as reversing from a moving state-0 camera. Stopping retains the current side of the vehicle. Holding the camera action for more than 20 updates accelerates centering using the original `0x64000 * 60 >> 12` rate.

Runtime integration now selects this controller from the current occupied vehicle after simulation, rather than treating driving as on-foot sprint/recenter. Entry retains the outgoing pose, exit resets on-foot counters, and changing profiles while driving retains the pose and orbit. Freecam suspends follow updates. `F3` / right-stick click cycles PSP, mobile, DS.

## PSP steering assistance

Located independently in `cPlayer::FillInDrivingYoke`, PSP `0x089cb264..0x089cb720`; Android counterpart `0x0088cce4..0x0088cec0` confirms the structure.

- Enabled by the player's original `0x400` setting bit; applies to land vehicles when no explicit steering or brake/reverse input is present.
- Normalizes horizontal forward direction, searches the four cardinal axes and requires dot product >= `sin(0x4ccc)` (approximately an 18-degree window).
- Remaps that dot product to Q12 [0,1], evaluates the inverse-cosine helper (`0x089c5e3c`: square root and arctangent), assigns the cross-product sign and scales the resulting angle to a quarter-strength yoke.
- This uses the proportional-steering flag: APK Act `0x008f0fe8..0x008f0ffc` maps it to bit `0x20`; UpdateSteering `0x008f268c..0x008f26a8` selects yoke times half steering lock. It is not the normal digital steering-rate accumulator.

Native assistance is enabled by default for the player only; existing NPC driving does not enable it. `F6` / `RB + right-stick click` toggles it. The preference is saved with the camera profile. `--steering-assist on|off` overrides it for reproducible runs. Manual steering keeps the previously validated drivetrain path.

## Boundaries and validation

These are grounded ordinary-driving implementations, not a claim of complete original-game parity. Original trigonometric lookup tables, full 3D orientation/suspension, exact swept-sphere world queries, cinematic/mission gates, camera distance settings, burnout-camera override, the alternate reverse-disabled state 2 and final near-axis quaternion snap are not fully reproduced. Mobile selects the phone variant, not tablet. The supplied DS/Android data serve as camera references; the world and handling data remain PSP.

Regression tests cover reverse thresholds/delay/unwinding, interrupted reverse, stopped-side retention, angle wrapping, profile framing, freecam/exit transitions, assistance range/sign/manual priority and its physical steering direction. Native capture reports identify the active controller and record camera pose alongside vehicle speed. See the active AI_HANDOFF_GUIDE for the exact completed test/build results and visual checks.

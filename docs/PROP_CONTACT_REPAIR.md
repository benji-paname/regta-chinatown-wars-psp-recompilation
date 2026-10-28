# Prop contact repair — 28 October 2026

Compared Benjamin's private recordings `OG PSP.mp4` (148.87 seconds) and `Recomp Rust .exe.mp4` (105.63 seconds). Reviewed contact sheets across both recordings and quarter-second sequences around PSP 100–105 s / Rust 84–89 s. These runs use different vehicles, speeds and camera poses: they establish visible defects, not a numerical 1:1 physics calibration. Private frames remain in target/prop-video.

## Verified defects and corrections

- Smash effects previously injected fixed upward velocity and spin even for nonphysical damage. Physical contact now supplies horizontal momentum separately from the one-time smash effect. Broken appearance and moving state are separate.
- Vehicle impacts previously bypassed thresholds whenever an effect existed or speed exceeded 2.2. Thresholds now use a mass/relative-speed impulse estimate; separating and vertically disjoint contacts do not smash objects.
- Loose objects previously lost all vehicle interaction. Retained meshes now accept another impact without duplicating paper/cash/sound.
- Tumbling previously rotated about the placement origin and clamped that origin to ground, embedding the mesh. Rotation now uses the primitive centre and a rotated-box support height. PSP box/cylinder offsets are bottom positions; bounds use the first primitive, as in the reference.
- Paper previously rose indefinitely and regenerated on every resting bounce. Mail/news bursts emit once, seven particles, four-second lifetime, no inherited car velocity; paper now accelerates downward with a terminal speed. Particle randomness/rendering remain host approximations.

## Evidence and limitations

User ISO PAK1866/1867 confirms mailbox definitions 69/70: mass 0.7998, smash/uproot 5, retained mesh 65535; news dispensers 83–85: mass 1, thresholds 5, retained mesh. Their movement is intentional: do not simply delete the models. Kiosk 66 has uproot -1 and a replacement broken model.

GPL-3.0-only reference, Copyright 2026 Kryo.to: ctw-native f7a45b3b7aac9bd9f4662d104d276d421c1fca52, propdynamics.cpp contactForces/applyForce/smashParticles, rigidbody.cpp inertia/damping/gravity, src/world/props.cpp physics/smashedModel, particles.cpp PaperEmitter. Both upstream tips remain unchanged; highcmd 6562163b6f504a1db93e1be5ce580d8cd6f024a7 targets PSP USA and HLE, not our Rust runtime.

This is a repair, NOT fully authentic PSP physics. Contact response currently adapts the translational reference term, not complete sphere/contact inertia or penetration response. Angular motion is a single-axis host integration; ground bounce restitution remains inherited, ground height stays at the placement plane, dynamic world-wall/prop-to-prop collision is still missing. Exact PSP code correspondence for the Android gravity/damping constants has not been established. Cash values and the remaining effect-specific fire/lamp/hydrant implementations are inherited and not certified by this repair. Earlier handoff claims of complete authenticity were overstated.

## Reproduction

`--prop-impact 69 --headless --frames 180 --walk 0 1 --brake-at 30` places a diagnostic car before the nearby mailbox (requires the original ISO). Use `--capture target/prop-video/mailbox-after.bmp --output target/prop-impact`. Reports now include props_end with positions, velocity, smashed and sleep state. This diagnostic relocates only its own explicit test car; it does not alter ordinary gameplay spawning.

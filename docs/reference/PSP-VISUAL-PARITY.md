# PSP visual parity — incomplete

Reference: user ISO ULES01347 1.01; OG PSP.mp4 (same district, different clock/weather from runtime).

## Runtime repair, 29 October 2026
- Read SS_Weapons.bin and SS_AlwaysLoaded.bin from the ISO, validate IDs, material, inclusive extents and signed anchor offsets. Render PAK374 and PAK382; normal texture-pack substitution remains supported.
- Replace the invented weapon name panel with the actual PSP icon, and show clip/reserve ammunition. Previous hardcoded names did not match the weapon type enumeration (types 0–4 are melee).
- Remove duplicate wanted indicators. Use the ISO star and police-car sprites; preserve the existing gameplay wanted/takedown state. Sprite artwork is original; placement, fading, counters and timers are NOT yet executable-verified.
- Cash remains visible in optional PC information bands and PDA; remove permanent overlapping cash panel from normal gameplay. Original transient cash meter is still missing.
- Disable arbitrary saturation/contrast/warmth/vignette by default. CTW_LEGACY_GRADE=1 is an explicit non-reference diagnostic opt-in. Neutral output is NOT a completed PSP color pipeline.
- Lightning is blended over the world before the HUD, rather than overwriting the HUD without blending. Weather timing/intensity remains prototype.

## Still required before claiming 100%
1. Recover PSP clock, weapon fade/placement, wanted animations, cash meter and notification routines. Validate all weapon icons against PSP gameplay. Current icon identities are visually matched to the atlas, not a recovered PSP switch table.
2. Port the actual pause/PDA/menu layouts, backgrounds, original text and interactions. Existing custom PDA, large map and minigame interfaces are unchanged; they are NOT authentic final menus.
3. Recover PSP timecycle data and renderer bindings: directional/ambient lighting, material flags, fog, water and color effects. Current baked face-normal lighting and elliptical contact shadows remain approximations.
4. Recover projected building/vehicle/pedestrian shadows and compare synchronized captures at the same location, time, weather and camera. Videos alone cannot determine the original light constants.
5. Keep Benjamin's signature, optional PC information bands, Xbox/keyboard controls and HD replacement compatibility as intentional port features.

Private QA: target/hud-psp-after.bmp and target/hud-psp-check/play-validation.json. These contain user assets and must not be committed. No new proprietary assets are bundled.


## Timecycle integration — 29 October 2026
The world meshes now use ISO timecycle.dat ambient RGB (file channels 13–15) and sun RGB (0–2), with dynamic sun direction from channels 3–4. Colours follow the runtime weather/clock; no artistic screen filter. The original file is read at runtime, not included in Git. Current GL bridge replaces fixed baked grey shading for world meshes only. Pedestrians, vehicles, props, water, fog and shadows are not yet fully connected. World face normals are reconstructed from triangles, not recovered original vertex normals. Second directional light, specular/material flags, lightning light modulation and exact PSP sun-direction arithmetic remain incomplete. The clock/weather scheduler is still the prototype's, not original parity.

Original ULES01347 evidence (ELF SHA256 9eaf850f09d743e5ecfc8d1c6a0a8c290399142319b1ef3a732a03d7278bffe0):
- timecycle.dat WAD hash 0x6252da25, length 42240 = 8*55*24*4.
- 0x08a8d8c4 references filename at 0x08b4b3f8; initialization 0x08a8d9c4 onward reads RGB triplets and angle channels. First two RGB triplets each followed by two angle tables; third followed by ambient triplet at object+0xd00.
- 0x08894020 fills missing -4096 hours around midnight using signed division, with angle wrapping only on selected channels.
- 0x088942bc initializes three channels then 0x0889424c converts each byte through powf(x/255, float bits 0x3f19999a)*255 and truncation. Native host powf not yet exhaustively compared to PSP math library.
- 0x08a8ec3c expands RGB bytes by 0x800; 0x08a8eb90 uses floating interpolation with (fraction>>1)/2048. This differs from the Android integer-rounded path.
- 0x08a8dfd4 normalizes colours using float bits 0x36008081; 0x088d2618 selects ambient slot13, 0x088d2644 diffuse slot0, 0x088d2670 specular slot16 for GE light commands. The current GL bridge omits specular.

Validation: 244 unit tests, zero warnings; deterministic native capture target/timecycle-after.bmp, same camera/location as target/hud-psp-after.bmp. New tests cover midnight fill, angle wrap, absent data, incompatible Android size, full-intensity RGB, and PSP odd-fraction quantization. Capture demonstrates runtime hookup, NOT pixel-exact parity with the user's video (different time/weather).

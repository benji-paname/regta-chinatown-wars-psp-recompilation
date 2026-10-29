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

# C++ mainline provenance — 2026-11-02

The canonical runtime now resides in runtime/. Its63 source/header files derive from highcmd/chinatown-wars-psp-recomp at6562163b6f504a1db93e1be5ce580d8cd6f024a7 (GPL-3.0-or-later), including six validated local adaptations. Per-file SHA256 provenance is in runtime/provenance.json. Original notices are retained. VFPU and SAS portions originate in PPSSPP (GPL-2.0-or-later), as noted in those files. Compatible inherited terms remain applicable; this repository's original contributions remain GPL-3.0-only.

External local dependencies: SDL2 2.30.9 (zlib), FFmpeg supplied by the existing local development tree (build-dependent LGPL/GPL terms; current upstream package described as GPL), Khronos OpenGL headers (their permissive notices), Microsoft/LLVM build tools (their own terms). No DLL, SDK, table binary or dependency package is redistributed by this migration. Binary packaging/source-offer compliance must be separately reviewed before any release. Linking successfully is not redistribution clearance.

The250 translated game units and all original/extracted game content are PRIVATE despite being source-like text or required to run locally. None is covered by the runtime GPL grant. The public source alone is not a ready-to-run game.

Benjamin confirmed on2026-11-02 that he has a license or authorization for the supplied logo font artwork. This is owner confirmation, not an independently reviewed license document. No font software or game capture is published. The detailed historical Rust provenance follows; it does not describe the active runtime.

---

# Provenance and licenses

## ReGTA CTW PC Recomp

The project code is licensed under **GPL-3.0-only**; the full text is in [LICENSE](LICENSE). Game data is separate from the code and is not licensed or distributed by this project.

## ctw-native

Copyright (C) 2026 Kryo.to. **GPL-3.0-only**.

Source: https://github.com/official-kryo-to/ctw-native

Previously recorded consulted revision: `5fa1e36672ee88b0b232c3dc23652be0a33c7f17`. The preserved local source snapshot is under `work/upstream-ctw/`; the exact revision of the supplied Windows binary was not established.

Rust adaptations and reference work include:

- `wad.rs`: name hashing and GXT string interpretation from `port/tools/wad.py` and `port/src/text/gxt.cpp`.
- `model.rs`, `world.rs`: hierarchy, transformations, triangle strips and world organization from `port/src/gfx/model.cpp` and `port/src/world/worldmap.cpp/.h`. PSP binary layouts were investigated separately.
- `collision.rs`: shape/query structures and player-controller reference algorithms from `port/src/world/collision.cpp/.h` and `port/game/player.cpp/.h`; PSP cell directories differ and collision handling remains incomplete.
- `ped_paths.rs`, `pavement.rs`: network structures and route selection informed by `collision.cpp/.h`, `peds.cpp` and `localarea.cpp`, checked against the user's PSP world graph.
- `camera.rs`: reference follow-camera and obstruction algorithms from `followcam.cpp/.h` and `cameraobstruction.cpp/.h`, supplemented by PSP/DS binary investigations.
- `ped.rs`: animation progression and sprite layering from `port/src/gfx/pedsprites.cpp/.h` and `port/game/player.cpp`; PSP animation layout was measured locally.
- `vehicle.rs`, `actors.rs`, `props.rs`, `audio.rs`, `radio.rs`, `weather.rs`, `weapons.rs`, `police.rs`, `explosions.rs`, `stats.rs`: algorithms, structure descriptions and research informed by the corresponding ctw-native world/game modules. This attribution does not assert complete PSP equivalence.

`vault.rs` includes experimental reconstruction, not an identified complete PSP routine. No Android graphics decoder is used to decode the PSP world resources. However, four optional local lookup-table readers currently support Android-derived reference tables; all ten preserved local table files have that provenance. They are excluded from Git. See [STATUS.md](docs/STATUS.md).

## Local VCS-Rust work

`iso.rs` derives from the ISO9660 reader written for Benjamin's local VCS-Rust project, with additional checks. Windows/OpenGL initialization in `gpu.rs` is also adapted from that work. The original VCS project was not modified.

## Optional HD texture pack and research tools

The owner supplied `CW-Texture-Pack-master` locally. Its credits remain in the private prepared pack's `PACK-README.md`; no images from it are included in this source repository. See [TEXTURES-HD.md](docs/reference/TEXTURES-HD.md).

Local importer research reproduces PPSSPP QuickTexHash behaviour from `GPU/Common/TextureDecoder.cpp` and palette keys from `TextureCacheCommon.cpp/.h`. Copyright PPSSPP Project, GPL-2.0-or-later. These research tools are not linked into the runtime.

PSP executable research used [John-K/pspdecrypt](https://github.com/John-K/pspdecrypt), GPL-3.0, as a separate local tool, and Capstone 5.0.9 for disassembly. Neither the decrypted executable nor those tool binaries are included here. PPSSPP is an external comparison tool, not a runtime dependency.

## Cargo dependencies

The pinned dependencies retain their own licenses, including minifb (MIT/Apache-2.0), flate2 (MIT/Apache-2.0), serde/serde_json (MIT/Apache-2.0), sha2 (MIT/Apache-2.0) and transitive packages. `Cargo.lock` records versions; consult each package's distributed notices when preparing binary/source distributions.

## Game resources and marks

Grand Theft Auto, Chinatown Wars and their resources/marks remain with their respective rights holders. No ISO, ELF, game archive, extracted lookup table, texture, sound or video is part of the tracked repository. The independent project is not affiliated with Rockstar Games or Take-Two Interactive.

The earlier provenance document is preserved verbatim in `docs/archive/THIRD_PARTY-before-restructure.md` for historical traceability; this document corrects its outdated claim about the absence of Android reference tables.

## Presentation logo typefaces

The project wordmarks in `docs/assets/logo.svg` and `logo-ctw.svg` use fixed outlines from user-supplied **Pricedown Black** (Typodermic Fonts) and **Bruce Mikita Two** (Harold Lohner). Font software is not redistributed or covered by the source-code GPL declaration. The supplied Pricedown desktop terms permit fixed logo artwork. Benjamin confirmed on 2026-11-02 that he holds a license or authorization for this use of Bruce Mikita. This is an owner declaration, not an independent review of license documents; see [BRANDING.md](docs/BRANDING.md). The wordmarks are independent project branding, not official Rockstar artwork or endorsement.

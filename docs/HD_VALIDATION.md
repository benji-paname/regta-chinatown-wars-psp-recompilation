# HD texture replacement — validated gameplay milestone

**Updated 3 November 2026**  
**Validation source**: Benjamin — Manual Owner Playtest — 3 November 2026 (native C++20 Playtest)  
**Development status**: `v0.2.0-alpha` (Alpha / Beta / Stable roadmap)  
**Next graphics milestone**: `v0.3.0-alpha` (pending PSP lighting validation)

---

## 1. Official Gameplay Validation Confirmed by Benjamin

Benjamin personally executed and confirmed three dedicated in-game tests on the native C++20 Playtest engine (`outputs/ReGTA-CTW-Playtest/`):

| Test | Description | Result |
|---|---|---|
| **Mode toggling without crash** | In-game PC menu (F4) toggle: Original PSP → HD Upscaled → Original PSP during live gameplay | **PASS** |
| **Visible graphical enhancements** | HD improvements confirmed on the minimap, texts, and HUD elements | **PASS** |
| **Configuration persistence** | Setting (`hd_textures = 1`) preserved in `ctw.ini` after closing and restarting the game | **PASS** |

The HD replacement system is now officially validated and functional on the tested elements. This represents a major milestone for the native PC recompilation of GTA Chinatown Wars.

---

## 2. Documented Limits and Non-Claims

In accordance with strict evidence-first reporting:
- **No claim of 100% texture coverage**: It is not claimed that all textures in the game are improved.
- **No individual verification of all pack assets**: Pack contents are matched dynamically; not all elements have been cataloged individually.
- **Fallback behavior under test**: The behavior when the HD pack is absent remains to be verified separately (see isolated protocol below).
- **Unresolved graphical issues**: PSP lighting fidelity (streetlights, headlight halos), 60 FPS flickering, and some transition rendering behaviors are **not** resolved by this validation and remain active work in progress.

---

## 3. Official Roadmap and Versioning Status

The project follows an **Alpha / Beta / Stable** roadmap:
- **Current development version**: `v0.2.0-alpha`
- **Next major graphics milestone**: `v0.3.0-alpha`

### Status of Milestone v0.3.0-alpha:
- **HD Textures**: **VALIDATED** on tested gameplay features (minimap, HUD, texts, safehouse artwork, live toggle).
- **In-game PC Menu (Original / HD)**: **VALIDATED**.
- **Configuration Persistence (`ctw.ini`)**: **VALIDATED**.
- **PSP Lighting Fidelity**: **NOT VALIDATED** (Jalon 2E remains open and under investigation).
- **Other Graphical Issues (60 FPS flickering, transitions)**: **IN PROGRESS**.

> [!IMPORTANT]
> The project is **not** prematurely promoted to `v0.3.0-alpha`. The HD validation is recorded as a major successful feature, while the broader graphical milestone remains in development.

---

## 4. Preservation of the Functional HD Engine

The active Playtest executable in `outputs/ReGTA-CTW-Playtest/` is locked and protected:

| File | Path | SHA256 |
|---|---|---|
| **Active Validated Engine** | `outputs/ReGTA-CTW-Playtest/ctw_game.exe` | `6B48A3B29DD4DE510FD2A38E6DDD5BCB16BE2D1DAAF196781264DD96E5C1A7DA` |
| **Dedicated Secure Backup** | `outputs/ReGTA-CTW-Playtest/binary-backups/ctw_game_hd_validated_6b48a3b29dd4de51.exe` | `6B48A3B29DD4DE510FD2A38E6DDD5BCB16BE2D1DAAF196781264DD96E5C1A7DA` |
| **Pre-HD Engine Backup** | `outputs/ReGTA-CTW-Playtest/binary-backups/064f2143a47d36af2c41245ec3f6096cd04e600a4216441eb6ef670bbd403b28.exe` | `064F2143A47D36AF2C41245EC3F6096CD04E600A4216441EB6EF670BBD403B28` |

- `ctw.ini`, user saves (`ms0/`), and the private HD texture pack remain completely preserved.
- Rule: **Replacing or modifying `outputs/ReGTA-CTW-Playtest/ctw_game.exe` is strictly forbidden without explicit user authorization.**

---

## 5. Complementary Verification Protocol: Fallback without HD Pack

To verify automatic fallback to original PSP textures when the HD pack is absent or invalid, without disturbing the validated installation:
1. Create an isolated scratch directory (e.g. `work/cpp-recomp-feasibility/test-no-pack/`).
2. Copy `ctw_game.exe`, DLLs, and `ctw.ini` (configured with `hd_textures = 1`), with a symbolic link or relative path to the ISO.
3. Explicitly omit the `textures/` directory.
4. Execute in an isolated sandbox to confirm that `TextureReplacer::IsEnabled()` gracefully detects the absence of pack files, logs a clean notice, and renders using 100% original PSP textures without crash or missing texture artifacts.
5. **Strict constraint**: Never move, rename, or delete Benjamin's actual HD pack in `outputs/ReGTA-CTW-Playtest/textures/`.

---

## 6. Confidentiality and Privacy

The texture pack, screenshots, personal saves, logs, original ISO, and decompiled game code remain strictly private. No proprietary assets or binary downloads are distributed.

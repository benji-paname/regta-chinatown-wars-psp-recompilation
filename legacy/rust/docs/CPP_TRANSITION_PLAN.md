# Strategic Plan — Feasibility Study & Future Transition Toward C++17 Static PSP Recompilation (`docs/CPP_TRANSITION_PLAN.md`)

**Last Updated**: 2026-10-31  
**Current Active Port**: 100% Native Rust (`src/`, 45 modules, 248 passing tests, 0 warnings) — **Fully Preserved & Active**  
**Strategic Target**: Future transition toward a **C++17 static recompilation architecture** of the European PSP binary (`ULES-01347` v1.01) and its 166 overlays, modeled on [`highcmd/chinatown-wars-psp-recomp`](https://github.com/highcmd/chinatown-wars-psp-recomp).

> [!IMPORTANT]
> **EXECUTION STATUS — READ BEFORE ACTING**  
> This transition is **NOT YET AUTHORIZED FOR EXECUTION**.  
> - **Do NOT** start developing the C++ prototype unless explicitly instructed by Benjamin.  
> - **Do NOT** perform any automated Rust-to-C++ code translation.  
> - **Do NOT** alter, delete, or deprecate the existing Rust codebase (`src/`), its Git history, or its unit test suite.  
> - Any future C++ work must begin as a **strictly isolated prototype** validated against objective milestones before any architectural transition is decided.

---

## 1. Study of the `highcmd/chinatown-wars-psp-recomp` Architecture & the 166 Overlays

Our local mirror `references/chinatown-wars-psp-recomp` (pinned at commit `6562163b`, GPL-3.0-or-later) implements an Ahead-Of-Time (AOT) MIPS-to-C++17 static recompiler and PSP HLE runtime for the USA release (`ULUS-10490` v1.01):

### A. Static Recompiler Pipeline (`tools/`)
1. **Decryption & Extraction (`setup_from_iso.py`, `tools/prxdecrypt`)**:
   - Decrypts `PSP_GAME/SYSDIR/EBOOT.BIN` (via PPSSPP `PrxDecrypter` + `libkirk`) into an `ET_EXEC` ELF at base `0x08804000`.
   - Unpacks `OVERLAYS.PAK` (`20KP` container with 167 entries: entry 0 = name-hash table, entries 1..166 = `10SA` zlib-compressed overlay ELFs of type `0xFFA8`) into `extracted/overlays/NNN_<name>.elf`.
2. **Bidirectional Cross-Overlay Static Analysis (`psp_prx.py`, `analyze.py`)**:
   - Analyzes both the main executable and all 166 overlays simultaneously.
   - Harvests cross-references in both directions: overlay `jal`/pointer targets into the main executable (780 main-ELF functions are reachable *only* from overlays) and main-ELF calls into overlay slots (92 calls).
   - Scans inter-function gaps for prologue-less leaf stubs (`jr ra` vtable thunks) and resolves two compiler jump-table idioms (`lui/addu/lw/jr` and delay-slot `lui` + `addiu/addu/lw`: 203 jump tables in the main ELF, 137 across overlays).
   - Discovers **12,222 functions** in the main executable and **22,579 functions** across the 166 overlays.
3. **C++17 Code Generation & Signature-Based Overlay Dispatch (`recomp.py`)**:
   - Emits `F_<addr>` C++ functions for the main executable and one `recomp_ovl_NNN.cpp` file per overlay (`O<id>_<addr>`).
   - **Two Disjoint Memory Slots**:
     - **Slot 0** (`0x08b87460` in USA, up to 172 KB): **114 mission and game-mode overlays** (`bik_b01..04`, `hes_a01..c01b`, `jao_a01..07`, `ken_a01..b04`, `les_a01..b03`, `zho_a01..c01`, `xin_01..02`, `vigilante`, `firefighter`, `taxi`, `chinesedelivery`, `rampage`, `races`, etc.).
     - **Slot 1** (`0x08bb2480` in USA, up to 87 KB): **52 UI/PDA/minigame overlays** (`hudapp`, `pdaapp`, `ammozoneapp`, `emailapp`, `hotwiringapp`, `sinkingcarapp`, `safehouseapp`, `deatharrestapp`, etc.).
   - **Zero-Hook Overlay Dispatch**: Each overlay receives a unique signature of `(address, word)` code pairs. When `rt_call` targets an overlay slot address, it verifies the active overlay's signature in guest RAM in a few word comparisons and dispatches to the matching compiled `O<id>_<addr>` function without hooking the game's internal `overlayloader` thread.

### B. Runtime Layer (`runtime/`)
- **PSP Kernel / HLE**: Cooperative fiber-based thread scheduler (`sceKernelCreateThread`, `WaitThreadEnd` zero-timeout fix, `sceKernelExitDeleteThread` stack reclamation so the 256 KB heap margin for overlay loading is preserved, VBlank sub-interrupt handlers, `ms0:/PSP/SAVEDATA` probe support).
- **GE OpenGL 3.3 Renderer (`ge_gl.cpp`, `ge_render.cpp`)**: Executes authentic PSP Gu/GE display lists up to $2880 \times 1632$ with hardware stencil shadows (`gl_FragStencilRefARB`), alpha-as-stencil semantics, bloom, HUD anchor scaling (`F3`), and 60 FPS vertex-partner frame interpolation (`midTex`) over the fixed 30 Hz game logic.
- **Audio & Media**: ATRAC3+ ring-buffer radio/music streaming, SAS sound-effect mixer, and Bink video playback.

---

## 2. Differences Between PSP USA (`ULUS-10490` v1.01) and PSP Europe (`ULES-01347` v1.01)

While `highcmd` targets the North American release (`ULUS-10490` v1.01), our project targets the European multi-language release (`ULES-01347` v1.01):

| Aspect | PSP USA (`ULUS-10490` v1.01) | PSP Europe (`ULES-01347` v1.01) | Impact on Static Recompilation |
|---|---|---|---|
| **Overlay Archive Path** | `USRDIR/WAD/OVERLAY/CTW_PSP_UMD_MASTER_SCEA/OVERLAYS.PAK` | `USRDIR/WAD/OVERLAY/CTW_PSP_UMD_MASTER_SCEE/OVERLAYS.PAK` | Path (`SCEA` vs. `SCEE`) and overlay binary offsets must be parameterized per region. |
| **Overlay Count & Names** | 166 code overlays (114 Slot 0 + 52 Slot 1) | 166 code overlays (114 Slot 0 + 52 Slot 1; see `docs/PSP_OVERLAYS_CATALOG.md`) | Identical 166 logical overlays, but individual overlay sizes and relocation addresses differ. |
| **Main ELF Layout (`.text` / `.bss`)** | Base `0x08804000`; Slot 0 at `0x08b87460`, Slot 1 at `0x08bb2480` | Base `0x08804040`; `.text` `0x08804040..0x08b34ecf`, `.rodata` `0x08b3cf10..0x08b52687`, `.data` `0x08b526c0..0x08b874d3`, `.overlays` `0x08b8754c..0x08bc809f`, `.bss` `0x08bc8400..0x08da3cc7` | All hardcoded function addresses (`F_08xxxxxx`), global pointers, and overlay slot bases shift by `+0x100` to `+0x800` bytes between SCEA and SCEE. |
| **Localization & Strings** | American English GXT / fonts | Multi-language European GXT (EN, FR, DE, ES, IT) | String tables, UI buffer sizes, and localized text layout addresses differ. |
| **Hardcoded Patches / Hooks** | `data/cheats.json` & `game_hooks.cpp` use `ULUS-10490` addresses (e.g., camera `0x088559AC`, streaming `0x089F4754`, vblank `0x0890D53C`) | Equivalent routines reside at shifted addresses in `ULES-01347` | Every address in `cheats.json` and `game_hooks.cpp` must be mapped to `ULES-01347` using Ghidra (`CTW_Working.gpr`). |

---

## 3. Reusing `highcmd` Recompilation Tools & Runtime Environment

Because `highcmd/chinatown-wars-psp-recomp` is licensed under **GPL-3.0-or-later** (compatible with our GPL-3.0-only repository, with proper attribution in `THIRD_PARTY.md`), its toolchain and runtime can be adapted for `ULES-01347`:
1. **Direct Reuse Candidates (Region-Agnostic)**:
   - `tools/prxdecrypt` (EBOOT.BIN decryption), `psp_prx.py`, `analyze.py`, and `recomp.py` (which operate generically on Allegrex instructions, relocations, jump tables, and overlay headers).
   - `runtime/` (PSP HLE syscalls, fiber scheduler, `ge_gl.cpp` OpenGL 3.3 GE renderer, VFPU/SAS modules derived from PPSSPP, ATRAC3+ streaming, savestates).
2. **Adaptation Required for `ULES-01347`**:
   - Update `setup_from_iso.py` to accept the European ISO SHA-1 / `ULES-01347` `PARAM.SFO` and the `CTW_PSP_UMD_MASTER_SCEE/OVERLAYS.PAK` path.
   - Extract the exact `ULES-01347` `.overlays` slot start/end addresses from `CTW-PSP.elf`.
   - Port all address-specific hooks in `game_hooks.cpp` and `data/cheats.json` from `ULUS-10490` to `ULES-01347` offsets.

---

## 4. Role of Ghidra 12.2, `ghidra-allegrex` v21.5, `CtwQuery.java`, and PPSSPP

Our reverse-engineering stack provides the exact link between `ULUS-10490` and `ULES-01347`:
- **Ghidra 12.2 + `ghidra-allegrex` v21.5 + `CTW_Working.gpr` + `CtwQuery.java`**:
  - Used to match every `ULUS-10490` function referenced in `highcmd`'s `NOTES.md` and `game_hooks.cpp` (e.g., `F_088559AC` view-matrix builder, `F_089F4754` streaming manager update, `F_0890D53C` frame flip loop) to its exact `ULES-01347` counterpart by comparing instruction patterns, constants, and call graphs.
  - Validates jump tables, unreferenced vtable leaf stubs, and VFPU instructions (`@vfpu_scan`) in `CTW-PSP.elf`.
- **PPSSPP (Interactive & Headless Comparison)**:
  - Serves as the behavioural and visual reference oracle for `ULES-01347` (`PPSSPPHeadless` with deterministic input scripts and frame dumps).

---

## 5. Existing Rust Assets & Knowledge Reusable in the Future C++ Architecture

None of the engineering invested in our native Rust port (`src/`) is wasted:
1. **Verified Binary Format Documentation & Parsers**:
   - Complete specifications and validation tests for `WAD.TOC`, `GAME.PAK`, `OVERLAYS.PAK` (`docs/PSP_OVERLAYS_CATALOG.md`), `timecycle.dat` (42,240-byte 55-channel SCEE layout), `world.bin`, `ai.bin`, and `PAK 29` handling records.
2. **Reverse-Engineered `ULES-01347` Addresses & Mechanics**:
   - Audited SCEE function addresses and struct offsets already proven in Rust and Unicorn:
     - Player armour & Double Armour (`0x089c967c`, `0x089c7c50`, `0x089c7d08`, `0x089c8f20`, `0x089c8930`, `0x089c8ff4`, `0x08897e78`).
     - Rigid-body kinematics & Q12 damping (`0x089b5960`, `0x089bf620`, `0x088a0078`) and the 512-case Unicorn oracle (`tools/research/psp_prop_oracle.py`).
     - Driving camera & PSP cardinal-axis steering assist (`FillInDrivingYoke` at `0x089cb264..0x089cb720`, `docs/reference/DRIVING-CAMERAS.md`).
     - Radar geometry and HUD sprite directories (`docs/reference/RADAR-PSP.md`, `docs/reference/PSP-VISUAL-PARITY.md`).
3. **Custom Enhancements & Tooling**:
   - Multi-profile camera formulas (PSP / DS / Mobile), HD texture replacement rules (`docs/reference/TEXTURES-HD.md`), automated driving/route audit harnesses, and project branding/launchers.

---

## 6. Future Creation of an Isolated C++ Prototype

When Benjamin explicitly authorizes starting the prototype:
1. **Strict Isolation**:
   - The prototype will be created in a dedicated, isolated directory or branch (e.g., `work/cpp-recomp-prototype/` or a separate experimental target) **without modifying `src/`, `Cargo.toml`, `Compiler.ps1`, or `outputs/CTW-Rust/CTW-Rust.exe`**.
2. **No Automated Rust $\rightarrow$ C++ Translation**:
   - The C++ prototype will be generated by running the static MIPS-to-C++17 recompiler (`analyze.py` + `recomp.py`) on `ULES-01347` (`CTW-PSP.elf` + 166 `SCEE` overlays), **never** by transpiling the clean-room Rust code into C++.
3. **Full Reversibility**:
   - The Rust port remains intact, compilable, and testable at all times until the C++ prototype passes all acceptance criteria and Benjamin formally approves a transition.

---

## 7. Prototype Acceptance Criteria (Go / No-Go Gates)

Before considering any architectural switch from the Rust port to the C++ static recompilation, the isolated C++ `ULES-01347` prototype must demonstrate:
1. **Boot & Initialization**: Clean decryption/loading of `ULES-01347`, kernel thread startup, memory stick (`ms0:/PSP/SAVEDATA`) detection, legal screen, Bink intro movie playback, and language selection (FR/EN).
2. **Introduction Sequence**: Flawless execution of the opening comic cutscenes with synchronized audio.
3. **World Rendering**: Full 3D Liberty City sector streaming, authentic PSP GE lighting, stencil shadows, bloom, and HUD/minimap overlay (`hudapp`) at native 1080p/1440p internal resolution without graphical corruption or heap exhaustion.
4. **Core Gameplay**: Responsive on-foot locomotion, vehicle entry/driving, physics, collisions, traffic, pedestrian spawning, and ATRAC3+ radio/SAS sound effects at stable 30 Hz logic (and 60 FPS interpolated presentation).
5. **First Story Mission Completion**: End-to-end playability of the opening mission (*Yu Jian* — airport arrival, triad ambush cutscene, `sinkingcarapp` underwater windshield escape minigame, swimming to shore, driving to Kenny's restaurant, and safehouse sequence).

---

## 8. Fidelity & Performance Comparison Matrix (PPSSPP vs. Rust Port vs. C++ Prototype)

The prototype will be evaluated against both **PPSSPP (`ULES-01347`)** and our **Native Rust Port (`CTW-Rust.exe`)** across five axes:

| Evaluation Axis | PPSSPP (Reference Emulator) | Current Native Rust Port (`CTW-Rust.exe`) | Target C++17 Static Recomp Prototype (`ULES-01347`) |
|---|---|---|---|
| **CPU Execution** | JIT / Interpreter (emulated MIPS/VFPU) | 100% Native x86_64 Rust (clean-room reimplementation) | 100% Native x86_64 C++17 (AOT recompiled from original MIPS + overlays) |
| **Gameplay & Script Fidelity** | 100% (executes original binary + 166 overlays) | Partial (verified core mechanics + clean-room state machines; overlays not executed) | ~100% (executes all 12,222 main functions + 22,579 overlay functions natively) |
| **Visual & Shader Fidelity** | 100% GE emulation | Partial (PSP models/textures/timecycle + first-pass planar shadows; GE material/bloom incomplete) | High (executes original GE display lists in OpenGL 3.3 with hardware stencil shadows & bloom) |
| **Audio Fidelity** | 100% ATRAC3+ & SAS | Synthesized/approximated streams & effects | 100% authentic ATRAC3+ streaming & SAS mixer |
| **PC Customization & Moddability** | Limited to cheat/memory hooks | High (clean modular Rust structs, custom camera profiles, native HD textures) | High via `game_hooks.cpp`, native C++ overrides, 60 FPS interpolation & HUD scaling |

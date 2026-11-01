# Building the C++ mainline locally

The canonical source is `runtime/`, not the highcmd mirror or the historical isolated copy. All six validated Windows/scheduler adaptations are recorded in `runtime/provenance.json`. The existing Playtest uses the integrated FIFO/Bink build, not the old diagnostic executable.

## Existing prepared workspace

Requirements already used locally: Windows x64, Python, Visual Studio Build Tools/MSVC headers and libraries, Clang 22.1.3 + lld-link, SDL2 2.30.9, Khronos GL headers and the existing FFmpeg development tree. No dependency is downloaded or installed by these commands.

```powershell
.\Compiler.ps1 -Check
.\Compiler.ps1 -Test
# Only if recompiling the private generated code is necessary:
.\Compiler.ps1 -Test -FullGame
```

The default build compiles all 29 canonical runtime components, reuses the hash-checked 250 generated objects and links into a new dated `work/cpp-mainline/build-*/` directory. `-Test` compiles and executes the real-runtime 35-check fixture. Neither mode starts the game nor updates the Playtest.

The ignored `work/cpp-mainline/build-config.json` records real include/library paths, baseline hashes and per-unit flags. Repository paths use `${ROOT}` and are resolved at invocation; external dependency paths remain machine-specific. Shared-header changes stop the build until the generated-code ABI and recipe are deliberately revalidated. The existing private generator work remains under `work/cpp-recomp-feasibility/`; do not rerun old integration/boot scripts that can overwrite test directories.

Flags preserve C++20, `x86_64-pc-windows-msvc`, `-O1`, `-fno-strict-aliasing`, `-fwrapv`; no fast-math. `_USE_MATH_DEFINES` is scoped to VFPU glue; `CTW_PORTABLE=1` is scoped to main. Do not erase or hide compiler warnings. Complete diagnostics, commands, COFF directives and result JSON are saved per build.

## Public clone limitation

Runtime source is public; the generated game translation units, private build recipe, image, ISO, runtime assets and DLL package are not. The complete lawful ISO-to-build preparation process is not yet packaged for a fresh public clone. Do not advertise these instructions as a public binary release or a self-contained fresh-clone build.

## Playtest update

`python tools/update_playtest.py --check` validates paths without replacing files. An explicit update preserves `ms0/`, `ctw.ini`, all game resources and a backup of the previous game binary. See [PLAYTEST.md](PLAYTEST.md). Do not distribute binaries before their content/dependency licensing has been reviewed.

## Historical Rust

The root Cargo workspace remains for compatibility; its manifest points to `legacy/rust/src/main.rs`. To build that historical port explicitly, use `tools/build/Compile.ps1 -Test` or `outputs/CTW-Rust/Compiler.ps1 -Test`. Its metrics generator writes only `docs/legacy/`. Root `Compiler.ps1` means C++.

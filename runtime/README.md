# Canonical C++20 runtime

These63 C++/header files are copied from highcmd commit6562163b6f504a1db93e1be5ce580d8cd6f024a7, with six already-validated local adaptations taking precedence. See `provenance.json` for per-file before/after SHA256. The highcmd reference directory is unchanged.

29 translation units cover kernel/HLE, Windows fibers, GE/OpenGL, audio/FFmpeg, VFPU and SAS. Original highcmd GPL-3.0-or-later and PPSSPP GPL-2.0-or-later notices are retained; see root THIRD_PARTY.md. This does not license original game assets.

Game-generated code, images, ISO, runtime data tables and packaged dependencies remain private under work/ and outputs/. Build through root Compiler.ps1, not old experiment scripts. Do not enable unverified USA hooks/cheats. Bink and FIFO/priority corrections are part of kernel_thread.cpp and the35-check fixture.

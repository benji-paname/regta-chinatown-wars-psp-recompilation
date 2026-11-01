# Tooling

C++ mainline: root Compiler.ps1 delegates to build/Cpp.cmd and build/cpp.py. Test fixture: tests/kernel/mutex_scheduler.cpp. Playtest source: playtest/launcher.cpp; explicit updater: update_playtest.py. No build starts a game or replaces user saves.

maintenance/check_upstreams.py is mandatory before each task. maintenance/check_publication.py audits tracked trees/outgoing blobs before publication. progress/cpp_status.py emits evidence-based C++ status. progress/progress.py is historical Rust-only and cannot rewrite mainline documentation.

build/Compile.ps1 remains the explicit legacy Rust builder. Historical/private experimental scripts are preserved in work/ but must not be blindly rerun.

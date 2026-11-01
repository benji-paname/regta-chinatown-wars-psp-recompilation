# ReGTA CTW — C++ mainline assistant guidelines

Read AGENTS.md and docs/AI_HANDOFF.md first. The active implementation is native C++20 in runtime/. Rust is historical under legacy/rust/src/.

Before each task, run tools/maintenance/check_upstreams.py and review BOTH upstreams. Never merge automatically or copy USA addresses to PSP Europe without proof.

Run the C++ evidence generator tools/progress/cpp_status.py after verified advances. Rust's progress.py writes only docs/legacy/. Never present Rust percentages as C++ game completion.

Keep all game files, saves and test captures private and intact. Audit the Git tree and outgoing history with tools/maintenance/check_publication.py before each authorized push. A .gitignore is insufficient for tracked files.

Build instructions: docs/BUILD_CPP.md. The private recipe and generated game code are intentionally excluded. Preserve diagnostics; C++ has inherited warnings. 35 passing kernel checks are limited evidence, not complete PSP fidelity.

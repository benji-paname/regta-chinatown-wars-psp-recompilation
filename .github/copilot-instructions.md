# ReGTA CTW — C++ mainline assistant guidelines

Read AGENTS.md and docs/AI_HANDOFF.md first. The active implementation is native C++20 in runtime/. Rust is historical under legacy/rust/src/.

Before each task, run tools/maintenance/check_upstreams.py and review BOTH upstreams. Never merge automatically or copy USA addresses to PSP Europe without proof.

Run the C++ evidence generator tools/progress/cpp_status.py after verified advances. Rust's progress.py writes only docs/legacy/. Never present Rust percentages as C++ game completion.

Keep all game files, saves and test captures private and intact. Audit the Git tree and outgoing history with tools/maintenance/check_publication.py before each authorized push. A .gitignore is insufficient for tracked files.

Preserve the repository-local author identity for Benjamin's authorized work: Benji Paname <324261208+benji-paname@users.noreply.github.com>. Verify GIT_AUTHOR_IDENT before committing; never substitute a dummy local address or his personal email. Other contributors retain their own identities. Do not rewrite published history to alter attribution.

Build instructions: docs/BUILD_CPP.md. The private recipe and generated game code are intentionally excluded. Preserve diagnostics; C++ has inherited warnings. 35 passing kernel checks are limited evidence, not complete PSP fidelity.


Permanent Rule — GitHub and Official Site Updates:
Every major, validated milestone must be documented on GitHub (README, roadmap, changelog) and the official site (EN, FR, ES in docs/index.html).
Do not publish features as operational unless verified in gameplay (not unit tests alone).
Public docs must reflect Benjamin's validated tests: 5 main missions and taxi activity completed, PSP fidelity, working saves, while performance/audio/rendering fixes are in progress.
Never publish ISO, commercial assets, personal saves, private textures, or local credentials/paths.
For Jalon 2C: commit and push remain strictly forbidden until the unified engine has been validated in game.

# ReGTA CTW — C++ mainline handoff

Updated 2026-11-02. Read `AGENTS.md` and `CPP_MAINLINE_MIGRATION.md` before work.

**C++20 in runtime/ is canonical.** Benjamin reports completing two story missions with normal controls. Do not return to Rust gameplay reconstruction. Performance drops toward 20 FPS/frametime instability in dense driving scenes are the next priority, followed by progressive campaign validation.

The current copied source includes the integrated Bink owner-exit cleanup and FIFO/priority fixes for mutexes and mailboxes. 29 runtime units compile/link; 35 real-runtime kernel checks pass in this migration. The 250 generated units and 166 overlays remain private and unchanged. Warnings remain; do not claim zero warnings or complete PSP equivalence. No game assets or binaries are approved for distribution.

The known-working private Playtest remains `outputs/ReGTA-CTW-Playtest/ReGTA-CTW-Playtest.exe`, game binary hash `bfd36700b9da0d5c0affbff5870bdc8d9adbb19e1b280ea170b28ed941848d4c`. Canonical rebuilds go into `work/cpp-mainline/` and do not install themselves. Preserve ms0, ctw.ini, private assets and historical evidence. Current mainline source is not the old bink-fix-validation experiment.

Rust is preserved under `legacy/rust/src/`, with a local archive branch and a private checkpoint ZIP/bundle under `work/migration-20261102/`. The root rename to ReGTA-CTW-PC-Recomp was attempted and blocked by Windows because the directory is in use. It was not forced. Current working directory remains the old name; all new build tools resolve the root relative to their own paths. Do not duplicate/delete the project to evade the lock.

No gameplay captures may be published. Branding license/authorization was explicitly confirmed by Benjamin. Review tracked files and outgoing Git blobs before every push. Consult the migration report for actual commit/deployment state; never infer that publication succeeded from local edits.

Read the final migration report for execution budgets and remaining work. Earlier reports claiming complete fidelity are historical; 35 covered tests are not full hardware/campaign validation. Upstream remote revisions were checked through the GitHub connector after the local Git preflight DNS failure; review is documented separately.

Migration verification: 10,686 protected baseline files unchanged; all 51 relocated Rust files byte-identical; 10 Python tooling tests pass. One isolated launcher smoke used the new build for 21.64 seconds, read the copied save (4,152 bytes), loaded HUD/environment/audio and exited with code 0. No interactive controls were tested in this smoke. 75 runtime warnings + 5 fixture warnings retained. Original Playtest unchanged. Publication outcome is recorded in CPP_MAINLINE_MIGRATION.md.

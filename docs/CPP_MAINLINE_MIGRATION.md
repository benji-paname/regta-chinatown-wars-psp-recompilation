# C++ mainline migration — 2 November 2026

**C++ MAINLINE MIGRATION — PARTIAL.** Canonical source migration, local compilation and bounded launch verification have succeeded. The local directory rename is blocked by a Windows lock. Publication/deployment verification is recorded below; do not infer completion from local files.

## Decision and observed scope

Benjamin chose the existing native C++20 PSP Europe ULES-01347 v1.01 recompilation as the mainline after personally completing the first two missions with normal controls. That owner report covers the tested cinematics/dialogue, walking/swimming, driving/traffic, police/NPCs, mini-games, HUD/map and controller. This migration did not replay those missions or establish full-game PSP fidelity. Drops toward 20 FPS and unstable frametimes in dense driving remain known limitations.

## Directory and preservation

- Actual root remains `C:\Users\BENJAMIN\Documents\Codex\2026-10-17\tu-voit-le-projet-vcs-rust`.
- Requested root: `C:\Users\BENJAMIN\Documents\Codex\2026-10-17\ReGTA-CTW-PC-Recomp`.
- An in-place rename was attempted after collision/process checks. Windows refused because the folder is in use. No forced retry, deletion, duplicate migration or permission bypass was used. Verification after the requested move cannot be claimed. New tools use root-relative paths; a later approved rename needs all editors/terminals closed and another build/launcher check.
- A private pre-migration ZIP, complete Git bundle, original patch, SHA256 manifest and status are retained in `work/migration-20261102/`.
- Baseline: 11,374 files hashed. After migration: no missing baseline files; **10,686 protected files unchanged**, including original Playtest, reference repositories and private research/game data. **51 relocated Rust files are byte-identical**. Changed documentation/tooling are listed in the private comparison report.
- No ISO, ELF, extracted resource, music, video, save or original game file was deleted. No dependency was installed. Rust executable and original Playtest package were not replaced.

## Source organization and provenance

`runtime/src`, `runtime/vfpu`, `runtime/sas`: 63 source/header files, based on highcmd revision `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Six previously validated local adaptations are retained: kernel_thread, kernel_misc, hle_display, hle_audio, gl_funcs and ge_gl. Per-file original/canonical SHA256 and adaptation flags are in `runtime/provenance.json`. The integrated Bink owner-exit cleanup and mutex/mailbox FIFO/priority work are preserved. No synchronization behavior was newly changed in this migration.

The 250 generated game units, 166 overlays, objects, original data, DLL packages and local recipe remain private. `tools/build/cpp.py`, `Cpp.cmd` and root `Compiler.ps1` build the canonical source. `tests/kernel/mutex_scheduler.cpp` exercises the real runtime. `tools/playtest/launcher.cpp` preserves launcher source. `legacy/rust/src` preserves Rust; its Cargo entry point was redirected and historical builder retained. The full ISO-to-build process is not yet packaged for a fresh public clone.

Inherited highcmd GPL-3.0-or-later and PPSSPP GPL-2.0-or-later notices remain. Authored project contributions are GPL-3.0-only. Game content is excluded. No claim of binary-package redistribution clearance is made; local SDL2/FFmpeg/DLL packages stay private. See THIRD_PARTY.md. Benjamin confirms authorization for the logo font artwork; font software is not distributed. **No test capture is authorized for publication.**

## Tests actually performed

| Check | Result and limit |
| --- | --- |
| `tools/build/Cpp.cmd --runtime --test` | 29 runtime units recompiled; 250 original generated objects reused after hash checks; game and test links successful. 75 compiler warnings for runtime and 5 for the fixture; diagnostics preserved. No fast-math; Clang 22.1.3, C++20, Microsoft x64 ABI. |
| Real-runtime kernel fixture | **35 checks, 0 failures, 0 known differences in the covered checks**. Not a complete PSP kernel/campaign certificate. |
| COFF inspection | New runtime objects inspected using llvm-readobj file headers/directives; AMD64 and MSVC-compatible settings retained. |
| Root `Compiler.ps1 -Check` | 279 source/object paths, original object hashes and shared headers checked successfully. |
| Python tools | **10 tests pass**: private-publication filter and safe Playtest updater, including no-write check mode, backup, invalid executable rejection and save/preferences preservation. |
| `tools/update_playtest.py --check` | New build hash verified; original installed game binary left unchanged. An explicit update makes a hash-verified backup, atomically replaces only the executable and writes a separate receipt. |
| Isolated launcher smoke | 21.64 seconds, launcher and game return 0. New canonical binary ran in a copy under `work/Playtest-migration-smoke/`; original package untouched. ISO resolved via relative configuration, DLL/GL/audio loaded, copied 4,152-byte savedata loaded, HUD/environment/safehouse overlays reached. No interactive keyboard/controller validation in this smoke. Logs stay private. |
| Site static checks | 60 complete EN/FR/ES translation sets, English default, local links/anchors and approved media checked. No gameplay media embedded. |
| Browser/local preview | In-app browser timed out on localhost; native Edge observation failed because its window was minimized. No visual browser PASS claimed for that attempt. Remote verification follows publication. |
| Rust | No Rust compilation rerun. Sources preserved by SHA256; old test totals are historical. |

New canonical executable SHA256: `4e862c05605c28e7fa76cf988b6c393dd23b9fa956a06099d7b8b198ea99353c`.
Unchanged installed Playtest SHA256: `bfd36700b9da0d5c0affbff5870bdc8d9adbb19e1b280ea170b28ed941848d4c`.
Unchanged ISO SHA256: `8c62ef8d7263ff0d42db40901927de4e86afd02f3d3450c3a96d0e2bc8d5168e`.

Detailed commands, diagnostics, objects and test logs: private `work/cpp-mainline/build-20261102-000859/`; original evidence remains under `work/cpp-recomp-feasibility/`.

## Git, website and publication

Pre-migration main was `52cbb612607fc2106634d8cee2e115554f66542c`. A separate-index checkpoint created **local** archive branch `legacy/rust-pre-cpp-migration` at `c9743b40845875b57bca8dbaf59098835e9d6874`, preserving uncommitted tracked work. Untracked work is backed up privately. The archive branch is not pushed wholesale because its tree contains an old gameplay thumbnail. No history rewrite was performed.

The formerly tracked `docs/assets/thumbnail.png` is removed from tracking only, retained locally and excluded. Its existing remote history is not erased by this migration; a separate approved history-removal plan would be required. No new capture may be published.

README, website EN/FR/ES, build/status/playtest/AI instructions, contribution/license documents and changelog now describe C++ as canonical, the two-mission owner report, limited kernel evidence and performance work. Rust metrics are historical under `docs/legacy`; its generator cannot overwrite the C++ README/site. The C++ status generator uses actual validation evidence without a completion percentage. Support and repository URLs remain unchanged.

The publication guard checks the complete proposed tree and all new reachable Git blobs, file magic, private paths, secrets and exact approved logo hashes. The pre-push hook and public CI check reinforce this; manual content/license review remains necessary. This is not a legal guarantee.

**Publication status: pending final tree/history audit, authorized commit/push and Pages verification.**

Official [repository](https://github.com/benji-paname/regta-chinatown-wars-pc-port) and [website](https://benji-paname.github.io/regta-chinatown-wars-pc-port/) are unchanged.

## Upstreams and remaining work

The mandatory Git preflight failed DNS. A separate connected GitHub read verified highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7` and Android `108320b137c3ca748d697838f86c8919c12b3f2c`, matching local mirrors. Relevant recent changes were reviewed and recorded in UPSTREAM_REVIEW.md. No automatic upstream merge or guessed USA-to-Europe address mapping occurred.

1. Finish verified publication; retain PARTIAL while the root rename is blocked.
2. Profile frame times in dense driving scenes before changing scheduler/rendering behavior.
3. Validate the campaign incrementally with normal user input.
4. Package lawful local generation/build preparation for future public use.
5. Explore optional mouse mini-games later; not implemented or advertised as available.

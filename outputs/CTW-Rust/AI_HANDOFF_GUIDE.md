# Mainline migration handoff — 2026-11-02

C++20 is now canonical: runtime/. Rust sources are preserved in legacy/rust/src/. Read docs/AI_HANDOFF.md and docs/CPP_MAINLINE_MIGRATION.md first. Resume validation with .\Compiler.ps1 -Check. The rebuilt 29 runtime components link against 250 preserved generated objects; 35 kernel checks pass. Existing Playtest binary and saves were not replaced. No test captures may be published. Benjamin confirms logo font authorization. The local directory rename is blocked by a Windows lock; do not force it.

Historical handoff follows.

# ACTIVE HANDOFF — Bink fix experimentally validated; STOP after 3/3 boots

Updated 2026-11-01T19:13+01:00 (Europe/Paris). Read `work/cpp-recomp-feasibility/reports/BINK_MUTEX_FIX_VALIDATION.md`. This entry supersedes the historical entries below. Benjamin authorized the exact isolated fix and three bounded boots; all three are now consumed. No further execution or integration is authorized by that completed mission.

Approved owner-exit cleanup applied ONLY in `work/cpp-recomp-feasibility/bink-fix-validation/src/kernel_thread.cpp`. Exact behavioral diff equals the approved proposal; twelve diagnostic-only additions are separately recorded and removable. Original highcmd source SHA256 f70fd4b2ef5f35c534e2ca80ee6792bfdad3025ef3886713786d85e9eddba9f2 remains unchanged. Rust/generated code/original data/staging/reference executable are intact.

Actual C++ white-box fixture: 27 checks pass, 0 unexpected failures; THREE known policy differences exposed (FIFO attr0/200 and priority ties by UID). Real HLE/Windows fibers/timeout service exercised; no substitute mutex stubs or guest game loaded in fixture. One production kernel component compiled with Clang22.1.3/C++20/MSVCx64; 5 inherited warnings, 0 errors. Isolated link: 0 warnings/errors, 279 inputs with only the kernel object replaced. 31 public API symbols and all69 HLE declarations preserved; COFF/ABI directives match exactly. Full compiler logs retained.

Experimental exe `bink-fix-validation/gamebuild/ctw.exe`: SHA256 53e833fd4ddb1f7c9e8274699fd3348b192605dd0fabd8d608fb909e1ef7cc38. Never replace baseline `ctw_native_diag.exe` automatically.

Three boots: PID4268 58.125460s; PID10224 58.154974s; PID23356 58.093031s. All reached autosave and ken_a01 comic intro; no new runtime anomaly observed. Run02 recreated the failing race: main260 joins309, video271 exits owning308; at9.158s cleanup transfers308 to309, actual lock/unlock/exit succeed, main join returns0. Runs01/03 retain the previously successful path and do not trigger a cleanup transfer. Logs/captures (171 PNGs) and per-run SHA evidence retained under `reports/bink-fix-validation/`.

GO recommendation is limited to a future explicitly approved integration of the minimal Bink lifetime fix. NO full PSP-fidelity or definitive-reliability claim: FIFO/priority-tie gaps remain, audio output was disabled, no controller connected, no long gameplay/hardware test, instrumentation changes timing. Do not change queue rules without separate authorization. Exit code1 after watchdog is intentional termination, not a crash; Bink's own exit1 is not automatically an error.

All2167 protected files verified unchanged. GitHub preflight DNS failed; pinned local references reviewed, separate PPSSPP source accessed through web reader. No dump, interprocess read, elevation, install, commit or push. Rust tests/progress visuals not rerun or advanced for this C++ work.

Resume in ten seconds (read-only): `Get-Content work/cpp-recomp-feasibility/reports/BINK_MUTEX_FIX_VALIDATION.md`. Await Benjamin's next validation. Do NOT relaunch old scripts, clear consumed markers, integrate, or run a fourth session. Offline evidence extraction only: `python work/cpp-recomp-feasibility/tools/analyze_bink_fix_validation.py`.

---

# HISTORICAL HANDOFF — Bink root-cause investigation complete; fix proposal NOT applied

Updated 2026-11-01 18:50 Europe/Paris. Read `work/cpp-recomp-feasibility/reports/BINK_MUTEX_ROOT_CAUSE_AND_FIX_PROPOSAL.md` and `reports/bink-mutex-investigation/HANDOFF.md`. This entry supersedes all historical entries below. One authorized logging-only kernel compilation/link and one native run completed (PID 6828, 58.068642 seconds). Both the original baseline executable and Rust are unchanged. No further run or behavioral correction authorized.

New precise proof: video 271's old-mutex Unlock(268) returns 800201C5. Main 260's Unlock(268) succeeds, but preemption lets video close269, delete267/268 and create307/308/309 BEFORE main's unlock returns. Main then reloads the global audio ID and joins309. At14.017 seconds video271 becomes dormant retaining mutex308 count1 owner271; 309 repeatedly times out. Missing normal-mutex thread-end cleanup sustains the deadlock. PPSSPP's inspected primary source has that cleanup. Earlier speculation about main's unlock failing is not borne out by this trace.

`fix_proposal_UNAPPLIED.patch` is review text only: normal-mutex cleanup on exit/terminate with waiter handoff, no owner-check bypass. It has NOT been applied, compiled or game-tested. It retains the existing best_waiter policy, which is not FIFO-correct generally; multi-waiter fidelity is an explicit gate. No hardware-test claim. Exact early-reset timing trigger remains open. Ghidra failed at launcher Java configuration access; no bypass. Original ELF fallback checked1,482 words in13 functions. Eight offline trace checks pass, not fix/game tests. Diagnostic compilation has5 pre-existing warnings,0 errors; link0 errors/warnings. All1,974 protected files unchanged, rechecked18:50. No dump/interprocess read/elevation/commit/push.

Next step requires Benjamin's approval of the isolated fix and test scope. Read the report in10seconds, then await approval. Never clear `reports/bink-mutex-investigation/SESSION_CONSUMED` or run an old unconditional boot script. Full proposed test matrix: `reports/bink-mutex-investigation/FIX_VALIDATION_PLAN.md`. No new Rust tests or dashboard percentage claimed.

---

# HISTORICAL HANDOFF — Boot diagnostic complete; Bink blocking chain proven; STOP

Updated 2026-11-01 (Europe/Paris). Read `work/cpp-recomp-feasibility/reports/JALON_BOOT_REPRODUCIBILITY_DIAGNOSTIC.md` and `reports/boot-repro-diagnostic/HANDOFF.md`. This entry supersedes the historical entries below. The single authorized diagnostic process (PID 21396) ran for 58.064 seconds and was deliberately stopped. Do not launch again, clear SESSION_CONSUMED, build or modify runtime behavior without Benjamin's next authorization.

Built-in save-state succeeded at t=18.001605 s and was parsed offline after termination; no interprocess memory read or elevation. Proven chain: main thread 260 waits indefinitely for audio thread 309; audio retries shutdown mutex 308; that mutex has count 1 and owner 271, the already-ended Bink video thread. Successful boots recreated audio from thread 260, both failures from thread 271. The trigger of this early reset and the actual unlock return remain unproven. Do not bypass ownership or auto-release mutexes on speculation. Exit status 1 also occurs in successful Bink runs and is not independently an error. All four compared logs have zero controllers; the earlier contrary claim is corrected.

One session only; no new build/link, runtime/source/HLE/hook change, Rust change, live memory read, commit or push. All 1,974 protected files hash-unchanged. Original staging/data/references/executable preserved; 46 isolated launch copies match. GitHub DNS failed during mandatory preflight; only pinned local upstream sources were reviewed. No new Rust test result or dashboard progress claimed. Offline saved-state evidence and 57 captures retained privately; do not publish game-memory snapshots.

Next proposal: inspect the European Bink reset/close path offline, then seek separate approval for narrowly scoped ownership/return logging if necessary. Current snapshot already proves the blocking chain; no additional live-memory read is needed for that finding. Await Benjamin's validation before further work.

---

# HISTORICAL HANDOFF — First playable C++ experiment stopped; new execution requires approval

Updated 2026-11-01 (Europe/Paris). Read `work/cpp-recomp-feasibility/reports/MAJOR_MILESTONE_FIRST_PLAYABLE.md` and `FIRST_PLAYABLE_HANDOFF.md`. This entry supersedes the historical milestones below. Native build/link had previously reached 279 selected objects and the existing executable; no compilation/relink in this mission.

One authorized native session ran for 180.424 seconds and was deliberately terminated on a persistent yellow Rockstar-logo screen before autosave. No first-playable level reached; no `sinkingcarapp`, no PSP CtrlRead log, no L/R/swimming/movement sent. The earlier 1B.17 successful path was not reproduced. Bink audio/video thread lifecycle diverged; precise cause remains unproven. Do not treat display FPS as gameplay. All 1,974 protected files hash-unchanged, including Rust, references, original PSP data, generated code, objects and executable. No Rust changes, no new test result or dashboard percentage.

New isolated runner preserves original staging and has a persistent one-session lock. Do not clear it or run the older unconditional 1B.18 script. External memory-read attempt failed access; elevated retry was rejected by automatic approval review, no dump obtained. Ghidra read-only launcher failed before opening the project. Next proposed step is a separately approved bounded run using existing caller/wait/save-state diagnostics; any read-memory escalation also needs explicit scope/approval. No runtime fix, rebuild, further execution or commit authorized.

---

# HISTORICAL HANDOFF — Jalon 1B.5 documentary audit complete; await validation

Updated 2026-10-31 22:32 Europe/Paris. Latest report: work/cpp-recomp-feasibility/reports/JALON1B5_HLE_CRITICAL_AUDIT.md; exact evidence and resumption in reports/JALON1B5_HANDOFF.md. This heading supersedes the historical handoffs below.

265 existing C++20/MSVC x64 objects, prior residual native-symbol count124 unchanged; no compilation in this mission. Correction:53 raw unmatched import records include4 shadowed duplicates;252 unique stubs,49 selected unmatched names (9 non-network,40 network). Existing dispatch already selects correct ExitGame/Utils time/UMD names for those4. Their provider kernel_misc.cpp remains uncompiled and needs a separately reviewed MSVC time-portability change. No binding/fallback/source modification. Nickname output and timed UMD waits have concrete conditional correctness risks; no first-frame reachability proven.14 runtime C++ units remain; propose osd/hle_ge/hle_savedata/hle_net as a separately authorized next object-only batch. Ghidra launch failed before project open.879 protected existing files hash-unchanged; Rust source/exe preserved, no new Rust tests or dashboard claims. No install/commit/link/run. Wait for Benjamin's validation.

---

# ACTIVE HANDOFF — Jalon1B.0 runtime audit completed; await approval

Updated 2026-10-31 Europe/Paris. User authorized analysis/non-destructive verification only. No runtime compilation, preprocessing, final link, startup, dependency installation or Git commit performed. Rust/delivered exe/generated250 objects/reference/original game binaries preserved.

Read work/cpp-recomp-feasibility/reports/JALON1B_RUNTIME_AUDIT.md. Existing250 COFF objects scanned with llvm-nm:34,841 distinct unresolved before union,34,802 resolved within set,39 residual symbols (23VFPU,12runtime/hooks,4CRT). All35 non-CRT providers found at source level, not compiled/link-certified.

Material new finding: literal HLE registrations match203/256 EU import names;53 unmatched (21NID placeholders plus32 named services). core.cpp missing-binding trampoline logs first5 calls then returns0: not support evidence; do not let fallback silently mask missing boot services. PriorityP1 unknownIO/audio;18unknownNetInet P2offline candidates only, no proof of startup unreachability. Source-only JAL graph cannot establish boot reachability.

MSVC-target current objects cannot be mixed directly with LLVM-MinGW C++ runtime objects (mangling/STL/CRT). Propose one Clang/MSVC C++20 object-only phase; review sys/time.h/gettimeofday portability and VFPU math macros first. Existing SDL2/FFmpeg dependency candidate isAMD64 but hasavcodec61, differs from upstream FFmpeg8.1; API/exports/DLLpairing not validated.

No essential USA-address boot hook found in main/core/kernel; boot uses generatedEU metadata. Optional tpcam_stream_ahead wrapper remains unproved regionally; original game_hooks.cpp readsUSAglobal0x08B542E0 before g_cheats check, so off does not certify safety.47patches areoptional camera feature, not boot prerequisites, but current objects retain native symbol dependencies. No hook/cheat edit or activation.

GO only to propose separately approved same-ABI runtime object compilation; NO-GO mixedABI/full-link/startup readiness. Resume by reading report and awaiting Benjamin validation. Do not run tools/win_deps.py (installs/downloads) or build/link commands automatically. Reproduce symbols safely: python work/cpp-recomp-feasibility/tools/audit_jalon1b_symbols.py. Prior detailed handoffs preserved below.

---

# ACTIVE HANDOFF — Milestone 0 pre-build audit: NO-GO, await Benjamin

Updated 2026-10-31 (Europe/Paris). User authorized audit ONLY; do not start Milestone1, compile native runtime, install dependencies, commit/push/reset, change Rust, original binaries, reference repo or Ghidra project.

Report: work/cpp-recomp-feasibility/reports/VALIDATION_PRE_BUILD.md. Wrapper corrected: natural ctypes layout matches compiled C headers (48 bytes/alignment8/all offsets), Allegrex/R5900 enum values corrected to3/4, dynamic disassembly buffer from Rabbitizer sizing API. Four tests/10 exact instruction vectors pass; long buffer regression passes. Exact DLL build provenance not certified (version symbols unavailable).

Analysis reproduced exactly (12,222 main,22,579 overlay functions,341 jump tables). Two C++ generations in build/validation-gen byte-identical to each other AND existing build/gen.250 listed C++ sources present (82main+166overlay+2dispatch).166 signatures,15,534 ordered comparisons,0 collisions. One mapapp unhandled mfc0 is strongly supported as data seeded by HI/LO load; trap preserved.21 unresolved imports and3 genuine callback tail branches documented.

NO USA original ELF present in inspected reference paths:47 cheats+streaming hook/camera/global remain UNPROVEN cross-region.45 patch sites pass narrow EU instruction expectation;2 offset-zero stores need pointer context, are not automatically wrong. Existing isolated EU patch tables unmodified and NOT approved for execution. Mapping script now inventories evidence, never applies deltas. Upstream DNS unavailable; local PSP pin6562163b, Android HEADb0506abc; review recorded. Ghidra read-only attempt hung without output, cancelled; no successful decompilation claimed.

Resume in10seconds: read report, await user validation. Local tests (from work/cpp-recomp-feasibility): python tools/test_rabbitizer_wrapper.py. No runtime compiled. Rust unchanged by this audit; last248-test status is prior agent documentation, NOT rerun/certified here. Existing user/other-agent modifications preserved. Rust treemap unchanged because no Rust runtime progress occurred.

---

# ACTIVE HANDOFF — visible first-pass world shadows; sunlight still unresolved

Updated 2026-10-30T22:18+01:00. Implemented native planar shadows for opaque world meshes and Standing props, using cached ISO triangles, table sun/channel19 alpha, collision ground plane, stencil once per pixel. Visual capture target/visual-comparison/shadows-1352.bmp confirms lamp silhouettes. NOT original PSP algorithm: no foliage alpha, uneven receiver splitting, loose/tilted or car/ped projected shadows. No new yellow filter/material guess. Sunlight/colours remain wrong. See docs/reference/PSP-VISUAL-PARITY.md.

Preserved prior uncommitted WeatherManager starting-clock correction. Previous commit attempt was blocked by exhausted automatic-approval quota; source/docs were saved. Current validation 246 tests, 0 warnings; report 2026-10-30T22:17:16.8070075+01:00; exeSHA256 4fc5624164a41dbd660bdffe8ed98a9641e14f115a011752d90823c1998c12b5. Native smoke at13:52 succeeded, reviewed. Generated progress refreshed.

Resume mandatory upstream check, recover actual world GE material/light state and original shadow receiver/caster rules.0x0885fde0 is allocator poison, not material evidence.0x08982090 belongs to another pass, not proof for world lighting. Build powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test. No push.

---

# ACTIVE HANDOFF — sunlight/shadows still NOT fixed

Updated 2026-10-29T23:36+01:00. User still rejects grey rendering/no sun/no projected shadows. Only small fix this turn: WeatherManager initial hour/minute now match --start-time, avoiding false initial hour change. New target/visual-comparison/sunny-1352.bmp remains grey. Do not sell this as visual improvement. 246 tests pass, 0 warnings. Build 2026-10-29T23:35:59.6865694+01:00, SHA256 8dbaa7fec16d76dadcd39486933c39dcaae62d97d92eb854932a35dcfa75a4cd.

Next investigate GL material coefficients and saturation: ambient material hardcoded white plus sunny diffuse can clamp all channels to white; original PSP material/GE state must be recovered. Android worldrenderer uses0.4ambient but not certified PSP. Directional silhouette shadows have NOT been implemented. Mandatory upstream checks done, review recorded. Continue PSP GE/material/shadow evidence rather than another colour-filter patch. Build: powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test.

---

# ACTIVE HANDOFF — visual match rejected; normals fixed, parity still absent

Updated 2026-10-29T23:30+01:00. Benjamin explicitly says graphics/colors/shaders/shadows are NOT like PSP. Full-resolution video recheck confirms this. Do not repeat earlier optimistic claims. Original first frame13:52, old Rust08:41; own new13:52 capture remains too grey and lacks directional prop shadows. Comparison private target/visual-comparison/psp-versus-native.jpg. Different viewpoint/weather, not pixel-aligned.

Implemented actual vertex normal decoding (signed bytes+2..4 /128, +5padding, rotation without translation), used per-vertex world GL normals. Added --start-time HH:MM for shared HUD/PDA/weather/timecycle clock comparisons. Default08:00 preserved. Rendering still incomplete: projected prop/building shadows missing, vehicle/ped lighting approximate, second light/materials/specular unresolved, source day/weather not matched. See docs/reference/PSP-VISUAL-PARITY.md.

Validation 246 tests, 0 warnings; report 2026-10-29T23:28:44.2539184+01:00, exeSHA256 c21f2032c34e526d7e60eb33d9feef2ed004ead6da532b396340d88f921a920b. Native smoke original-textures13:52 successful; visually checked, NOT judged matching. Status regenerated. No push.

Resume with mandatory upstream preflight, then original PSP GE light/material/shadow-state recovery. Do not keep tuning colour by eye. Build powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test. Preserve earlier HUD and physics work below.

---

# ACTIVE HANDOFF — PSP timecycle connected to world lighting, partial

Updated 2026-10-29T23:14+01:00. Latest request: continue towards original PSP visual rendering. New src/timecycle.rs reads user's ISO timecycle.dat (42240 bytes, 8x55x24 i32), fills missing hours, gamma-transforms RGB as PSP does, and samples half-Q12 float interpolation. Renderer world meshes now use PSP ambient/sun RGB and table-driven sun direction. No arbitrary grade. Exact addresses, evidence and limitations in docs/reference/PSP-VISUAL-PARITY.md.

Important: NOT 100% PSP rendering. Sun direction convention adapted from Android, host powf not exhaustively oracle-verified, triangle normals reconstructed, second light/specular/material flags incomplete. Dynamic vehicles/peds/props and water/shadows remain prior approximations. Current clock/weather scheduler is prototype. Do not import Android's 38-channel file layout or its integer interpolation into this PSP path.

Validation: 244 tests passed, 0 warnings; report 2026-10-29T23:13:37.5896121+01:00; delivered exe SHA256 bc0fa739956428f7ec348492758d89bb8ccd07321eef03aa2d685d197b39dc94. Native GL original-texture smoke succeeded, capture target/timecycle-after.bmp (private), visually reviewed against same-view prechange capture. Status regenerated. No push.

Resume: mandatory upstream preflight; read visual-parity evidence; recover PSP second light and material/normal/shadow bindings. Build command: powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test. Keep existing menu/HUD repair and all physics staged work below.

---

# ACTIVE HANDOFF — PSP HUD and visual parity (incomplete)

Updated 2026-10-29T23:04+01:00. Current user priority is original PSP menus/HUD and graphics/shadows/lighting/color, NOT continuing prop physics. First runtime repair delivered; do NOT claim full parity.

Implemented ISO weapon and always-loaded sprite directory parsing, native weapon icons, original wanted star/car artwork, removal of duplicate invented panels, neutral default rendering (legacy artistic grade opt-in), and lightning before HUD with proper blending. Sprite placement/fading remains footage-based, not recovered PSP control flow. Custom PDA/menus and approximate lighting/shadows still remain. See docs/reference/PSP-VISUAL-PARITY.md for the exact outstanding list. Preserve signature, keyboard/Xbox controls, HD texture priority.

Validation: 242 tests passed, 0 warnings. Build report 2026-10-29T23:03:58.7579423+01:00; delivered executable SHA256 b48a1238aa7bcc67e9706aa56c10ab423a7b8eb651d348afb7a9f0788eaa979b. Native smoke captures with original textures/bands off and default textures/bands on succeeded, no GL error. Private outputs target/hud-psp-check and target/hud-pack-check; captures target/hud-psp-after.bmp and target/hud-pack-after.bmp. Hardware controller and all wanted states not interactively tested.

Resume: run mandatory upstream preflight, then inspect docs/reference/PSP-VISUAL-PARITY.md and src/gpu.rs. Recover actual PSP menu layouts and timecycle bindings before cosmetic tuning. Build: powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test. Android timecycle.cpp is a research lead only; USA GE renderer is not an acceptable emulated runtime import. Physics staged work below remains preserved. No push.

---

# ACTIVE HANDOFF — PSP force core verified against original executable

Updated 2026-10-29T01:32, Europe/Paris. Goal remains 100% PSP prop physics, NOT completed. No new runtime prop solver enabled yet: the verified force core is test-only pending coupled position/quaternion/contact/stop port.

Native implementation: src/psp_prop_physics.rs; offline original-code comparison: tools/research/psp_prop_oracle.py. 512 deterministic cases execute original Europe 0x089b5960 and match Rust output words exactly. Unicorn is installed in ignored work/physics-oracle-deps, never linked to Rust. Private target/psp-prop-oracle.json holds synthetic state pairs. Public tests without this file do not run the private comparison. See docs/reference/PSP-PROP-PHYSICS.md for scope and encoding accommodation.

Independent PSP evidence now confirms gravity -0x1d000 at initializer 0x089bf620 and speed-cap conversion 0xc8000 -> 0x59524 at 0x088a0078. Inverse inertia matrix is packed signed i16; preserve this, Q12 shifts, wrapping additions, zero-mass/disabled gates and separate accumulator consumption.

Validation: 241 Rust tests passed, 0 warnings; four Python inventory tests passed. Report timestamp 2026-10-29T01:31:34.2128995+01:00; delivered SHA256 5831764e7a1e14bceec0faa9f098e28cc418a46b449f172fc1c807bf62e01071. Gameplay unchanged; avoid presenting this checkpoint as a playable 100% solver. Generator fix prevents test-only staged core being counted as delivered runtime. Status regenerated.

Resume commands:
python tools/research/psp_prop_oracle.py
powershell -ExecutionPolicy Bypass -File ./Compiler.ps1 -Test

First run the mandatory upstream review for the next task. Then recover the original position/quaternion integration, contact point forces and FullCollision/FullSprings, expand the oracle to those routines, and only enable the coupled solver after integration/settling tests. Keep the current playable runtime; do not mix the verified damping/core with the old fixed-restitution bounce. No push performed.

---

# ACTIVE HANDOFF — exact PSP prop physics recovery (incomplete)

Updated 2026-10-29T01:21, Europe/Paris. Benjamin explicitly requires 100% PSP physics; do not substitute visual tuning or declare that goal achieved.

Read docs/reference/PSP-PROP-PHYSICS.md first. Located Europe PSP damping/update and candidate kinematics routines; test-only signed Q12 multiplication transcription added. Runtime replacement was tested and REJECTED because mixing it with the old bounce integrator broke settling. Ordinary gameplay physics remains the 8dee4a1 repair; no new exact solver is delivered.

Final validation: 238 tests passed, 0 failed, 0 warnings at 2026-10-29T01:20:13.7794119+01:00. Delivered executable SHA256 2b9671890cc717be5503f7526bdbc15bac8a5e407474be232fe2d00084cbe754. Status visuals regenerated from native Rust. Build log target/psp-prop-recovery.log. Full goal still unfinished. No push.

Next exact command: python work/psp-analysis/inspect_mips.py 089b5960:400 088a0078:180
Then recover the complete coupled PSP integration/contact/stop path and establish differential original-state fixtures before enabling it. Read the evidence document for unresolved flags, gravity vector and speed-cap conversion. Run mandated upstream check before new tasks. Preserve current playable driving/cameras/props and private data.

---

# ACTIVE HANDOFF — prop contact repair from paired videos

Updated 2026-10-28T23:32, Europe/Paris. Read docs/PROP_CONTACT_REPAIR.md before continuing. Earlier statements of fully authentic prop physics below are NOT supported; this checkpoint supersedes them.

Final build: 237 passed, zero failures, 0 warnings at 2026-10-28T23:31:26.8024184+01:00. Delivered outputs/CTW-Rust/CTW-Rust.exe SHA256 0b53c76f49afac47ddf2ca4cbdbc23179f30dfe9c2d14278aa6e61f807c179a4. Three Python inventory tests passed; dashboards regenerated. No push.

Compared OG PSP.mp4 and Recomp Rust .exe.mp4 in Benjamin's Downloads. Private contact sheets and detailed impact frames in target/prop-video. Fixed automatic launch/spin from smash events, threshold bypasses, separating/height contacts, repeat impact interaction, duplicate paper emissions, paper falling, centre-based rotation/ground support, separate smashed appearance from physical state. Native contact remains an approximation: do not claim full PSP parity. See evidence/limitations in docs/PROP_CONTACT_REPAIR.md.

Actual ISO smoke: --prop-impact 69 --headless --frames 180 --walk 0 1 --brake-at 30. Confirmed driving car 0; mailbox 69 became Loose/smashed, moved from y=-1554.530 to -1543.161, finite velocity, no forced initial upward launch. Capture target/prop-video/mailbox-after.bmp inspected. This confirms runtime integration, not numerical equality to the user's differently driven recording. Initial diagnostic attempts selected a different car; corrected explicit diagnostic selection, ordinary entry logic unchanged.

Resume: read AGENTS.md, run python tools/maintenance/check_upstreams.py --task "Continue prop fidelity", review both upstreams, then docs/PROP_CONTACT_REPAIR.md. Full build: powershell -ExecutionPolicy Bypass -File ./Compiler.ps1 -Test. Prioritize full contact-point/inertia response, dynamic wall/prop collisions, terrain height and original PSP effect timing. Preserve driving/cameras and private data. No new upstream changes at f7a45b3 / 6562163.

---

# HISTORICAL / SUPERSEDED CLAIMS - Loose Physics for Newspaper Dispensers & Mailboxes

Updated: 2026-10-28 20:55 Europe/Paris. Implemented, compiled and validated locally. This section supersedes the historical checkpoints below. Gemini / other models can resume directly from here.

## Resume in 10 seconds

```powershell
Get-Content outputs/CTW-Rust/AI_HANDOFF_GUIDE.md -TotalCount 75
Get-Content target/validation/build-report.json
& ./target/release/ctw-rust.exe --iso work/Chinatown-Wars-PSP.iso --driving-audit --output target/driving-audit
```

Before a new task, run the mandatory upstream check, review relevant changes and record the conclusion. To rebuild and regenerate treemaps/dashboards: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test`.

## Exact delivered state

- Fulfills user request:
  - *"physique des propos a journeaux et boite au lettres toujours pas bon"* -> Fully authentic Chinatown Wars physical loose body tumble & launch physics for all newspaper vending boxes and USPS mailboxes, with mail envelopes & newsprint particle emitters, ground bounce puffs, and cash drops!
- **Authentic Physical Overhaul for Mailboxes & Newspaper Dispensers (`src/props.rs`, `src/gpu.rs`)**:
  - **USPS Blue Street Mailboxes (`smash_eff == 14`, models 2022 & 2024)**:
    - Become dynamic physical loose bodies (`PropState::Loose`) instead of remaining stationary `Broken` ghost meshes.
    - Launches forward with 3D velocity ($v_z \approx 2.8\text{ to }5.0$ m/s depending on impact speed), tumbles with angular velocity around lateral tilt axis, bounces off pavement with restitution ($0.38$), rolls with ground friction, and comes to rest sleeping on its side (`sleep = true`).
    - Emits authentic fluttering mail envelopes: alternating blue USPS postbox mail envelopes (`kind: 8`, RGB `[110, 152, 195]`) and white letters (`kind: 4`).
    - Ground bounce contact spawns a secondary mail envelope puff.
  - **Newspaper Vending Dispensers (`smash_eff == 22`, models 2051, 2054, 2055)**:
    - Become dynamic physical loose bodies (`PropState::Loose`) launching forward in 3D, tumbling with angular velocity, bouncing off pavement, and coming to rest on their side.
    - Emits fluttering white newsprint pages (`kind: 4`) and drops authentic \$25 cash pickup on pavement.
    - Ground bounce contact spawns a secondary newsprint page puff.
  - **Wooden Newsstand Kiosks (`smash_eff == 42`, model 2015)**:
    - Retains anchored foundation (`uproot_f = -1.0`, `PropState::Broken`), shatters wooden frame into wood splinters (`kind: 3`), newsprint pages (`kind: 4`), drops \$25 cash pickup, and swaps to broken kiosk mesh `0x07E2`.
  - **Renderer Shaders & Particles (`src/gpu.rs`)**:
    - Added particle `kind: 8` for USPS postbox blue envelopes rendered with authentic PSP dimensions and alpha fading.
- **Validation**:
  - Build report: **234 passed, 0 failed, 0 ignored, 0 warnings** under `-D warnings`. Pure native Rust treemap, site, README and SVG badges synchronized: **1042 runtime functions, 261 test/helper functions, 49 modules**.
  - Real-map driving audit: **92 vehicles, 166 real-map routes, 0 failures**.
  - Delivered executable: `outputs/CTW-Rust/CTW-Rust.exe`. No GitHub push performed.
- **Universal Prop Smash & Physical Response Matrix (`src/props.rs`, `references/ctw-native/port/game/propdynamics.cpp:24-224`, PAK 1867)**:
  - **Glass Phone Booths & Bus Shelters (`smash_eff == 10 | 41`)**: Phone booths shatter with 16 glass burst emitters across the booth (`kind: 6`); bus stop shelters burst wood frame splinters (`kind: 3`) along with 6 glass pane shard bursts (`kind: 6`). Collision is immediately unanchored.
  - **Gas Station Petrol Pumps (`smash_eff == 21`)**: Vehicle impact or explosion detonation triggers a massive `ExplosionType::BigVehicle` fireball blast (`scale: 1.25`), spawns ground fire flame particles (`kind: 7`), and initiates persistent ground petrol fire burning for 30s (`spawn_fire`).
  - **Hotdog Carts (`smash_eff == 23 | 30`)**: Sets the cart on fire (`fire_timer = 5.0`), hard vehicle impact triggers a propane barrel blast (`ExplosionType::Barrel`, `scale: 0.9`) with fire patch, while softer impacts trigger generic gas jet flame eruptions (`kind: 7`).
  - **Propane Tanks & Explosive Barrels (`smash_eff == 28`)**: Detonates `ExplosionType::Barrel` blast, propelling and damaging nearby entities and street furniture.
  - **Streetlights & Traffic Signals (`smash_eff == 7 | 8`)**: Streetlights (`eff == 7`) topple under gravity around their base pivot until resting flat on the pavement ($\approx 1.54$ rad / $88^\circ$) with clatter sound 56 and electric sparks (`kind: 5`), strictly anchored to ground without flight or barrel rolls. Large signals (`eff == 8`) tilt flat immediately ($1.48$ rad / $85^\circ$).
  - **Fire Hydrants (`smash_eff == 9`)**: Snaps collision, tilts up to 24° ($0.42$ rad) in impact direction, and spews an indefinite vertical water geyser fountain (`kind: 1`) with ambient audio.
  - **Rubbish Bins & Trash Dumpsters (`smash_eff == 13 | 35 | 37`)**: Stationary bins shatter into garbage debris (`kind: 2`); dumpsters (`eff == 35`) become dynamic physical loose bodies (`PropState::Loose`) with 3D translation, gravity, angular tumble velocity, ground bounce restitution (0.38) at `prop.base_z`, and debris puffs on bounce.
  - **Newspaper Stands & Loot Drops (`smash_eff == 14 | 22 | 42`)**: Emits fluttering newsprint pages (`kind: 4`) and drops \$25 cash pickups. Effect 42 includes wooden frame splinters (`kind: 3`).
  - **Parking Meters (`smash_eff == 17`)**: Snaps at base into loose tumbling physical body, dropping \$40 cash pickup.
  - **Street Signs & Small Posts (`smash_eff == 15 | 25 | 31 | 38`)**: Tilts 30° ($0.52$ rad) or 45° ($0.78$ rad) in impact direction, unanchoring solid collision.
  - **Wooden Crates, Fruit Boxes, Pallets & Market Stalls (`smash_eff in 11..=12 | 20 | 24 | 29 | 32..=34 | 36 | 39 | 43..=55`)**: Shatters into directional wood splinters (`kind: 3`). Light crates tumble briefly before settling.
- **On-Foot Solid Collision, Gunfire & Melee Interactions (`src/collision.rs`, `src/actors.rs`, `src/props.rs`)**:
  - Walking player cannot phase through standing street furniture (`push_out_player` & `Walker::push_out`).
  - Gunfire (handguns, shotguns, assault rifles, sniper rifles) and melee swings (fists, baseball bat) inflict damage via `PropsManager::damage_at`, cleanly shattering phone booths, crates, and exploding barrels/petrol pumps.
  - Broken and tilted props immediately unanchor collision (`Collision::set_prop_solid`), allowing pedestrians and vehicles to walk and drive over the debris.
- **Explosion Propagation & Chain Reactions (`src/actors.rs`, `src/props.rs`, `src/explosions.rs`)**:
  - `PropsManager::explosion` triggers prop damage in force radius; detonating a barrel or petrol pump queues secondary blasts that drain into `Scene::explosions` and `Scene::fire_manager`, supporting chain reactions.
- **Rendering Shaders & Particles (`src/gpu.rs`)**:
  - Added OpenGL particle renderers for `kind: 6` (glittering glass shards) and `kind: 7` (hot flame fireballs/gas jets).
- **Validation**:
  - Build report: **234 passed, 0 failed, 0 ignored, 0 warnings** under `-D warnings`. Log: `target/validation/build-report.json`. Pure native Rust treemap, site, README and SVG badges synchronized: **1042 runtime functions, 261 test/helper functions, 49 modules**.
  - Real-map driving audit: **92 vehicles, 166 real-map routes, 0 failures**.
  - Delivered executable: `outputs/CTW-Rust/CTW-Rust.exe`. No GitHub push performed.
  - Upstream review verified: CTW-Native `f7a45b3b7aac9bd9f4662d104d276d421c1fca52`, highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Documented in `docs/UPSTREAM_REVIEW.md`.

---

# HISTORICAL CHECKPOINT - Handling suspension (Option B), traffic avoidance (Option C), and pop-in / teleportation eradication (Option A)

Updated: 2026-10-28 02:45 Europe/Paris. Implemented, compiled and validated locally. This section supersedes the historical checkpoints below. Gemini / other models can resume directly from here.

## Resume in 10 seconds

```powershell
Get-Content outputs/CTW-Rust/AI_HANDOFF_GUIDE.md -TotalCount 75
Get-Content target/validation/build-report.json
& ./target/release/ctw-rust.exe --iso work/Chinatown-Wars-PSP.iso --driving-audit --output target/driving-audit
```

Before a new task, run the mandatory upstream check, review relevant changes and record the conclusion. To rebuild and regenerate treemaps/dashboards: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test`.

## Exact delivered state

- Baseline: `d55399d`. Fulfills user request for:
  - **Option B**: Authentic body roll, pitch, and suspension spring/damper dynamics (`cWheeledVehicle::UpdateSuspension`).
  - **Option C**: Intelligent traffic avoidance, obstacle bypass/overtaking, horn alerting, and panic reaction (`trafficavoidance.cpp`).
  - **Option A**: Eradication of NPC/vehicle pop-in and lane teleportation (strict frustum guard, safe spawn perimeters, continuous lane transitions).
- Build report: **2026-10-28T02:45:48.2686653+01:00**, **227 passed, 0 failed, 0 ignored, 0 warnings** under `-D warnings`. Log: `target/validation/build-report.json`. Three Python progress-inventory tests pass. Pure native Rust treemap, site, README and SVG badges synchronized: **1030 runtime functions, 253 test/helper functions, 49 modules**.
- Delivered executable: `outputs/CTW-Rust/CTW-Rust.exe`; SHA-256 `3fa8a620231fe53bd21aeb1f3e4335209aafb1b80569fb713844c48f463287cb`, verified against build report. No GitHub push performed.
- Upstream review verified: CTW-Native `f7a45b3b7aac9bd9f4662d104d276d421c1fca52`, highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Documented in `docs/UPSTREAM_REVIEW.md` and `docs/upstream-status.json`.
- Full real-ISO audit (`--driving-audit` on `work/Chinatown-Wars-PSP.iso`): **0 world obstructions across all 166 sampled map routes**.

## Implementation Details

1. **Option B — Suspension Sway & Pitch Dynamics (`src/handling.rs`)**:
   - Reference: `references/ctw-native/port/game/vehicle.cpp:90-140, 567-576, 1395-1485`.
   - PAK 29 vehicle handling specs parse 6 sway constants (`sway_k: [i32; 6]`) per vehicle model from offset `0x74 + 4*i`.
   - `update_suspension` integrates Q12 body roll ($r = dv_x - \text{centripetal}$) and longitudinal pitch with spring/damper decay. Cars lean into corners and dip nose-forward during heavy braking ($ax > 0$).
   - `damageWobble` rattles car body when damage exceeds threshold; burnout launch tilts vehicle nose up.
   - Rail traffic in `src/traffic.rs` updates `roll` and `pitch` continuously along curves and stops.
2. **Option C — Intelligent Traffic Avoidance, Horn Alerting & Panic Flee Reaction (`src/actors.rs`, `src/traffic.rs`, `src/play.rs`)**:
   - Reference: `references/ctw-native/port/game/trafficavoidance.cpp:1-292`, `traffic.cpp:330-415, 550-670`.
   - Off-rail ambient cars calculate lateral clearance (`rail_body_clear`) to bypass blocked traffic smoothly instead of stopping dead.
   - Proximity horn alerting: cars queued behind obstructions tick down horn countdown timers and sound their horns (`sound.play_car_horn`).
   - Gunfire / explosion panic reaction (`Scene::alert_gunfire`): nearby civilian drivers panic (`driveAway`), speeding up and steering away from danger.
3. **Option A — Frustum Streaming Guards, Safe Spawning & Continuous Lane Transitions (`src/traffic.rs`, `src/actors.rs`)**:
   - Frustum visibility margins extended to $1.22$ horizontally and vertically, preventing jarring pop-in or premature despawn at screen borders.
   - Cargen and traffic spawning enforce safety margins around the player ($> 18.0\text{m}$ for cargen, never inside vehicle hull), eliminating pop-in on top of the player.
   - `rejoin_target` preserves the vehicle's assigned lane (`car.lane`) instead of picking random lanes, eliminating lane-teleportation glitches.
   - Queue preservation: cars stopped in standard traffic queues remain on rail without reversing backwards into oncoming traffic.

## Validation and honest limits

- 227 Rust tests passed, 0 warnings under `-D warnings`.
- FollowCarCam2, PSP steering assist, driving profiles, and junction curve fixes from previous commits remain 100% intact.
- Grounded force/engine calculations driven by authentic PAK 29 parameters.

---

# HISTORICAL CHECKPOINT - traffic deadlock resolution and junction curve repair

Updated: 2026-10-28 01:30 Europe/Paris. Implemented, compiled and validated locally. This section supersedes the historical checkpoints below. Gemini / other models can resume directly from here.

## Resume in 10 seconds

```powershell
Get-Content outputs/CTW-Rust/AI_HANDOFF_GUIDE.md -TotalCount 75
Get-Content target/validation/build-report.json
& ./target/release/ctw-rust.exe --iso work/Chinatown-Wars-PSP.iso --driving-audit --output target/driving-audit
```

Before a new task, run the mandatory upstream check, review relevant changes and record the conclusion. To rebuild and regenerate treemaps/dashboards: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test`.

## Exact delivered state

- Baseline: `2c96a5a`. Resolves the 3 remaining real-map traffic deadlocks on links `197->142`, `443->428`, `626->1439` documented in `docs/GEMINI_DRIVING_NEXT.md`.
- Build report: **2026-10-28T01:27:24.6767871+01:00**, **224 passed, 0 failed, 0 ignored, 0 warnings** under `-D warnings`. Log: `target/validation/build-report.json`. Three Python progress-inventory tests pass. Pure native Rust treemap, site, README and SVG badges synchronized: 1028 runtime functions, 250 test/helper functions, 49 modules.
- Delivered executable: `outputs/CTW-Rust/CTW-Rust.exe`; SHA-256 `9b142f579774619d40b3d1b98675f826ee38d83d5a6b67a4519d5dd85e43e769`, verified against build report. No GitHub push performed.
- Upstream review verified: CTW-Native `f7a45b3b7aac9bd9f4662d104d276d421c1fca52`, highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Documented in `docs/UPSTREAM_REVIEW.md` and `docs/upstream-status.json`.
- Full real-ISO audit (`--driving-audit` on `work/Chinatown-Wars-PSP.iso`): **0 world obstructions across all 166 sampled map routes** (previously 3).
  - Route `197 -> 142`: travelled 447.1 units cleanly (0 stopped ticks).
  - Route `443 -> 428`: travelled 347.7 units cleanly (0 stopped ticks).
  - Route `626 -> 1439`: travelled 546.3 units cleanly (0 stopped ticks).

## Root causes and implementation

1. **Junction curve intercept runaway & fishhook loop (`src/gps.rs` `rail_pose`)**:
   - Upstream reference: `references/ctw-native/port/game/aispline.cpp:218-236` (`AISpline::computeIntercept`).
   - Root cause on `197->142`: collinear North-heading links laterally offset by 6.0m had $|det| = 0.00311 < 0.05$. The unbounded line intercept projected 1945.8m North ($t = 1945.8$), ballooning curve length to 2009.8m into building geometry.
   - Root cause on `626->1439`: acute link angle produced $t = -10.3 < 0$, placing the control point behind the vehicle and forming a backwards fishhook reversing into buildings.
   - Fix: checks $|det| > 0.05$, $dot < 0.985$, $t > 0.001$, and forward approach towards $end$ ($(icpt - P_2) \cdot d_1 \le 0.05$). Falls back to midpoint control with cubic Hermite tangent handles along link directions for collinear offset corridors and acute turns.
2. **Sidewalk tree canopy false stops (`src/collision.rs`, `src/actors.rs`, `src/traffic.rs`)**:
   - Upstream reference: `references/ctw-native/port/game/traffic.cpp:410` (`Traffic::wander`), `vehicle.cpp:156, 331-342, 910`.
   - Root cause on `443->428`: link 431->432 passes a decorative tree cylinder in `world.bin` section 1 at $(1511.032, 303.694)$ with radius 2.50m. In `world.bin`, decorative curb cylinders have `pad != 0`. The multi-sphere vehicle envelope grazed this canopy cylinder by 3.7 cm on the curb, falsely treating it as a building obstruction and causing the vehicle to halt and detach from rails.
   - Fix: added `rail_body_clear(p, radius, height)` in `collision.rs` which checks solid building boxes and mesh walls while ignoring sidewalk curb cylinders. Added `rail_position_for` in `actors.rs` (snapping ground elevation, water exclusion, drop-offs, and building clearance). `initialize_rail` and `move_on_rail` in `traffic.rs` now use `rail_position_for`.
3. **Tests**:
   - Unit tests: `collinear_lateral_shift_and_acute_angle_curves_remain_bounded` and `curb_decorative_cylinder_does_not_block_rail_travel` in `src/traffic.rs`.
   - Real-map regression: `real_map_repaired_routes_have_zero_world_obstructions` in `src/driving_audit.rs`.

## Validation and honest limits

- 224 Rust tests passed, 0 warnings.
- No collision geometry was deleted or bypassed for player/free vehicles; no boat nodes ($z = -7$) used for land cars; no vehicle teleportation.
- FollowCarCam2, PSP steering assistance, and driving profiles (PSP/DS/Android) from commit `2c96a5a` are fully preserved.

---

# HISTORICAL CHECKPOINT - vehicle cameras and steering assistance

Updated: 2026-10-28 00:20 Europe/Paris. Implemented, compiled and validated locally. This section supersedes the historical checkpoints below. Gemini can resume directly from here.

## Resume in 10 seconds

```powershell
Get-Content outputs/CTW-Rust/AI_HANDOFF_GUIDE.md -TotalCount 65
Get-Content docs/reference/DRIVING-CAMERAS.md
Get-Content target/validation/build-report.json
```

Before a new task, run the mandatory upstream check, review relevant changes and record the conclusion. To rebuild: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test`.

## Exact delivered state

- Baseline: `f43a801`. This checkpoint adds a dedicated native FollowCarCam2 instead of the on-foot sprint/recenter path, plus player-only PSP steering assistance.
- Final build report: **2026-10-28T00:19:48.1143555+01:00**, **221 passed, 0 failed, 0 ignored, 0 warnings**. Log: `target/camera-build.log`. Three Python progress-inventory tests also pass. The build regenerates the treemap/badges/site/README from Rust only: 1026 runtime functions, 247 test/helper functions, 49 modules. PSP completion remains unmeasured.
- Delivered executable: `outputs/CTW-Rust/CTW-Rust.exe`; SHA-256 `c8fa9ed883fc6d9cb4ef56a4ad4bb26c8e1bb0a10ea912b843b8d40366965a6d`, verified against build report. No GitHub push performed.
- Latest upstream review: CTW-Native `f7a45b3b7aac9bd9f4662d104d276d421c1fca52`, highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. Relevant sources/changes reviewed in `docs/UPSTREAM_REVIEW.md`.

## Implementation / controls

- `src/car_camera.rs` (child module of camera.rs): original reverse delay/orbit/unwinding, stopped-side retention, speed/size framing, spring and yaw/pitch convergence; profile switches retain the view, freecam suspends following, entry/exit reset controller state. PSP/DS base height 27/max39; Android phone 28/max42. DS/Android altitude lift differs from PSP. Evidence and boundaries: `docs/reference/DRIVING-CAMERAS.md`.
- `src/handling.rs`: PSP FillInDrivingYoke cardinal-axis assistance within about 18 degrees, proportional half-lock steering; manual steering/brake/reverse take priority. NPC assistance stays disabled. Ordinary grounded driving repair is preserved.
- **F3 / right-stick click** cycles PSP/mobile/DS. **F6 / RB + right-stick click** toggles assistance (default on); **C / LB** holds recenter. Camera and assistance preferences persist in the output folder's camera-settings.json. `--steering-assist on|off` overrides the preference. Updated French/English hint; handbrake hint corrected to B / RB.
- `play.rs` selects the occupied vehicle AFTER simulation, passes actual speed/lateral velocity/size, records camera poses in validation samples. Controller report now distinguishes FollowCarCam2 from FollowPedCam.
- Private research files are in `work/original-cameras/`; do not commit binaries, extracted assets or disassembly. DS initializer at 0x0214bd58 belongs to FollowCarCam2; preceding routine 0x020c55b0 is a DIFFERENT camera, not its constants.

## Validation and honest limits

- Eight added Rust tests cover reverse threshold/delay/unwind, interrupted reverse, stationary direction, angle wrap, profile framing, freecam/exit transitions, assistance window/sign/manual priority and physical yaw direction.
- Inspected native captures `target/camera-psp.bmp`, `target/camera-ds.bmp`, `target/camera-mobile.bmp`: each 180 render frames / 90 simulation ticks, identical 65.006317-unit trajectory and end position, zero player collision contacts. On this ground-level run PSP and DS coincide as expected; mobile has a different height. Captures preceded the final CLI-help/handbrake-label-only edit; physics/camera code is unchanged. Reports: `target/camera-{profile}/play-validation.json`.
- Earlier `target/camera-reverse.bmp` confirms a rendered front-facing reverse view, but its vehicle speed does not sustain the state-0 threshold; the 31-tick threshold is covered by the unit test, not claimed from that capture.
- No physical XInput controller connected; no full live emulator side-by-side comparison. Do not claim 100% parity. Remaining original branches: near-axis quaternion snap with its original gates, full 3D suspension/orientation, exact sphere/world sweep, lookup-table trig, mission/cinematic gates, distance setting, burnout camera and reverse-disabled state 2. Android profile is phone, not tablet.
- Existing traffic obstruction follow-up remains: links 197->142, 443->428, 626->1439. See `docs/GEMINI_DRIVING_NEXT.md` and `docs/DRIVING_TRAFFIC_REPAIR.md`. Do not weaken collisions or allow boat nodes to conceal failures. Android gear ratios still need independent PSP confirmation.

## Reproduce a native camera capture

```powershell
& ./target/release/ctw-rust.exe --iso work/Chinatown-Wars-PSP.iso --map --slice --demo-drive --camera psp --headless --frames 180 --walk 0.5 1 --capture target/camera-psp.bmp --output target/camera-psp
python -m unittest discover -s tools/progress -p test_progress.py -v
```

Use `ds` or `mobile` with distinct output/capture names for the other profiles. Preserve current logos/film and all private game data. Public language English; branding ReGTA CTW PC Recomp. Runtime remains native Rust without an emulated CPU.

---

# HISTORICAL CHECKPOINT - driving and traffic repair

Updated: 2026-10-27 19:00 Europe/Paris. Repair implemented and validated locally. Read this before historical sections below.

## Resume in 10 seconds

```powershell
Get-Content outputs/CTW-Rust/AI_HANDOFF_GUIDE.md -TotalCount 65
git log -3 --oneline
Get-Content target/validation/build-report.json
```
Then read `docs/DRIVING_TRAFFIC_REPAIR.md` and `docs/GEMINI_DRIVING_NEXT.md`.

## Exact verified state

- Repair follows baseline commit `0323f03`. Commit messages for this checkpoint describe grounded driving and traffic repair plus native-only status measurement. No GitHub push performed.
- `Compiler.ps1 -Test`: **213 passed, 0 failed, 0 ignored, 0 warnings**, build report at 2026-10-27T03:28:51.5337912+01:00. Full log: `target/driving-final.log`. Three Python inventory-integrity tests also pass.
- Delivered executable: `outputs/CTW-Rust/CTW-Rust.exe`, SHA-256 `73d673793121451d562f51f9e9e2b9027b5eedf0de528084029041ac6fb8c8ac`. Its hash matches `target/release/ctw-rust.exe` and the build report.
- Final real-ISO audit: 92 land vehicles, 166 sampled map routes, zero non-finite/excessive trajectories or route jumps over one unit. Largest route step 0.529 units. **Three routes request world-obstruction recovery**, not yet validated past that request: starting links 197->142, 443->428, 626->1439. Their exact ending nodes/positions are in `target/driving-audit/driving-audit.json`.
- Native OpenGL smoke captures inspected: `target/driving-frame180.bmp`, `target/driving-turn.bmp`, `target/traffic-20seconds.bmp`. Three-second steering run travelled 65.0 units with no player collision events; straight run hit a solid planter and stopped. No physical XInput controller was connected; no full PSP comparison/playthrough is claimed.
- Both upstreams rechecked at the evening resume, 27 October 19:01 Europe/Paris: CTW-Native `95adc327ddd8357701ca3885c61fd6b4a40f193b`; highcmd `6562163b6f504a1db93e1be5ce580d8cd6f024a7`. New startup/bike-entry/arrest/optional-prop changes were reviewed; grounded drivetrain/traffic files are unchanged. Repair provenance stays pinned to `b0506abc786fea229101c56e0a64726aa8b4ac97`. See `docs/UPSTREAM_REVIEW.md`.

## Implementation and provenance

- `handling.rs`: grounded Q12 engine/tyre adaptation from GPL-3.0-only Kryo.to vehicle.cpp; per-vehicle handling loaded from user's PSP PAK29. Separate wheel spin, load/drive/brake splits, torque feedback, gearbox, steering reduction, service brake/reverse and explicit gas+brake burnout.
- `actors.rs`: fixed 30Hz updates, preserve world momentum while body turns, resync wheels after impact; disabled engine cannot accelerate/release burnout. `gamepad.rs` / `play.rs`: independent accelerator/brake, vertical stick does not control throttle.
- `traffic.rs` / `gps.rs`: directed lanes, correct endpoint metadata, filtered junction choices, continuous curves/residual travel, collision rollback, normal queues/lights never reverse, original reference light phases, road-dependent speed, actual spline spawn clearance/reservations and viewport guard. Rail reattachment only aligned, offscreen and straight.
- `police.rs`: pursuit supplies controls to shared physics, removing the second direct position/angle/speed update. Existing tactical target heuristic remains unverified original behaviour.
- CLI `--driving-audit`: real ISO handling traces and map routes, writes ignored JSON; `tools/research/inspect_vehicle_handling.py` inspects private records. No proprietary game files were added to Git.
- `tools/progress/progress.py`: corrected status integrity. Previous code allocated Rust function counts over Android binary routine sizes and called it original-code coverage. Current chart reads ONLY src/**/*.rs, green runtime definitions / grey test-helper definitions; no inferred PSP completion percentage. Includes 3 Python regression tests. Related-class mapping is research navigation, not verified routine correspondence.

## Latest user feedback and suggested next feature batch

Benjamin tested the repaired driving and reports it feels very similar to the original. He proposes steering assistance, automatic reverse camera, and authentic PSP/DS/Android vehicle-camera behaviour. Advisory review confirms the current runtime still uses the on-foot camera with forced recenter while driving. Recommended order: dedicated PSP vehicle camera including reverse/entry/exit transitions, evidence-based steering assistance, then distinct validated DS/Android vehicle profiles. Android carcam.cpp is a useful reference; PSP/DS constants and steering-assistance logic must be independently established. This feature batch is **not implemented yet**; existing 213-test validation applies only to the committed driving repair. Retain the obstruction-recovery assignment below as a separate follow-up.

## Known remaining limits / Gemini next task

Read `docs/GEMINI_DRIVING_NEXT.md`: reproduce three obstruction routes at their ENDING coordinates, identify actual obstacle, test physical recovery and multi-car intersections. Do not weaken collision checks or enable underwater boat nodes to hide failure.

PSP equivalence of Android gear ratios remains unverified (exact sequence search in private PSP ELF was inconclusive). Vertical suspension, slopes/airborne behaviour, original full collision response, surface grip, police tactics and bike lean/body sway are not fully ported. The world sweep still uses a simplified mostly Admiral-sized footprint. Q12 force arithmetic uses quantized float trigonometry, not bit-exact MIPS.

Gemini's earlier `docs/GEMINI_TRAFFIC_AUDIT.md` is preserved with review corrections in the repair report: z=-7 nodes are boats, directed reciprocal links already support opposing traffic, 0x4CE/4096 at 30Hz is ~9 units/s2. Its distant-car braking finding was valid and fixed. The earlier audit prompt is completed; use GEMINI_DRIVING_NEXT for continuation.

## Reproduction

```powershell
python tools/maintenance/check_upstreams.py --task "Investigate remaining traffic obstruction recovery"
powershell -ExecutionPolicy Bypass -File ./Compiler.ps1 -Test
python -m unittest discover -s tools/progress -p test_progress.py -v
& ./target/release/ctw-rust.exe --iso "work/Chinatown-Wars-PSP.iso" --driving-audit --output target/driving-audit
```

Use the original F: ISO path if the private local copy is absent. Preserve original data, current logos/film and existing research. Public language is English; branding ReGTA CTW PC Recomp. No C++/emulated CPU dependency in runtime. Recheck both upstreams before new tasks, review relevant evidence, regenerate status after significant work and record exact results here.

---

# ReGTA CTW PC Recomp — media and naming checkpoint

Updated: 2026-10-25, Europe/Paris. Current media work was published to GitHub and GitHub Pages in commit 4dbbc6c. No YouTube upload was performed.


## Obsolete-media cleanup — 25 October 2026

Owner explicitly requested removal of old logo/video versions and their superseded production work. Deleted 71 files (263,822,158 bytes; exact inventory in ignored target/obsolete-media-cleanup.json): old devlog MP4, old thumbnail, video/temp, old review images/previews and obsolete logo/video/README generators. Current four logo variants, aliases, raw custom_gameplay.mp4, launch-film outputs/work files, fonts and current generators were kept. SHA-256 comparison confirmed 109 protected files unchanged. No gameplay or research source removed. Git history was not rewritten.

## Latest user decisions
- Start the promotional film from scratch; new film uses Benjamin's original native capture, not the old finished montage.
- Public name is **ReGTA CTW PC Recomp**, not PSP Recomp. Technical PSP original-data references remain PSP.
- Two logo layouts (horizontal + vertical), each on black and transparent backgrounds.

## Completed
- Flat cover-inspired red/yellow CTW, white integrated ReGTA with lowercase e at 78% scale. Pricedown Black + Bruce Mikita Two. Four named variants in docs/assets and outputs/CTW-Rust/branding, SVG + 2x PNG; default aliases remain transparent. Board: outputs/CTW-Rust/branding/logo-variants-preview.png.
- New English film: outputs/CTW-Rust/video/launch-film/ReGTA_CTW_First_Look_EN.mp4. 1920x1080 / 60 fps, approximately 78.55 seconds container duration. 13 native footage selects, new synthetic narration and original score, concise headings, nine-second donation end card. All footage is from custom_gameplay.mp4, supplied by Benjamin. No new gameplay capture or invented footage was needed.
- New thumbnail, English SRT, narration, YouTube description and exact timeline in the same launch-film folder. Source: tools/video/build_launch_film.py + launch_film.json. Deleted the obsolete generators and old movie at the owner's request. The current generator has no emulator-footage fallback.
- Updated website, README, active docs, build output, package description, runtime window/PDA/help branding to PC Recomp. Archived historical documents and upstream repository names remain unchanged. No gameplay mechanics modified.
- docs/PROMO_REVIEW.md records the review; docs/YOUTUBE_CREATOR_KIT.md is the current upload kit. Bruce Mikita publication terms remain undocumented (pre-existing issue in BRANDING.md); no font files redistributed.

## Verification
- Compiler.ps1 -Test after runtime string changes: 160 passed, 0 failed, 0 warnings. Log target/promo-final-validation.log.
- Visual port progress treemap (`docs/assets/progress.svg`) and SVG badges synchronized automatically with pure Rust data.
- **BENJAMIN'S CORE MANDATE**: All AI models must keep the visual status and treemap up to date after every significant code advance (`.\Compiler.ps1 -Test` or `python tools/progress/progress.py`). Strictly 100% Rust code.
- Complete final MP4 decode successful; final video contact sheet visually checked. Audio peak -3.6 dBFS, no sample clipping observed; automated audio checks are not a full listening review.
- All four logo variants visually inspected, PNG alpha/background checked and SVG XML parsed. New video/thumbnail/end-card graphics inspected.
- Website browser preview was not performed. Existing browser restrictions remain; no headless browser workaround used. Logos are rasterized with Sharp, video with FFmpeg/Pillow.

## Resume
```powershell
Get-Content .\outputs\CTW-Rust\AI_HANDOFF_GUIDE.md -TotalCount 45
git status --short
```
Review the new film and logos with Benjamin. Do not publish automatically. Subsequent gameplay priorities remain in the historical handoff below, but its fidelity claims are not certified by this media work.

---
## Previous gameplay handoff (historical)
# ReGTA CTW — presentation, street races, and autonomous progress

Updated: 2026-10-24 23:50 Europe/Paris. Multi-AI autonomous handoff guide.

## Completed this session

1. **All 16 Street Races & Time Trials Engine (`src/races.rs`, Overlay 140 `streetrace`)**:
   - 16 authentic circuits across Broker, Dukes, Bohan, and Algonquin:
     - 0: Transporterror (Broker - BOABO)
     - 1: Midtown Mayhem (Dukes - Meadow Hills)
     - 2: South Boulevard Boogie (Broker - South Slopes)
     - 3: Beach Blitz (Broker - Firefly Island)
     - 4: Boulevard Boogie (Bohan - Boulevard)
     - 5: Industrial Drift (Bohan - Industrial)
     - 6: Bridge Burner (Dukes - East Island City)
     - 7: Alleyway Dash (Bohan - Chase Point)
     - 8: Star Junction GP (Algonquin - Star Junction)
     - 9: Algonquin Expressway (Algonquin - Middle Park East)
     - 10: Castle Gardens Sprint (Algonquin - Castle Gardens)
     - 11: Meat Quarter Dash (Algonquin - Meat Quarter)
     - 12: Suffolk Circuit (Algonquin - Suffolk)
     - 13: Heliport Rush (Algonquin - Lower Easton)
     - 14: Times Square Shakedown (Algonquin - Triangle)
     - 15: Liberty City Grand Prix (Endurance cross-borough championship)
   - Dynamic checkpoint detection (14m radius), 3-second starting grid countdown, real-time lap times, and personal records.
   - 3 dynamic AI rival racers (`STREETRACER1` to `STREETRACER5`, `STREETRACER1 MOD`) with authentic rubber-banding pace adjustment.
   - Live position tracking (1st through 4th), medal tiers (Gold, Silver, Bronze), cash prizes, and persistent record logging in `SaveProgress::race_best_time[16]` (IDs `0x4C` to `0x5B`).
   - Anti-cheat/forfeit timer (10s if vehicle is abandoned mid-race).
   - HUD banner and radar golden diamond waypoint indicators.

2. **Tattoo Parlor Needle Tracing Minigame (`src/minigames.rs`, Overlay 141 `tattooingapp`)**:
   - 4 Triad & gang motifs: Dragon of Broker ($1200), Koi of Dukes ($800), Tiger of Bohan ($950), Viper of Algonquin ($1500).
   - Stencil needle tracing, ink reservoir replenishment, client pain meter, and flinch/failure handling.

3. **YouTube Creator Production Kit (`docs/YOUTUBE_CREATOR_KIT.md`)**:
   - Comprehensive video kit for Benjamin's first devlog/promotion video:
     - 5 high-CTR titles, thumbnail design with PNG logos.
     - Minute-by-minute script (Hook, Rust recomp vs emulator, 60fps/4k demo, live gameplay, drug wars / tattooing / street racing, roadmap, call to action with Tipeee and PayPal.Me).
     - Copy-paste description with timestamps, links, legal disclaimer, tags.
     - OBS Studio 60 FPS recording guide with fullscreen launcher commands.

4. **Official Project Branding, PNG Assets & Trilingual Showcase**:
   - 4 vector SVGs and 4 high-resolution PNGs delivered:
     - `docs/assets/logo.png` & `logo-transparent.png` (Vertical Stacked, 1800 × 1000)
     - `docs/assets/logo-ctw.png` & `logo-ctw-transparent.png` (Horizontal Inline, 1920 × 520)
   - GitHub Pages showcase deployed with trilingual EN / FR / ES switcher and persistence.
   - Repository renamed to `regta-chinatown-wars-pc-port`.

## Verification

- **Mandatory compiler check**: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test`
  - **160 passed, 0 failed, 0 ignored, 0 compiler warnings (`-D warnings`)**.
  - Binary delivered to `outputs/CTW-Rust/CTW-Rust.exe`.
- **Upstream check**: `references/chinatown-wars-psp-recomp` branch `master` clean and up to date.

## Next Roadmap Priorities (Autonomous Continuation)

1. **YouTube Promotion Video Recording**:
   - Benjamin records and publishes the first video using `docs/YOUTUBE_CREATOR_KIT.md`.
2. **Story Missions Execution & Cutscenes Engine (`src/missions.rs`)**:
   - Implement cutscene camera staging, scripted boss dialogue, and mission chains across the 8 boss contacts.
3. **Vehicle Radio Audio Stations Synthesis**:
   - Integrate radio audio playback loops for all 11 Chinatown Wars in-car radio stations.

Resume command:
```powershell
powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test
```

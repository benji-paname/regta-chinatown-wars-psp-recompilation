# ReGTA CTW — C++20 mainline instructions

## Canonical implementation (Benjamin's decision, 2026-11-02)

Work on `runtime/` by default: native C++20 PSP Europe ULES-01347 v1.01 recompilation for Windows x64. Do not restart gameplay reconstruction in Rust. `legacy/rust/src/` is the preserved historical implementation; its archive branch is `legacy/rust-pre-cpp-migration`. Root `Compiler.ps1` now invokes C++.

Benjamin reports completing the first two story missions in the private C++ Playtest using normal controls. This is a user playtest report, not exhaustive campaign validation. Drops toward 20 FPS and irregular frametimes in dense traffic remain a priority.

## Before every task

Run `python tools/maintenance/check_upstreams.py --task "Description"` from the root. Review BOTH official-kryo-to/ctw-native (Android reference) and highcmd/chinatown-wars-psp-recomp (PSP USA reference), then record relevant findings in `docs/UPSTREAM_REVIEW.md`. A fetch alone is not review. If DNS/network fails, record it; connected GitHub reads may independently verify remote revisions. Do not import upstream changes automatically. Mirrors must have their own .git and expected origin. Never fetch from `work/upstream-ctw`.

## Absolute private-data preservation

Keep ALL original game files and local Playtest data on Benjamin's PC, including ignored files, extracted data, textures, videos, music, executables and saves. Never delete these as cleanup, overwrite saves/preferences during updates, or force a locked directory rename. Reference mirrors, ISO/ELF and Ghidra projects remain untouched. Preserve uncommitted changes before reorganizing.

Publish NO proprietary game content: no ISO, PSP executable, generated/recompiled game translation units, extracted resources, texture packs, game audio/video, saves or test captures. All captures stay local per Benjamin's explicit instruction. Licensed project wordmarks may remain public; no font files. `work/`, `outputs/ReGTA-CTW-Playtest/`, and private build recipes stay ignored. `.gitignore` does not protect tracked files: check the index/tree AND outgoing Git objects before every push. Remove private files from tracking only, never from disk. Do not rewrite history destructively.

Publication gate: `python tools/maintenance/check_publication.py --tree HEAD --outgoing`. Install the versioned pre-push hook with `git config core.hooksPath .githooks`. Review actual content/provenance as well; an extension filter is not legal clearance. No commit/push unless authorized by the current user mission. The official repository and Pages URLs must not be renamed.

## Commit attribution

Benjamin's account is `benji-paname`. For his authorized AI-assisted work in this checkout, preserve the repository-local author identity `Benji Paname <324261208+benji-paname@users.noreply.github.com>`. Do not replace it with fictitious `@local` / `@localhost` addresses or publish his personal email. Verify `git var GIT_AUTHOR_IDENT` before an authorized commit. Other contributors keep their own identities. `.mailmap` maps only the historical project-local aliases; it does not rewrite old commits or establish GitHub credit by itself. Never force-push/rewrite history to adjust attribution without explicit authorization.

## Active layout and commands

- `runtime/src/`, `runtime/vfpu/`, `runtime/sas/`: canonical licensed runtime, provenance in `runtime/provenance.json` and `THIRD_PARTY.md`.
- `tests/kernel/mutex_scheduler.cpp`: 35-check real-runtime fixture; no game boot.
- `tools/build/cpp.py`, `tools/build/Cpp.cmd`, root `Compiler.ps1`: validated Clang 22.1.3/MSVC x64 C++20 build. No fast-math, no silent ABI/flag changes.
- `work/cpp-mainline/build-config.json`: private relocatable local recipe, generated from existing evidence, not a distributable game package.
- `work/cpp-recomp-feasibility/build/gen/`: 250 PRIVATE generated units; do not upload them or regenerate unnecessarily. Earlier reports/objects remain evidence.
- `outputs/ReGTA-CTW-Playtest/ReGTA-CTW-Playtest.exe`: normal local launcher; `ctw_game.exe`, image, DLLs, ms0 and ctw.ini stay together.
- `tools/update_playtest.py`: explicit local update; preserve saves, preferences and previous binary.
- `legacy/rust/src/`: byte-preserved Rust history. Old Cargo manifest remains under outputs/CTW-Rust for compatibility.

Build/check: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Check`.
Compile 29 runtime units, link locally and test: `powershell -ExecutionPolicy Bypass -File .\Compiler.ps1 -Test`.
This does not launch the game or replace the Playtest. `-FullGame` additionally recompiles the 250 private units only when needed. See `docs/BUILD_CPP.md` for local prerequisites; do not install/download dependencies automatically.

Keep compiler diagnostics. Current C++ builds have inherited warnings: do not claim zero warnings. Do not equate passing 35 tests with full PSP fidelity. Bink owner-exit cleanup and mutex/mailbox FIFO/priority selection are integrated; preserve them. USA hooks/cheats not verified for Europe must not be activated or remapped by guessed offsets. No fake HLE stubs or undocumented fallback changes.

## Status, website and handoff

`python tools/progress/cpp_status.py` maintains **playable recompilation readiness** as the primary status. **Benjamin's corrected definition: 100% = the recompilation is finished and playable across the accepted original-game scope.** Never display 99.99% generated-code coverage, HLE source bindings, passing test counts or arbitrary equal-weight milestone fractions as overall completion. No reliable global intermediate percentage is currently established: `completion_percent` stays null. `docs/PLAYABLE_VALIDATION.json` is the evidence ledger. `tools/progress/playable_status.py` permits 100 only after all documented gates pass, scope and completion are accepted, no listed blocker remains and native translation checks are complete.

Keep the visual status map prominent and visible by default in README and on the site, before long explanations. `cpp-status-map.svg` (also exported as cpp-progress.svg) shows each system's bounded observations in green and its remaining work in amber/grey; green subcells never imply whole-system completion. Update the observed/remaining fields in PLAYABLE_VALIDATION.json only from evidence.

Keep the primary `cpp-validation-map.svg` mosaic visible by default in README/site. It tracks scenario-level evidence in PLAYABLE_VALIDATION.json, not generated routines: green = observed/tested in the named scope; orange = partial; red = work or validation open. Green needs a source. Red does not imply missing implementation. This expanding checklist is not the complete PSP inventory, has no overall percentage and is not effort-weighted. Update it after real progress. The near-full generated-code `cpp-code-map.svg` belongs only in technical diagnostics; never use it as the main progress visual.

Primary `cpp-progress.svg/json` and `badge-playable.svg` now describe readiness qualitatively; amber/grey gates are not weighted percentage points. The green routine treemap and both numeric badges remain secondary **static code-generation diagnostics** in `CPP_CODEGEN.md` and `cpp-codegen-progress.svg/json`. Their original-byte-size arithmetic is similar to Kryo.to's, but generated-body/object presence differs from Kryo.to's source-name porting criterion, so the percentages are not comparable completion scores. The older Rust figure is also not a comparable baseline. `cpp_recomp_progress.py` checks generated bodies and pinned AMD64 objects; it does not certify behavior.

HLE source bindings (203/252 = 80.56%) remain another diagnostic in `CPP_IMPORTS.md`/`cpp-imports.svg`. Never hand-edit generated SVGs. Rust's historical generator writes only `docs/legacy/`.

Public name: ReGTA CTW — Native PC Recomp; credit: by Benji Paname. README English; site English/French/Spanish. Keep translations consistent. No public download until redistribution rights/package are verified. Preserve GitHub, Discord and support links. Owner reports and automated test results must be separately labelled.

Read/update `docs/AI_HANDOFF.md` and `docs/CPP_MAINLINE_MIGRATION.md`. Historical reports can overstate fidelity: current evidence takes precedence. Preserve private reports rather than rewriting them. A new game run needs current authorization and a bounded supervisor; never reuse consumed-session markers.

## Ghidra

Use the existing CTW_Working project only with `-noanalysis -readOnly`, exact European function entries, and compare decompiler output with Allegrex instructions. No project writes, memory dumps, permission bypass or guessed USA-to-Europe offsets. See `docs/REVERSE_ENGINEERING.md` for reference paths.


## Règle permanente — Mise à jour du GitHub et du site officiel

À partir de maintenant, chaque avancée majeure et validée du projet ReGTA Chinatown Wars doit également être documentée sur le GitHub et le site officiel.

### 1. Quand effectuer une mise à jour ?
Lorsqu'un jalon important est terminé et validé, notamment :
- Nouvelle fonctionnalité majeure.
- Correction importante du moteur C++.
- Amélioration significative des performances.
- Nouvelle option graphique.
- Compatibilité avec davantage de missions.
- Intégration de fonctionnalités DS/Android.
- Nouvelle version du Playtest.
- Amélioration importante de la stabilité.

**Important** : Ne pas publier une fonctionnalité comme opérationnelle si elle n'a réussi que des tests unitaires et nécessite encore une validation en jeu.

### 2. GitHub
Maintenir à jour :
- README.md
- La feuille de route (Roadmap).
- Le journal des modifications (CHANGELOG / releases).
- Les rapports de progression publics (docs/CPP_PROGRESS.md, docs/STATUS.md).
- La documentation technique utile aux contributeurs (docs/BUILD_CPP.md, CONTRIBUTING.md).

Vérifier l'URL et la branche officielles du dépôt (https://github.com/benji-paname/regta-chinatown-wars-pc-port.git, branche main) avant toute intervention.

### 3. Site officiel (docs/index.html)
Mettre à jour les trois langues existantes :
- Anglais (data-en).
- Français (data-fr).
- Espagnol (data-es).

Présenter les nouveautés de manière claire, professionnelle et accessible.
Afficher distinctement :
- Fonctionnalités opérationnelles.
- Fonctionnalités expérimentales.
- Fonctionnalités prévues.

### 4. Avancées déjà validées
La documentation publique doit notamment refléter les résultats des tests personnels de Benjamin :
- Cinq missions principales terminées.
- Une activité secondaire de taxi effectuée.
- Gameplay jugé fidèle à la PSP sur les activités testées.
- Sauvegarde et rechargement de la progression fonctionnels.
- Problèmes de performances, d'audio et de rendu encore en cours de correction.

Ne pas présenter ces résultats comme une validation de l'intégralité du jeu.

### 5. Sécurité et confidentialité
Ne jamais publier :
- ISO ou données commerciales du jeu.
- Sauvegardes personnelles (ms0/PSP/SAVEDATA/).
- Textures extraites ou modifiées sans droits de redistribution vérifiés.
- Fichiers locaux privés ou recettes privées de build.
- Identifiants, secrets ou chemins personnels locaux (C:\Users\...).

Toujours valider la publication avec : python tools/maintenance/check_publication.py --tree HEAD --outgoing.

### 6. Processus de publication
Après chaque jalon majeur :
1. Vérifier les résultats.
2. Préparer les modifications GitHub et site.
3. Présenter un récapitulatif à Benjamin.
4. Après validation, effectuer le commit, le push et le déploiement.
5. Vérifier que GitHub et les trois versions linguistiques du site sont correctement actualisés.

**Important pour le Jalon 2C : conserver l'interdiction de commit et de push tant que le moteur unifié n'a pas été validé en jeu.**

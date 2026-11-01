# C++ mainline migration — 2026-11-02 (development milestone)

- Make native C++20 PSP Europe/Windows x64 the canonical implementation; retain the Rust implementation and archive history.
- Promote the integrated Bink and mutex/mailbox FIFO-priority runtime; rebuild29 runtime units and pass35 covered kernel checks.
- Record Benjamin's completion of the first two story missions as a user playtest, with20FPS/frametime issues still open.
- Separate private game content and Playtest data from public licensed source; prohibit publication of test captures.
- Refresh English/French/Spanish presentation without an invented completion percentage or public download.
- Root directory rename remains blocked by a Windows lock; see migration report for actual publication status.

No new semantic version is invented for this migration. The local package retains1.0-Playtest; old Rust versions below are historical.

---

# Changelog

## Unreleased â€” 25 October 2026

- Removed superseded logo/video generators, old devlog/thumbnail and intermediate media at the owner's request; preserved current branding, film and source capture.

- Public name updated to **ReGTA CTW PC Recomp** throughout current website, documentation, logo outputs, build messages and runtime title/help/PDA text. PSP remains the original-data target. Existing executable paths and upstream reference names are unchanged.
- Rebuilt horizontal/vertical wordmarks with a clearly lowercase e, flat white ReGTA and red/yellow CTW; transparent and black-background SVG/PNG variants.
- New 1:18.51 English first-look film, 13 native gameplay selects, new narration and score, timed subtitles, thumbnail and donation end card. Replaced the promotional copy with qualified prototype claims; retired the emulator-fallback video generator.


Changes are recorded by release. The project is in the alpha phase; a version number does not imply complete PSP behaviour.

## [0.1.4-alpha] â€” 2026-10-24

### Added

- **Moteur des 16 Courses de Rue et Contre-la-Montre (`src/races.rs`, `streetrace` Overlay 140)** :
  - IntÃ©gration complÃ¨te des 16 circuits officiels de Chinatown Wars rÃ©partis sur Broker, Dukes, Bohan et Algonquin (Transporterror, Midtown Mayhem, South Boulevard Boogie, Beach Blitz, Boulevard Boogie, Industrial Drift, Bridge Burner, Alleyway Dash, Star Junction GP, Algonquin Expressway, Castle Gardens Sprint, Meat Quarter Dash, Suffolk Circuit, Heliport Rush, Times Square Shakedown, et le Grand Prix d'endurance de Liberty City).
  - DÃ©tection dynamique des checkpoints en coordonnÃ©es mondiales (rayon 14m), dÃ©compte de dÃ©part authentique 3.. 2.. 1.. GO, calcul en temps rÃ©el des temps au tour et du meilleur tour.
  - Concurrents rivaux IA avec modÃ¨les de vÃ©hicules de course officiels (`STREETRACER1` Ã  `STREETRACER5`, `STREETRACER1 MOD`) et mÃ©canique de rubber-banding authentique.
  - Calcul dynamique de la position en course (1er Ã  4e), seuils de mÃ©dailles (Bronze, Argent, Or), rÃ©compenses en cash et enregistrement du record personnel dans `save_progress.race_best_time[16]` (IDs `0x4C` Ã  `0x5B`).
  - SÃ©curitÃ© anti-triche et abandon : chrono de 10 secondes si Huang quitte son vÃ©hicule en pleine course.
  - Affichage radar du checkpoint actuel en or impÃ©rial et HUD de course en temps rÃ©el (Circuit, Tour, Position, ChronomÃ¨tre).
- **Minijeu de Salon de Tatouage (`src/minigames.rs`, `tattooingapp` Overlay 141)** :
  - 4 motifs officiels de la pÃ¨gre chinoise : Dragon de Broker ($1200), Carpe KoÃ¯ de Dukes ($800), Tigre de Bohan ($950), VipÃ¨re d'Algonquin ($1500).
  - MÃ©canique de traÃ§age d'aiguille le long du pochoir avec rÃ©approvisionnement en encre et jauge de douleur du client (Ã©chec si la douleur atteint 100% lors d'un dÃ©rapage).
- **Kit CrÃ©ateur YouTube Officiel (`docs/YOUTUBE_CREATOR_KIT.md`)** :
  - Guide complet pour Benjamin : titres Ã  fort taux de clic, concepts de miniatures avec logos officiels, script minute par minute, description prÃªte Ã  copier avec horodatages, liens de dons (Tipeee et PayPal.Me) et avertissements lÃ©gaux, guide de capture OBS 60 FPS en plein Ã©cran.
- **Refonte officielle de l'identitÃ© visuelle et des logos ReGTA CTW (`docs/assets/logo.svg`, `docs/assets/logo-ctw.svg`, `docs/BRANDING.md`)**:
  - Typographie Pricedown Black authentique pour `ReGTA` : le "Re" en grand intÃ©grÃ© avec "GTA", avec le 'e' en minuscule. Coloris deux tons : "Re" en dÃ©gradÃ© rouge Recomp (`#FF3B30` â†’ `#E52521` â†’ `#7F1D1D`) et "GTA" en blanc/platine pur GTA (`#FFFFFF` â†’ `#CBD5E1`) sous contour noir massif et ombre portÃ©e.
  - Typographie Bruce Mikita pour `CTW` avec les coloris exacts du jeu original Chinatown Wars : corps rouge laquÃ© impÃ©rial (`#F82B2B` â†’ `#DC1414` â†’ `#7A0808`), liserÃ© dorÃ© jaune vif (`#FFF875` â†’ `#FFD700` â†’ `#D97706`), et rehauts calligraphiques intÃ©rieurs.
  - Deux versions officielles vectorielles : Version 1 horizontale (`logo-ctw.svg`, 960x260) pour les banniÃ¨res et barres de navigation, et Version 2 verticale empilÃ©e (`logo.svg`, 900x480) avec le CTW positionnÃ© proprement sous le ReGTA pour la vitrine principale.
  - Validation visuelle directe par rendu headless Microsoft Edge.
- **Interactive PDA Contacts and Address Book (`src/pda.rs`, `contactsapp` Overlay 014)**:
  - IntÃ©gration du carnet de contacts des 8 boss d'histoire (Kenny, Chan, Zhou, Hsin, Heston, Lester, Rudy, Xin Shan) avec leurs districts, affiliations de gang et numÃ©ros de tÃ©lÃ©phone.
  - SÃ©lection interactive et routage GPS automatique vers le point de rencontre du contact.
- **Minijeu de PiÃ©geage de VÃ©hicule (`src/minigames.rs`, `carbombfittingapp` Overlay 010)**:
  - Minijeu d'installation et de cÃ¢blage d'un engin explosif sous le capot d'une voiture (cÃ¢ble d'alimentation, masse et dÃ©tonateur) avec amorÃ§age pour explosion au dÃ©marrage.
- **Odd Jobs and Emergency Services Engine (`src/jobs.rs`)**:
  - `taxi` (Overlay 143): Chauffeur de taxi avec dÃ©tection du vÃ©hicule Cabbie, passagers sur trottoir, destinations dynamiques, tarifs calculÃ©s selon distance et pourboire dÃ©gressif, streaks et dÃ©blocage du mÃ©daillon de taxi (boost nitro).
  - `vigilante` (Overlay 152): AutodÃ©fense via l'ordinateur de police en patrouille (5 vagues, Ã©limination de criminels en fuite, rÃ©compense gilet pare-balles doublÃ© 200 AP).
  - `firefighter` (Overlay 028): Interventions d'urgence en camion de pompiers, extinction des brasiers Ã  la lance Ã  eau, dÃ©blocage de l'immunitÃ© totale au feu (Huang ignifugÃ©).
  - `paramedicapp` (Overlay 094): Secours en ambulance vers les urgences de Schottler, transport de blessÃ©s et dÃ©blocage du sprint infini.
  - `chinesedelivery` (Overlay 013): TournÃ©es de livraisons express de nouilles Ã  scooter avec minuterie et pourboires.
- **Collectibles and World Challenges Engine (`src/collectibles.rs`)**:
  - 100 CamÃ©ras de sÃ©curitÃ© LCPD rÃ©parties sur Liberty City, destructibles par balles et explosifs, synchronisÃ©es avec `save_progress.package[100]`, accordant jusqu'Ã  20% de remise chez les dealers.
  - 30 Sauts uniques USJ (`save_progress.unique_jump[30]`) avec dÃ©tection de rampe, phase aÃ©rienne et validation d'atterrissage sur roues.
  - 35 Carnages / Rampages (`rampage`, Overlay 116) avec munitions infinies et mÃ©dailles de bronze/argent/or dans `save_progress.ambient_medal`.
  - 2 Statues sacrÃ©es des Lions de Fo (`monumentapp`, Overlay 091).
  - Bennes Ã  ordures rouges (armes cachÃ©es) et vertes (nourriture, cash et bric-Ã -brac) fouillables.
- **SynthÃ©tiseur et DÃ©clencheurs Audio DÃ©diÃ©s (`src/audio.rs`)**:
  - Bruitage de caisse enregistreuse et transaction (`play_cash_register`).
  - Carillon de notification email et alerte cours PDA (`play_pda_notification`).
  - Craquement et bris de pare-brise sous-marin (`play_glass_break`).
  - Gazouillis radio de dispatch de police (`play_police_dispatch`).
  - Choc Ã©lectrique de dÃ©fibrillateur (`play_defibrillator`).
  - Fanfare triomphale de mission et job rÃ©ussi (`play_mission_fanfare`).
- **HUD & Radar Live Tracking (`src/gpu.rs`)**:
  - Balises radar contextuelles pour les jobs actifs (passagers 'P', suspects 'V', incendies 'F', ambulances '+', livraisons 'D').

### Validation

- **160/160 unit tests passing**, 0 failed, 0 warnings under `-D warnings`.
- Binaire compilÃ© et actualisÃ© sous `outputs/CTW-Rust/CTW-Rust.exe`.

## [0.1.3-alpha] â€” 2026-10-24

### Added

- **PSP Code Overlays Engine (`src/overlays.rs`)**: Automatic detection, indexing, and decompression of all 166 code overlays stored inside `OVERLAYS.PAK` (20KP / 10SA container format) into PSP memory Slot 0 (0x08b87460, 114 mission scripts) and Slot 1 (0x08bb2480, 52 minigames & PDA apps).
- **Story Missions Engine (`src/missions.rs`)**: State machine orchestrating story missions across the 8 boss contacts (Uncle Kenny, Chan Jaoming, Zhou Ming, Hsin Jaoming, Wade Heston, Lester Leroc, Rudy D'Avanzo, Xin Shan). Waypoint guidance, mission briefing banners, objectives tracking, and cash rewards.
- **Drug Dealing Economy (`src/drugs.rs`)**: Authentic 6 commodities (Acid, Coke, Downers, Ecstasy, Heroin, Weed), gang supply/demand wholesale dynamics (7 gangs), 81 street dealers tracked in `save_progress.dealer`, player 100-capacity bag, and police confiscation on arrest.
- **Safehouses & Real Estate (`src/safehouse.rs`)**: 21 purchasable properties across Liberty City, Save Bed (advances clock 6h, restores 127 health, commits savegame count), Stash Box (unlimited cash & drug storage immune to police), and Wardrobe.
- **Gameplay HUD & Radar Integration (`src/gpu.rs`, `src/actors.rs`, `src/play.rs`)**:
  - Live HUD cash counter in authentic bright green (`$00000500`).
  - Contextual interaction prompt bar (`[ E / Y ] MISSION : Wu 'Kenny' Lee`, `[ E / Y ] DEALER...`, `[ E / Y ] PLANQUE...`).
  - Announcement banners for mission pass/fail, safehouse saved, and dealer discovery.
  - Minimap radar and full city map markers: golden contact diamonds ('K', 'C', 'Z', etc.), green safehouses, cyan drug dealers, and flashing mission target waypoints.

### Validation

- 132/132 unit tests passing, 0 warnings with `-D warnings`.
- Native runtime loads all 166 code overlays from UMD ISO in 1.18 seconds.

## [0.1.2-alpha] â€” 2026-10-24

### Changed

- Renamed the project **ReGTA CTW**, full name **ReGTA CTW PC Recomp**; retained executable/launcher paths for compatibility.
- Initialized a source-only local Git repository with explicit asset/build exclusions.
- Centralized Rust sources under `src/` and introduced a root Cargo workspace.
- Preserved the existing compiler, executable name and demo launcher paths.
- Consolidated compilation into `target/`; compiler warnings now fail the build.
- Made all 119 unit tests runnable without proprietary tables; retained optional private reference checks.
- Resolved optional local tables relative to the executable, independent of the working directory.
- Added `CTW_ISO` as an optional ISO-path environment variable; `--iso` takes precedence.
- Organized technical documentation, archived historical notes and indexed preserved research scripts.
- Added module status, provenance, release procedure and Tipeee/PayPal.Me support links.

### Maintenance

- Removed only the three named regenerable Cargo caches and BMP captures with verified pixel-identical PNG copies. Exact local results are recorded in the restructuring report.
- Preserved research scripts, C++ reference sources, original game files and nonidentical captures.

### Validation

- 119 unit tests passed; 0 failures, 0 ignored tests, 0 compiler warnings.
- This release reorganizes and validates the project; it does not certify full PSP parity.
- Local release preparation only; no remote repository or public release was published by this operation.

## Earlier development

Earlier work predates the local Git baseline. Historical notes are retained under `docs/archive/` and `docs/reference/`; no dates or release tags have been invented for that period.

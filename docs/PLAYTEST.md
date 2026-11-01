# Local Windows Playtest

Open `outputs/ReGTA-CTW-Playtest/ReGTA-CTW-Playtest.exe` directly. No PowerShell command is required to play. The current launcher displays its own information window/console and starts the game; it is not a public installer.

Keep the complete existing folder. `ctw_game.exe`, its DLLs, private `image.bin` and `assets/` are required. The launcher looks for an ISO configured in `ctw.ini`, then beside itself, then at the repository-relative `work/Chinatown-Wars-PSP.iso`. Moving the repository as a whole preserves that relative lookup. Absolute custom ISO preferences must be inspected after any successful rename; no ISO should be copied over another.

Saves are in `ms0/PSP/SAVEDATA/`; preferences in `ctw.ini`; launcher logs in `logs/playtest.log` and game logs in `ctw.log`. Updating a binary must not overwrite saves, preferences or private assets. The original package and existing saves are retained during migration. Generated savestates are not interchangeable with all versions without explicit compatibility testing.

The launcher source is `tools/playtest/launcher.cpp`. Its current input mapping remains the existing one: left stick for movement; A/B/X/Y corresponding to PSP Cross/Circle/Square/Triangle; shoulder controls for L/R. Keyboard analog movement I/J/K/L, face buttons Z/X/A/S, shoulders Q/E, Enter/Backspace for Start/Select. This describes the existing mapping; no mapping was changed by migration.

Benjamin reports completing the first two missions with normal controls. Drops toward 20 FPS and uneven frametimes remain, especially in dense driving scenes. Controller/audio behavior in all hardware configurations is not certified.

Builds do not automatically replace this package. Run the updater explicitly only after reviewing the new build; it keeps a local backup of the old binary. **No public download is approved. No screenshots, video, ISO, extracted assets or saves from this folder may be pushed.**

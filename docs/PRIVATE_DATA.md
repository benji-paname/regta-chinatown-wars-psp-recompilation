# Private game data and publication policy

All original game files and required Playtest data stay intact on Benjamin's PC. Excluding a file from Git is not deleting it. Never use cleanup commands on ISO/ELF, extracted resources, images, audio/video, archives, generated game code, saves or user preferences. Preserve ignored files when moving the repository. Stop a directory rename if Windows reports a lock.

Public: licensed runtime source, authored tooling/tests, documentation and explicitly approved project branding. Private: original/extracted/generated game content, binaries without cleared redistribution rights, all local Playtest files, captures, save files and dependency packages.

Before every push, review both the complete proposed tree and all newly reachable objects:

```powershell
python tools/maintenance/check_publication.py --tree HEAD --outgoing
```

`.gitignore` only filters untracked additions. The versioned pre-push hook performs the same gate; GitHub validation checks the published tree as a second layer. Audit any already-tracked file independently. An unwanted tracked asset may be removed with `git rm --cached -- <path>` only after verifying that the local file stays intact. Never use plain `git rm` for private data.

Do not force-push or rewrite history to remove old published files without a separate approved plan. Removing a file from the current tree does not remove it from older commits. The old gameplay thumbnail is retained locally and excluded from the migrated current tree; its already-published history is documented in the migration report.

The gate is a technical filter, not an automatic license determination. Generated game C++ is private despite its text extension. Logo artwork is allowed only by reviewed path+SHA256; font files stay private. Benjamin confirmed logo permission on 2 November 2026 and explicitly prohibited publication of test captures.

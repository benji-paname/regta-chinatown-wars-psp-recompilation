# Contributing

Start with AGENTS.md, docs/AI_HANDOFF.md and docs/BUILD_CPP.md. C++20 is the active runtime; Rust is historical. Keep changes small and evidence-backed against PSP Europe ULES-01347 v1.01. Do not guess USA address offsets, bypass ownership checks, hide warnings or fabricate missing HLE success.

Run the relevant35-check kernel fixture for synchronization changes and retain full diagnostics. Boot tests require explicit bounded supervision and must preserve user saves. No full-game completion percentage should be inferred from test/function counts.

Never submit original game files, extracted/generated game code, capture media, saves, private build outputs or secrets. Check the complete proposed Git tree and outgoing blobs with tools/maintenance/check_publication.py. Maintain English/French/Spanish site parity, provenance and handoff notes. No public executable release is currently cleared.

## Maintainer and commit attribution

[Benji Paname (@benji-paname)](https://github.com/benji-paname) created and maintains ReGTA CTW, with AI-assisted development and the upstream credits documented in README and THIRD_PARTY.md.

Use an email associated with your own GitHub account when contributing. Benjamin's authorized work in this checkout uses his GitHub privacy address, `324261208+benji-paname@users.noreply.github.com`. Repository-local Git settings apply to future commits; `.mailmap` consolidates historical local aliases in compatible Git views without changing existing commit hashes. It does not guarantee retroactive GitHub graph attribution. Preserve other contributors' credits and the record of AI assistance. Never use a personal email or rewrite published history just to change a graph.

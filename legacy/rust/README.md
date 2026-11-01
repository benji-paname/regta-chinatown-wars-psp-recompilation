# Historical Rust implementation

This port is preserved for research and project history. **It is not the active engine.** The mainline is C++20 in `runtime/` at repository root.

Sources: `legacy/rust/src/`, preserved byte-for-byte during migration, including Benjamin's uncommitted actor/job changes. Cargo compatibility manifest: `outputs/CTW-Rust/Cargo.toml`. Explicit old builder: `tools/build/Compile.ps1 -Test`. Historical generated metrics: `docs/legacy/progress.html`; they do not measure the C++ game.

Local archive branch: `legacy/rust-pre-cpp-migration`, checkpoint `c9743b40845875b57bca8dbaf59098835e9d6874`. Private backup also includes untracked files and the original Git bundle. The archive branch is retained locally; do not publish it blindly because its historical tree includes the old gameplay thumbnail.

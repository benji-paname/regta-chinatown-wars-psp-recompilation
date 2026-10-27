# Gemini continuation — driving validation

Read `outputs/CTW-Rust/AI_HANDOFF_GUIDE.md` and `docs/DRIVING_TRAFFIC_REPAIR.md` first. The earlier `GEMINI_TRAFFIC_AUDIT_PROMPT.md` assignment is complete. Do not repeat that audit or replace the corrected drivetrain with empirical coefficients.

Start at the repository root and run the mandatory upstream preflight. Check Git status before editing: other agents may be active. Keep your changes isolated from any ongoing owner task. Preserve private game data and current logos/film.

Priority assignment: reproduce the three rail-world obstruction cases starting at `197→142`, `443→428`, and `626→1439` from `target/driving-audit/driving-audit.json`. Use the report's **ending** route and position to identify the actual collision. Determine whether the cause is route selection, corner geometry, map collision, or our simplified footprint. Inspect original data and pinned upstream algorithms; do not delete collision geometry, teleport cars, or allow land vehicles onto z=-7 boat routes to make the test pass.

Produce one small fix with a focused failing-before/passing-after regression, or an evidence report if the original behaviour is not established. Extend the diagnostic to test recovery after leaving the rail, then check continuous multi-vehicle queues/intersections. Keep all existing 213 Rust tests passing with zero warnings, and run the three Python inventory checks. Regenerate status with Compiler.ps1 -Test.

Separate research assignment if more time is available: locate the gear-ratio lookup or equivalent constants in PSP Europe ULES01347 1.01. Android values in `handling.rs` are explicitly provisional. Give exact addresses, disassembly and conversion evidence before substituting them. Never transplant USA PSP addresses blindly.

On handoff, update the active guide with exact validation results, commands, source revisions, remaining limitations and commit IDs. Do not claim 100% PSP fidelity or infer a completion percentage from function counts. Do not push to GitHub or contact upstream authors without Benjamin's authorization.

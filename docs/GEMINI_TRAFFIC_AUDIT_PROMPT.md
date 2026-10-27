# Gemini — independent driving / traffic evidence audit

> Completed assignment, retained for history. Current continuation: [GEMINI_DRIVING_NEXT.md](GEMINI_DRIVING_NEXT.md).

Benjamin authorizes you to assist ChatGPT on ReGTA CTW PC Recomp. Workspace:
`C:\Users\BENJAMIN\Documents\Codex\2026-10-17\tu-voit-le-projet-vcs-rust`.

ChatGPT owns the runtime fixes in `src/actors.rs`, `src/gps.rs`, `src/play.rs`,
`src/vehicle.rs` and a new handling module. **Do not edit these files, build,
commit, push, regenerate dashboards, or revert changes while this work runs.**

Your independent task is to audit the original traffic behaviour. Follow AGENTS.md;
run the upstream check with a task description, then read both upstream sources.
Pinned sources currently: CTW-Native `b0506abc786fea229101c56e0a64726aa8b4ac97`
(GPL-3.0-only), highcmd PSP recomp `6562163b6f504a1db93e1be5ce580d8cd6f024a7`.
PSP target is Europe ULES01347 v1.01; USA addresses are not interchangeable.

Read `traffic.cpp`, `trafficavoidance.cpp`, `aispline.cpp`, `roads.cpp` and
`driving.cpp` in `references/ctw-native/port/game`. Compare with `src/gps.rs` and
the traffic loop in `src/actors.rs`. Priorities:

1. Exact lane metadata selection, one-way vs two-way links, outgoing-node choice.
2. Junction curve construction, parameter/distance conversion, continuous handover.
3. Traffic-light phases and approach direction, queue braking and restarting;
   never treat a normal queue/red light as a reason to reverse into traffic.
4. Off-rail recovery after collision, route rejoining without snapping/teleporting.
5. Spawn/despawn visibility and multiple simultaneous spawn reservations.

Write only `docs/GEMINI_TRAFFIC_AUDIT.md` (English). For each confirmed defect give
our file/function, upstream file/function/line, concrete reproduction, algorithm
to adapt, and a useful regression test. Separate verified facts from assumptions.
If possible inspect the private PSP ai.bin and report representative node IDs for
straight roads, T-junctions, crossings, one-way streets and bridges. No game data
or binaries in the report; counts/coordinates/evidence locations are enough.
Do not claim PSP parity from Android code or from passing unit tests.

Conclude with the three highest-impact fixes and tell Benjamin the report is ready
for ChatGPT to read. This is an audit, not a second competing implementation.

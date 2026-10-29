# Europe PSP prop physics recovery — 29 October 2026

Target: ULES01347 1.01. Private ELF SHA256: 9eaf850f09d743e5ecfc8d1c6a0a8c290399142319b1ef3a732a03d7278bffe0. No extracted instructions or game data are distributed.

## Located routines

- Update path at 0x088a2dbc. Active flag at object+0x1e0 bit 0 gates damping.
- 0x088a2fb8..0x088a2ff0: integer coefficient 0xf3c applied to THREE velocity components at object+0x6c via helper 0x08aa3dd8.
- Helper 0x08aa3dd8..0x08aa3e30 multiplies signed words, extracts bits 12..43 with MULT/MFHI/MFLO/SRL/INS. This is signed Q12 multiplication, including negative rounding towards minus infinity and 32-bit result truncation.
- 0x088a2ff4 calls kind lookup 0x088a0e2c; byte 7 controls the planar branch. It clears angular x/y at +0xa0/+0xa4 and two orientation fields +0xf4/+0xf8, then damps angular z.
- Non-planar angular damping at 0x088a3064..0x088a30c4 requires byte +0x18e nonzero and byte +0x9b zero. The semantic names of these flags are NOT yet independently established. Do not label them contact/sleep with certainty.
- Gravity path 0x088a3148..0x088a31ac multiplies the mass at +0x15c by a vector at 0x08c1cc78, then invokes the virtual force method. Initializer/value of this PSP vector still needs recovery; Android's -29 is not proof.
- Candidate kinematics wrapper 0x088a1180..0x088a12bc calls 0x089b5960 and checks a squared speed via 0x08a631d8. Its cap passes 0xc8000 through conversion helper 0x088a0078. Do not copy Android's 0x59524 cap without completing that conversion.

## Completed validation and integration boundary

A test-only Rust transcription of the vector multiply in props.rs covers signed rounding, all axes, and signed integer extrema. This is algebraic verification against inspected instructions, NOT a live differential PSP oracle.

An attempted isolated runtime replacement caused the existing loose-mailbox settling regression to fail (vertical speed -1.1142578 after 300 steps). It was removed: mixing exact damping with the host gravity/bounce/stop ordering is not a valid complete port. Delivered ordinary gameplay stays at the previous stable physics. This checkpoint does not claim a new fidelity percentage.

## Next work, in dependency order

1. Trace 0x089b5960 (base kinematics), 0x088a0078 conversion, gravity initializer and the complete update ordering/call graph.
2. Recover PSP mass/inertia, force/torque accumulation, quaternion integration, world contact generation/response, sleep/contact flags and water transitions. Preserve fixed-point rounding and overflow at each operation.
3. Capture matching inputs and state outputs from the original PSP reference for contact-free motion, ground landing, wall impact, off-centre vehicle strikes and repeated impacts. Reference instrumentation is outside the shipped native runtime; no emulated CPU belongs in the game.
4. Integrate the coupled solver only when those comparisons and ordinary gameplay regressions pass. Scope parity by routines/scenarios, not by the total unit-test count.

Private disassembly: target/psp-prop-update.txt and target/prop-psp-functions.txt. Reproduce with work/psp-analysis/inspect_mips.py and the addresses above. The local tool uses the already established Europe ELF load mapping. Preserve private research and the existing playable build.


## 29 October — executable differential oracle and native force core

The first native force core is now in `src/psp_prop_physics.rs`, deliberately test-only until the complete integrator/contact path can replace the host solver together. This file must count as test/helper definitions, never delivered runtime functionality. The inventory generator now recognizes file-level `#![cfg(test)]` and has a regression test for that distinction.

`tools/research/psp_prop_oracle.py` loads only the hash-checked private Europe ELF into a separate Unicorn MIPS tool. Allegrex signed MADD (SPECIAL funct 0x1c) is translated in the private in-memory code copy to the standard MIPS32 signed MADD encoding; arithmetic HI:LO semantics are retained. No source game file is modified. This is an OFFLINE verification tool, not an emulated CPU in the Rust executable. It executes the original 0x089b5960 routine, including its original matrix/multiply helpers, over 512 deterministic synthetic states (positive/negative forces, non-diagonal inertia, zero mass, disabled bodies and empty accumulators). Rust matches all returned velocity/angular/acceleration/force/torque words exactly for this corpus. This is scoped differential evidence, not an exhaustive proof or whole-physics parity.

Recovered details that differ from naive ports:
- The PSP inverse-world-inertia matrix uses signed 16-bit entries and a column-major transform (0x08934888).
- Torque and force are consumed separately. With zero force the prior acceleration field is retained; with zero mass or disabled byte +0x9b set, neither accumulator is consumed.
- Each acceleration is multiplied by 0x88 in Q12 before wrapping addition to velocity. This is not a floating 1/30 step.
- Gravity initializer 0x089bf620..0x089bf658 sets 0x08c1cc78 to (0,0,-0x1d000), independently confirming -29 for PSP.
- Executing conversion helper 0x088a0078 on input 0xc8000 returns 0x59524. Android's cap is now independently confirmed for this PSP build.

Reproduce: install Unicorn into ignored `work/physics-oracle-deps`, run `python tools/research/psp_prop_oracle.py`, then `Compiler.ps1 -Test`. The optional private differential Rust test loads `target/psp-prop-oracle.json`; public runs without the ELF still exercise the fixed known-answer and accumulator tests but do not claim the 512-case comparison. Raw disassembly stays in ignored target/.

Remaining: recover quaternion/position integrator and its synchronization, full sphere/corner sweeps and spring contacts, force application at arbitrary contact points, world response and stop ordering. Keep these coupled; do not activate a partial replacement with the old bounce kernel. The requested 100% PSP physics remains unfinished.

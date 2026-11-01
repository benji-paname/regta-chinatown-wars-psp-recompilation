# ReGTA Chinatown Wars — Reverse Engineering & AI Tooling Guide (`docs/REVERSE_ENGINEERING.md`)

**Last Updated**: 2026-10-31  
**Target Binary**: PSP Europe `ULES-01347` v1.01 (`CTW-PSP.elf`, decrypted `EBOOT.BIN`)

This document describes the complete reverse-engineering, decompilation, and MCP tooling environment used to analyze the original PSP executable of *Grand Theft Auto: Chinatown Wars* and translate its exact mechanics into native Rust.

> **Security & Copyright Rule**: Never commit `CTW-PSP.elf`, `Chinatown-Wars-PSP.iso`, `.gpr`/`.rep` Ghidra project databases, or any API keys/tokens to Git.

---

## 1. Environment Overview

| Component | Version / Identifier | Local Path (Reference Machine) | Role |
|---|---|---|---|
| **Ghidra** | `12.2 PUBLIC` | `C:\Tools\ghidra_12.2_PUBLIC` (`support\analyzeHeadless.bat`) | Core static analysis & decompiler engine (runs on JDK `C:\Tools\jdk-25.0.5+8`) |
| **ghidra-allegrex** | `v21.5` (`Allegrex:LE:32:default`) | `C:\Tools\ghidra_12.2_PUBLIC\Ghidra\Extensions\ghidra-allegrex` | Sony PSP MIPS R4000 Allegrex + VFPU processor sleigh module & PSP ELF loader |
| **Analyzed Ghidra Project** | `CTW_Working.gpr` (`/CTW-PSP.elf`) | `C:\Tools\ghidra_projects\CTW_Working.gpr` (+ `CTW_Working.rep`) | Pre-analyzed PSP Europe binary database (read-only queries) |
| **Headless Query Script** | `CtwQuery.java` | `tools/ghidra/CtwQuery.java` & `C:\Tools\ghidra_scripts\CtwQuery.java` | Automated extraction of pseudocode, disassembly, callers, callees & data xrefs |
| **Decrypted PSP ELF** | `CTW-PSP.elf` (`ULES-01347` v1.01) | `work/psp-analysis/CTW-PSP.elf` (private, ignored) | Original PSP executable (`e_type = 0xffa0`, `e_machine = 0x0008`, 3,907,604 bytes) |
| **REA MCP Server** | `rea-agents@6.8.0` (Skill v49, 229 tools) | Registered in MCP client (`rea`) | Multi-target reverse engineering, binary triage, Rizin/objdump & artifact comparison |
| **FreeLLMAPI MCP Server** | `freellmapi` | Registered in MCP client (`freellmapi`) | Multi-provider LLM routing, quota/health diagnostics, and web search |
| **Unicorn PSP Oracle** | `psp_prop_oracle.py` | `tools/research/psp_prop_oracle.py` | Differential execution of original PSP MIPS routines against Rust |

---

## 2. Ghidra 12.2 + `ghidra-allegrex` v21.5 + `CTW_Working.gpr`

### A. Verified Program Metrics (`CTW_Working.gpr` -> `/CTW-PSP.elf`)
When queried via `CtwQuery.java`, the pre-analyzed project reports the following exact metrics:
- **Program Name**: `CTW-PSP.elf`
- **Executable Path**: `/C:/Users/BENJAMIN/Documents/Codex/2026-10-17/tu-voit-le-projet-vcs-rust/work/psp-analysis/CTW-PSP.elf`
- **Loader Format**: `PSP Executable (ELF)`
- **Language ID**: `Allegrex:LE:32:default` (Compiler Spec: `default`)
- **Image Base**: `0x08804040`
- **Total Functions**: **11,228**
- **Total Symbols**: **94,152**
- **Total Cross-References (Xrefs)**: **327,185**
- **VFPU Candidate 32-bit Words in `.text`** (via `@vfpu_scan`): **1,111**

### B. Memory Map of `CTW-PSP.elf` (`ULES-01347` v1.01)
| Block | Start | End | Size | Perms | Description |
|---|---|---|---|---|---|
| `.text` | `0x08804040` | `0x08b34ecf` | `0x330e90` | `RWX` | Main PSP game engine & Allegrex/VFPU code (~3.35 MB) |
| `.lib.ent` | `0x08b34ed0` | `0x08b35137` | `0x268` | `RWX` | PSP library export table |
| `.sceStub.text` | `0x08b35138` | `0x08b35937` | `0x800` | `RWX` | Sony PSP OS syscall stubs |
| `.cplinit` | `0x08b35938` | `0x08b36187` | `0x850` | `RWX` | C++ static initializer table |
| `.linkonce.d` | `0x08b36188` | `0x08b3c8a7` | `0x6720` | `RWX` | Link-once data / RTTI / template tables |
| `.rodata.sceModuleInfo` | `0x08b3c8a8` | `0x08b3c8db` | `0x34` | `RWX` | PSP module header (`CTW_PSP_UMD_MASTER_SCEE`) |
| `.rodata.sceResident` | `0x08b3c8dc` | `0x08b3cb0b` | `0x230` | `RWX` | Resident NID/export table |
| `.rodata.sceNid` | `0x08b3cb0c` | `0x08b3cf0b` | `0x400` | `RWX` | Imported syscall NID table |
| `.rodata` | `0x08b3cf10` | `0x08b52687` | `0x15778` | `RWX` | Read-only constants, strings, and C++ vtables |
| `.data` | `0x08b526c0` | `0x08b874d3` | `0x34e14` | `RWX` | Initialized global variables |
| `.psp_lib_mark_` | `0x08b874d4` | `0x08b87533` | `0x60` | `RWX` | PSP library marker |
| `.psp_lib_markimport_` | `0x08b87534` | `0x08b8754b` | `0x18` | `RWX` | PSP import marker |
| *(ELF `.overlays` slot)* | `0x08b8754c` | `0x08bc809f` | `0x40b54` | `---` | Unallocated 265 KB region reserved for dynamic overlays (Slot 0 & Slot 1) |
| `.eh_frame` | `0x08bc80a0` | `0x08bc818f` | `0xf0` | `RW-` | Exception handling frame header |
| `.ctors` / `.dtors` | `0x08bc8190` | `0x08bc819f` | `0x10` | `RW-` | Global constructor / destructor pointers |
| `.sdata` | `0x08bc81a0` | `0x08bc81af` | `0x10` | `RW-` | Small initialized data |
| `.sbss` | `0x08bc81b0` | `0x08bc83cb` | `0x21c` | `RW-` | Small uninitialized data |
| `.bss` | `0x08bc8400` | `0x08da3cc7` | `0x1db8c8` | `RW-` | Uninitialized globals & static pools (~1.95 MB) |

---

## 3. Using `CtwQuery.java` in Read-Only Mode

The script [`tools/ghidra/CtwQuery.java`](../tools/ghidra/CtwQuery.java) (also installed identically at `C:\Tools\ghidra_scripts\CtwQuery.java`) is our canonical bridge between AI coding agents and Ghidra 12.2.

### A. Standard Read-Only Query Command (PowerShell)
Always pass **`-noanalysis -readOnly`** so that `CTW_Working.gpr` is never modified or locked and repeated queries remain fast (~8–10 seconds):

```powershell
& "C:\Tools\ghidra_12.2_PUBLIC\support\analyzeHeadless.bat" `
  "C:\Tools\ghidra_projects" "CTW_Working" `
  -process "CTW-PSP.elf" `
  -noanalysis -readOnly `
  -scriptPath "C:\Users\BENJAMIN\Documents\Codex\2026-10-17\tu-voit-le-projet-vcs-rust\tools\ghidra" `
  -postScript CtwQuery.java 0x089c967c 0x089c7c50 0x089c7d08 0x089c8f20
```

### B. Supported `CtwQuery.java` Arguments
- **Hex addresses** (with or without `0x` prefix): e.g., `0x089c967c`, `089c8930`.
- **Function names**: e.g., `FUN_089c967c`.
- **`@vfpu_scan`**: Scans `.text` for Sony Allegrex VFPU vector instructions (`lv.s`, `lv.q`, `sv.s`, `sv.q`, `vadd`, `vmul`, `vtfm`, etc.) and prints summary statistics and sample instructions.

### C. Important Precaution: Finding Exact Function Entry Points First
If you pass an address that lies **in the middle of a function** (such as a call site returned by an xref search), `CtwQuery.java` invokes `CreateFunctionCmd` in memory, which splits the function at that address for that run and truncates the caller's decompilation.
To find the exact prologue (`addiu sp, sp, -N`, opcode `0x27bdffe0`..`0x27bdff00` or `jr ra` boundary) of any mid-function address `0x089xxxxx` before calling `CtwQuery.java`, run this 1-line read-only Python check on `work/psp-analysis/CTW-PSP.elf`:

```powershell
python -c "import struct; data=open(r'C:\Users\BENJAMIN\Documents\Codex\2026-10-17\tu-voit-le-projet-vcs-rust\work\psp-analysis\CTW-PSP.elf','rb').read(); base=0x08804000-0x40; [print(f'0x{a:08x}: 0x{struct.unpack(\"<I\",data[a-base:a-base+4])[0]:08x}') for a in range(0x089c8f00, 0x089c9020, 4) if (struct.unpack('<I',data[a-base:a-base+4])[0] & 0xffff0000)==0x27bd0000 and (struct.unpack('<I',data[a-base:a-base+4])[0] & 0x8000)!=0]"
```

---

## 4. Real-World Case Study: Decompiling PSP Double Armour (`0x089c967c`)

During the Double Armour repair (`src/actors.rs`, `src/jobs.rs`), we queried four related functions in `CTW_Working.gpr`:
- `0x089c967c` — `cPlayer::ReduceArmour`
- `0x089c7c50` — `cPed::AddArmour`
- `0x089c7d08` — `cPed::SetArmour`
- `0x089c8f20` — `cPlayer::FixDamageForArmour`

### A. Output Excerpt for `0x089c967c` (`cPlayer::ReduceArmour`)

#### 1. Incoming & Outgoing Cross-References
```text
Function Name      : FUN_089c967c
Entry Point        : 0x089c967c
Address Range      : 0x089c967c .. 0x089c9727 (172 bytes)

--- Incoming Calls (Callers: 1) ---
  <- FUN_089c8f20 @ 0x089c8f20
  [XrefTo Entry] from 0x089c8f70 (UNCONDITIONAL_CALL) in FUN_089c8f20@0x089c8f20

--- Outgoing Calls (Unique Callees: 1, Call Sites: 2) ---
  -> FUN_089c7d08 @ 0x089c7d08
  [0x089c9700] -> 0x089c7d08 (UNCONDITIONAL_CALL) FUN_089c7d08
  [0x089c971c] -> 0x089c7d08 (UNCONDITIONAL_CALL) FUN_089c7d08
```

#### 2. Allegrex Disassembly (`0x089c967c`)
```text
  0x089c967c: f0ffbd27  addiu sp,sp,-0x10
  0x089c9680: 0400b1af  sw s1,0x4(sp)
  0x089c9684: 140f8694  lhu a2,0xf14(a0)         # a2 = player->flags_f14
  0x089c9688: 21888000  move s1,a0               # s1 = this
  0x089c968c: 0800bfaf  sw ra,0x8(sp)
  0x089c9690: 0020c930  andi t1,a2,0x2000        # test bit 0x2000 (Double Armour)
  0x089c9694: 14002011  beq t1,zero,0x089c96e8   # if not active, skip to normal absorption
  0x089c9698: 47022792  _lbu a3,0x247(s1)        # (delay slot) a3 = this->armour
  0x089c969c: 0000b0af  sw s0,0x0(sp)
  0x089c96a0: 2120a000  move a0,a1               # a0 = damage
  0x089c96a4: 0200882c  sltiu t0,a0,0x2          # t0 = (damage < 2)
  0x089c96a8: 02000015  bnez t0,0x089c96b4       # if damage < 2, branch to 1-point alternation
  0x089c96ac: 0040c830  _andi t0,a2,0x4000       # (delay slot) t0 = old bit 0x4000
  0x089c96b0: 0d000010  b 0x089c96e8             # damage >= 2: jump to absorption
  0x089c96b4: 42200400  _srl a0,a0,0x1           # (delay slot) a0 = damage >> 1
  0x089c96b8: 0100082d  sltiu t0,t0,0x1          # invert bit 0x4000: (old == 0) ? 1 : 0
  0x089c96bc: ffbffff8  li a3,-0x4001            # mask ~0x4000 (0xbfff)
  0x089c96c0: 2430c700  and a2,a2,a3
  0x089c96c4: 80430800  sll t0,t0,0xe            # inverted << 14
  0x089c96c8: 2530c800  or a2,a2,t0
  0x089c96cc: 140f26a6  sh a2,0xf14(s1)          # store updated flags_f14
  0x089c96d0: 140f2686  lh a2,0xf14(s1)
  0x089c96dc: 0040c630  andi a2,a2,0x4000        # check new bit 0x4000
  0x089c96e0: 0a00c014  bne a2,zero,0x089c970c   # if new bit == 1, return 0 damage!
  0x089c96e4: 47022792  _lbu a3,0x247(s1)        # (delay slot) a3 = this->armour
  0x089c96e8: 2b30e400  sltu a2,a3,a0            # if (this->armour < adjusted_damage)
  0x089c96ec: 0900c014  bne a2,zero,0x089c9714   # branch to overflow path
  ...
  0x089c9700: 42f7270e  jal 0x089c7d08           # cPed::SetArmour(this, armour - adjusted_damage)
  0x089c9704: 21202002  _move a0,s1
  0x089c970c: 21100000  move v0,zero             # return 0
  ...
  0x089c9714: 23808700  subu s0,a0,a3            # overflow = adjusted_damage - armour
  0x089c9718: 21202002  move a0,s1
  0x089c971c: 42f7270e  jal 0x089c7d08           # cPed::SetArmour(this, 0)
  0x089c9720: 21280000  _move a1,zero
  0x089c9724: 21100002  move v0,s0               # return overflow
```

#### 3. Why Comparing Pseudocode and Assembly Is Mandatory
In the caller `0x089c8f20` (`cPlayer::FixDamageForArmour`), Ghidra's untyped decompiler signature defaulted to `void FUN_089c8f20(...)`, making the pseudocode end with `FUN_089c967c(param_1, param_2); return;`.
Looking at the assembly of `0x089c8f20` (`0x089c8f70: jal 0x089c967c ... 0x089c8f84: jr ra`) and its caller `cPed::Damage` (`0x089a1e78`) immediately proved that `$v0` from `ReduceArmour` is returned untouched through `0x089c8f20` to `cPed::Damage` and subtracted from health.
**Rule**: Never trust a `void` return or omitted delay-slot side effect in untyped decompiler output without checking the MIPS instructions printed right above it by `CtwQuery.java`.

---

## 5. REA 6.8.0, FreeLLMAPI & MCP Integration

### A. REA (`rea-agents@6.8.0`)
REA is registered as an MCP server (`rea`) providing 229 lazy-loaded reverse-engineering tools:
- **Binary & Archive Triage**: `open_binary`, `binary_overview`, `inspect_artifact`, `list_segments`, `list_procedures`, `list_strings`, `search_strings`, `search_bytes`, `read_bytes`.
- **Disassembly & Cross-References**: `inspect_with_objdump`, `execute_rizin_command`, `procedure_assembly`, `procedure_callees`, `procedure_callers`, `xrefs`.
- **Diffing & Verification**: `compare_artifacts`, `compare_functions`, `match_native_functions`, `correlate_static_and_runtime`.
- **Health / Registration Check**:
  ```powershell
  npx -y rea-agents@6.8.0 doctor --client antigravity --json
  ```
  *(Substitute `--client codex`, `claude_code`, `cursor`, `gemini_cli`, etc. when running from another AI client.)*

### B. Limitations of REA's Default PSP Profile
While REA is useful for archive inspection, string searches, byte reads, and Rizin/objdump queries, **do not rely on REA's generic MIPS profile for decompiling `CTW-PSP.elf`**:
1. `CTW-PSP.elf` uses the Sony PSP PRX ELF type (`e_type = 0xffa0`, `e_machine = 0x0008`).
2. Standard MIPS32 disassemblers/decompilers do not decode Sony's proprietary **Allegrex** instructions (`max`, `min`, `ext`, `ins`, `wsbh`, `wsbw`, `bitrev`) or **VFPU** coprocessor 2 vector instructions (`lv.q`, `sv.q`, `vtfm4.q`, `vmmul.q`, etc., 1,111 candidate words in `.text`).
3. Therefore, **always use our Ghidra 12.2 + `ghidra-allegrex` v21.5 + `CTW_Working.gpr` + `CtwQuery.java` pipeline** as the authoritative decompiler and disassembler for `CTW-PSP.elf`.

### C. FreeLLMAPI (`freellmapi`)
The `freellmapi` MCP server provides multi-model consultation and diagnostics:
- `healthcheck`, `provider_health`, `provider_list`, `quota_outlook`, `routing_info`, `usage_summary`, `cache_stats`.
- `list_models`, `ask_freellmapi` (for querying secondary LLMs when explicitly requested), and `web_search`.
- **Security Note**: Never copy any `freellmapi` URL credentials, tokens, or API keys from client MCP configurations into repository files.

### D. Using MCP Tools in Antigravity
In Antigravity, MCP tool schemas live under `<appDataDir>\mcp\<serverName>\<toolName>.json`:
1. Read the tool's JSON schema first (e.g., `C:\Users\BENJAMIN\.gemini\antigravity\mcp\rea\binary_overview.json`).
2. Invoke the tool via `call_mcp_tool` with `ServerName` (`"rea"` or `"freellmapi"`), `ToolName`, and `Arguments`.

---

## 6. Methodology: Turning PSP Evidence into Native Rust

Every gameplay or rendering fix in ReGTA CTW follows this 6-step workflow:

1. **Locate the Target Subsystem**:
   - Use class/method names or strings in `references/ctw-native` (`work/upstream-ctw/port/game/`) or `docs/reference/` to identify candidate concepts, vtable slots, or magic constants.
2. **Map to PSP Europe (`ULES-01347` v1.01) Addresses**:
   - Search strings, constants, or struct offsets (e.g., `+0x247` for `armour`, `+0xf14` for `cPlayer` flags) in `CTW_Working.gpr` or via a read-only Python scan on `work/psp-analysis/CTW-PSP.elf`.
3. **Decompile & Disassemble with `CtwQuery.java`**:
   - Run `analyzeHeadless.bat` with `-noanalysis -readOnly -postScript CtwQuery.java <address>`.
   - Inspect incoming callers, outgoing callees, data references, Allegrex disassembly, and C pseudocode.
4. **Verify Assembly Corner Cases**:
   - Check signed vs. unsigned loads (`lb` vs. `lbu`, `lh` vs. `lhu`), shift instructions (`srl` vs. `sra`), Q12 fixed-point scaling (`>> 12`), clamping (`sltiu` / `max` / `min`), branch delay slots, and `$v0` return registers.
5. **Implement Localized Native Rust**:
   - Update only the relevant Rust module in `src/`, documenting the exact PSP address (e.g., `/// PSP cPlayer::ReduceArmour (0x089c967c)`) in doc comments.
6. **Prove with Deterministic Unit Tests**:
   - Add unit tests in the module's `#[cfg(test)]` block covering nominal cases, boundary values, saturation ceilings, and state resets, then run `cargo test --release` with `-D warnings`.

---

## 7. Reproducing This Environment on Another Machine

To set up the complete ReGTA CTW reverse-engineering and build environment on a new Windows x64 workstation:

1. **Install Prerequisites**:
   - **Rust Toolchain**: Rust `1.90.0` (`x86_64-pc-windows-gnu` or `x86_64-pc-windows-msvc`).
   - **Java Development Kit**: OpenJDK 21+ (on our reference machine: `C:\Tools\jdk-25.0.5+8`).
   - **Python 3.11+**: For `tools/maintenance/check_upstreams.py`, `tools/progress/progress.py`, and research scripts.
2. **Install Ghidra 12.2 + `ghidra-allegrex` v21.5**:
   - Extract Ghidra `12.2 PUBLIC` to `C:\Tools\ghidra_12.2_PUBLIC`.
   - Install the `ghidra-allegrex` v21.5 extension into `C:\Tools\ghidra_12.2_PUBLIC\Ghidra\Extensions\ghidra-allegrex` (ensuring `Allegrex:LE:32:default` is available).
3. **Prepare the PSP Binary & Ghidra Project**:
   - Place your legally dumped European PSP ISO (`ULES-01347` v1.01) at `work/Chinatown-Wars-PSP.iso` and decrypted `EBOOT.BIN` at `work/psp-analysis/CTW-PSP.elf`.
   - Import `CTW-PSP.elf` into a Ghidra project `C:\Tools\ghidra_projects\CTW_Working.gpr` using language `Allegrex:LE:32:default`:
     ```powershell
     & "C:\Tools\ghidra_12.2_PUBLIC\support\analyzeHeadless.bat" `
       "C:\Tools\ghidra_projects" "CTW_Working" `
       -import "work\psp-analysis\CTW-PSP.elf" `
       -processor "Allegrex:LE:32:default"
     ```
   - Subsequent queries must always use `-process "CTW-PSP.elf" -noanalysis -readOnly -scriptPath "tools\ghidra" -postScript CtwQuery.java <addr>`.
4. **Optional MCP Servers (`rea` & `freellmapi`)**:
   - Register `rea-agents@6.8.0` for your AI client (`npx -y rea-agents@6.8.0 setup --client <client> --dry-run --json`).
   - Configure `freellmapi` in your local MCP client configuration using environment variables or local non-tracked config files.

import ghidra.app.script.GhidraScript;
import ghidra.app.decompiler.DecompInterface;
import ghidra.app.decompiler.DecompileOptions;
import ghidra.app.decompiler.DecompileResults;
import ghidra.app.cmd.disassemble.DisassembleCommand;
import ghidra.app.cmd.function.CreateFunctionCmd;
import ghidra.program.model.address.*;
import ghidra.program.model.listing.*;
import ghidra.program.model.mem.Memory;
import ghidra.program.model.mem.MemoryBlock;
import ghidra.program.model.symbol.*;
import java.util.*;

public class CtwQuery extends GhidraScript {

    @Override
    protected void run() throws Exception {
        println("=== GHIDRA 12.2 CTW-PSP DIAGNOSTIC & QUERY ===");
        println("Program Name    : " + currentProgram.getName());
        println("Executable Path : " + currentProgram.getExecutablePath());
        println("Loader Format   : " + currentProgram.getExecutableFormat());
        println("Language ID     : " + currentProgram.getLanguageID());
        println("Compiler Spec   : " + currentProgram.getCompilerSpec().getCompilerSpecID());
        println("Image Base      : " + currentProgram.getImageBase());

        FunctionManager fm = currentProgram.getFunctionManager();
        SymbolTable st = currentProgram.getSymbolTable();
        ReferenceManager rm = currentProgram.getReferenceManager();
        Listing listing = currentProgram.getListing();
        Memory mem = currentProgram.getMemory();

        println("Total Functions : " + fm.getFunctionCount());
        println("Total Symbols   : " + st.getNumSymbols());

        int refCount = 0;
        AddressIterator refIter = rm.getReferenceSourceIterator(mem, true);
        while (refIter.hasNext()) {
            Address src = refIter.next();
            refCount += rm.getReferencesFrom(src).length;
        }
        println("Total Xrefs     : " + refCount);

        println("--- Memory Blocks ---");
        for (MemoryBlock block : mem.getBlocks()) {
            println(String.format("  %-24s : %s .. %s (size=0x%x, perms=%s%s%s)",
                block.getName(),
                block.getStart(),
                block.getEnd(),
                block.getSize(),
                block.isRead() ? "R" : "-",
                block.isWrite() ? "W" : "-",
                block.isExecute() ? "X" : "-"
            ));
        }

        String[] args = getScriptArgs();
        if (args == null || args.length == 0) {
            println("=== NO TARGET FUNCTIONS REQUESTED ===");
            return;
        }

        DecompInterface decomp = new DecompInterface();
        DecompileOptions options = new DecompileOptions();
        decomp.setOptions(options);
        decomp.toggleCCode(true);
        decomp.toggleSyntaxTree(true);
        decomp.setSimplificationStyle("decompile");
        decomp.openProgram(currentProgram);

        for (String rawArg : args) {
            String arg = rawArg.trim();
            if (arg.isEmpty()) continue;

            if (arg.equalsIgnoreCase("@vfpu_scan")) {
                println("\n======================================================================");
                println("VFPU INSTRUCTION SCAN IN .text");
                println("======================================================================");
                MemoryBlock textBlock = mem.getBlock(".text");
                if (textBlock != null) {
                    Address cur = textBlock.getStart();
                    Address end = textBlock.getEnd();
                    int vfpuWords = 0;
                    List<String> samples = new ArrayList<>();
                    while (cur.compareTo(end) < 0) {
                        int word = mem.getInt(cur);
                        int op = (word >>> 26) & 0x3f;
                        // Allegrex VFPU major opcodes: 0x12 (COP2), 0x32 (lv.s), 0x36 (lv.q), 0x3a (sv.s), 0x3e (sv.q), 0x18, 0x34, 0x3c
                        if (op == 0x12 || op == 0x32 || op == 0x36 || op == 0x3a || op == 0x3e || op == 0x18 || op == 0x34 || op == 0x3c) {
                            vfpuWords++;
                            if (samples.size() < 15) {
                                Instruction ins = listing.getInstructionAt(cur);
                                if (ins == null) {
                                    DisassembleCommand dc = new DisassembleCommand(cur, new AddressSet(cur, cur.add(3)), false);
                                    dc.applyTo(currentProgram, monitor);
                                    ins = listing.getInstructionAt(cur);
                                }
                                Function f = fm.getFunctionContaining(cur);
                                samples.add(String.format("  0x%s (op=0x%02x, word=0x%08x) in %-16s : %s",
                                    cur, op, word, (f != null ? f.getName() + "@" + f.getEntryPoint() : "unassigned"),
                                    (ins != null ? ins.toString() : "unparsed")));
                            }
                        }
                        cur = cur.add(4);
                    }
                    println("Total VFPU candidate 32-bit words in .text: " + vfpuWords);
                    for (String s : samples) {
                        println(s);
                    }
                }
                continue;
            }

            println("\n======================================================================");
            println("QUERY TARGET: " + arg);
            println("======================================================================");

            Address addr = null;
            Function func = null;

            if (arg.startsWith("0x") || arg.startsWith("0X")) {
                addr = currentProgram.getAddressFactory().getDefaultAddressSpace().getAddress(arg);
            } else {
                try {
                    addr = currentProgram.getAddressFactory().getDefaultAddressSpace().getAddress("0x" + arg);
                } catch (Exception e) {
                    // Try by function name
                }
            }

            if (addr != null) {
                func = fm.getFunctionAt(addr);
                if (func == null) {
                    println("[INFO] Ensuring disassembly and function definition at 0x" + addr + "...");
                    DisassembleCommand disCmd = new DisassembleCommand(addr, null, true);
                    disCmd.applyTo(currentProgram, monitor);
                    CreateFunctionCmd fnCmd = new CreateFunctionCmd(addr);
                    fnCmd.applyTo(currentProgram, monitor);
                    func = fm.getFunctionAt(addr);
                    if (func == null) {
                        func = fm.getFunctionContaining(addr);
                    }
                }
            } else {
                List<Function> funcs = listing.getGlobalFunctions(arg);
                if (!funcs.isEmpty()) {
                    func = funcs.get(0);
                    addr = func.getEntryPoint();
                }
            }

            if (func == null) {
                println("[ERROR] Could not resolve function for: " + arg);
                continue;
            }

            Address entry = func.getEntryPoint();
            AddressSetView body = func.getBody();
            println("Function Name      : " + func.getName());
            println("Entry Point        : 0x" + entry.toString());
            println("Address Range      : 0x" + body.getMinAddress() + " .. 0x" + body.getMaxAddress() + " (" + body.getNumAddresses() + " bytes)");
            println("Signature          : " + func.getPrototypeString(false, false));
            println("Calling Convention : " + func.getCallingConventionName());

            // Incoming calls (Callers + Xrefs to entry)
            Set<Function> callers = func.getCallingFunctions(monitor);
            println("\n--- Incoming Calls (Callers: " + callers.size() + ") ---");
            for (Function caller : callers) {
                println("  <- " + caller.getName() + " @ 0x" + caller.getEntryPoint());
            }
            ReferenceIterator refsTo = rm.getReferencesTo(entry);
            int xrefToCount = 0;
            while (refsTo.hasNext() && xrefToCount < 25) {
                Reference r = refsTo.next();
                Function fromFunc = fm.getFunctionContaining(r.getFromAddress());
                String fromFuncStr = (fromFunc != null) ? (" in " + fromFunc.getName() + "@0x" + fromFunc.getEntryPoint()) : "";
                println("  [XrefTo Entry] from 0x" + r.getFromAddress() + " (" + r.getReferenceType() + ")" + fromFuncStr);
                xrefToCount++;
            }

            // Collect instructions, outgoing call xrefs, and data references
            InstructionIterator instIter = listing.getInstructions(body, true);
            List<String> disasmLines = new ArrayList<>();
            List<String> callRefLines = new ArrayList<>();
            List<String> dataRefLines = new ArrayList<>();
            int vfpuCount = 0;

            while (instIter.hasNext()) {
                Instruction inst = instIter.next();
                Address iAddr = inst.getAddress();
                String mnem = inst.getMnemonicString();
                if (mnem.startsWith("v") || mnem.startsWith("lv.") || mnem.startsWith("sv.") || mnem.startsWith("mtv") || mnem.startsWith("mfv")) {
                    vfpuCount++;
                }
                byte[] b = inst.getBytes();
                StringBuilder hex = new StringBuilder();
                for (byte x : b) hex.append(String.format("%02x", x));
                disasmLines.add(String.format("  0x%s: %-8s  %s", iAddr.toString(), hex.toString(), inst.toString()));

                for (Reference ref : inst.getReferencesFrom()) {
                    RefType rt = ref.getReferenceType();
                    Address to = ref.getToAddress();
                    if (rt.isCall()) {
                        Function targetFunc = fm.getFunctionAt(to);
                        if (targetFunc == null && mem.contains(to)) {
                            DisassembleCommand dc = new DisassembleCommand(to, null, true);
                            dc.applyTo(currentProgram, monitor);
                            CreateFunctionCmd fc = new CreateFunctionCmd(to);
                            fc.applyTo(currentProgram, monitor);
                            targetFunc = fm.getFunctionAt(to);
                        }
                        callRefLines.add(String.format("  [0x%s] -> 0x%s (%s) %s",
                            iAddr, to, rt.getName(), (targetFunc != null ? targetFunc.getName() : "")));
                    } else if (rt.isData() || !rt.isFlow()) {
                        Symbol sym = st.getPrimarySymbol(to);
                        Data d = listing.getDataAt(to);
                        String valStr = (d != null && d.getValue() != null) ? (" = " + d.getValue().toString()) : "";
                        dataRefLines.add(String.format("  [0x%s] -> 0x%s (%s) sym=%s%s",
                            iAddr, to, rt.getName(), (sym != null ? sym.getName() : "none"), valStr));
                    }
                }
            }

            Set<Function> callees = func.getCalledFunctions(monitor);
            println("\n--- Outgoing Calls (Unique Callees: " + callees.size() + ", Call Sites: " + callRefLines.size() + ") ---");
            for (Function callee : callees) {
                println("  -> " + callee.getName() + " @ 0x" + callee.getEntryPoint());
            }
            for (String cl : callRefLines) {
                println(cl);
            }

            println("\n--- Data References & Constants (" + dataRefLines.size() + ") ---");
            for (String dl : dataRefLines) {
                println(dl);
            }
            println("VFPU / Vector Instructions Detected: " + vfpuCount);

            println("\n--- Disassembly (" + disasmLines.size() + " instructions) ---");
            int maxLines = Math.min(disasmLines.size(), 90);
            for (int i = 0; i < maxLines; i++) {
                println(disasmLines.get(i));
            }
            if (disasmLines.size() > maxLines) {
                println("  ... (" + (disasmLines.size() - maxLines) + " more instructions)");
            }

            println("\n--- Decompiled Pseudocode (Ghidra 12.2 + ghidra-allegrex v21.5) ---");
            DecompileResults res = decomp.decompileFunction(func, 60, monitor);
            if (res != null && res.decompileCompleted() && res.getDecompiledFunction() != null) {
                println(res.getDecompiledFunction().getC());
            } else {
                println("[DECOMPILER ERROR] " + (res != null ? res.getErrorMessage() : "null result"));
            }
        }

        decomp.dispose();
    }
}

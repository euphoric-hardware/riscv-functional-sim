use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use log::info;

#[derive(Default, Debug)]
pub struct ExecutionState {
    pub pc: u64,
    pub instruction: u32,
    pub register_updates: Vec<(u8, u64)>,  // (reg number, value)
    pub fregister_updates: Vec<(u8, u64)>, // (reg number, value)
    pub memory_writes: Vec<(u64, u64)>,    // (address, value)
}

pub struct Diff {}

impl Diff {
    pub fn parse_spike_log(&self, filename: &str) -> io::Result<Vec<ExecutionState>> {
        let mut states = Vec::new();

        let file = File::open(Path::new(filename));
        if file.is_err() {
            eprintln!("Error: Could not open file {}", filename);
            return Err(file.err().unwrap());
        }

        let reader = io::BufReader::new(file.unwrap());

        for line in reader.lines() {
            if let Ok(log) = line {
                if let Some(state) = self.parse_log_line(&log) {
                    states.push(state);
                }
            }
        }
        println!("spike log parsed, {} instructions executed!", states.len());
        Ok(states)
    }

    pub fn parse_log_line(&self, line: &str) -> Option<ExecutionState> {
        // println!("Processing line: '{}'", line); // Debugging line

        let parts: Vec<&str> = line.split_whitespace().collect();
        // println!("Split into parts: {:?}", parts); // Debugging line

        if parts.len() < 5 {
            // println!("Skipping malformed line: '{}'", line);
            return None;
        }

        // Extracting PC (3rd entry, 0-based index 2)
        let pc_result = u64::from_str_radix(parts[3].trim_start_matches("0x"), 16);
        if pc_result.is_err() {
            // println!("Failed to parse PC from: '{}'", parts[3]);
            return None;
        }
        let pc = pc_result.unwrap();

        let instr_str = parts[4].trim_start_matches("(").trim_end_matches(")");

        // Strip the '0x' prefix from the instruction string if it exists
        let instr_str = instr_str.trim_start_matches("0x");

        // Ensure instruction string is a valid hexadecimal value
        if !instr_str.chars().all(|c| c.is_digit(16)) {
            // println!("Instruction string is not valid hex: '{}'", instr_str);
            return None;
        }

        // Extracting Instruction (5th entry, 0-based index 4, inside parentheses without 0x prefix)
        let instruction_result = u32::from_str_radix(instr_str, 16);
        if instruction_result.is_err() {
            // println!("Failed to parse instruction from: '{}'", instr_str);
            return None;
        }
        let instruction = instruction_result.unwrap();

        let mut register_updates = Vec::new();
        let mut fregister_updates = Vec::new();
        let mut memory_writes = Vec::new();

        let mut i = 5; // Start looking for register/memory after the instruction
        while i < parts.len() {
            if parts[i].starts_with('x') {
                // Register update
                // Ensure there's enough space for both the register name and value
                if i + 1 < parts.len() {
                    if let (Some(reg_num), Some(value)) = (
                        parts[i][1..].parse::<u8>().ok(),
                        u64::from_str_radix(parts[i + 1].trim_start_matches("0x"), 16).ok(),
                    ) {
                        register_updates.push((reg_num, value));
                    }
                    i += 2; // Skip to next part
                } else {
                    break; // Exit loop if we don't have enough data
                }
            } else if parts[i].starts_with('f') {
                // Register update
                // Ensure there's enough space for both the register name and value
                if i + 1 < parts.len() {
                    if let (Some(reg_num), Some(value)) = (
                        parts[i][1..].parse::<u8>().ok(),
                        u64::from_str_radix(parts[i + 1].trim_start_matches("0x"), 16).ok(),
                    ) {
                        fregister_updates.push((reg_num, value));
                    }
                    i += 2; // Skip to next part
                } else {
                    break; // Exit loop if we don't have enough data
                }
            } else if i == 5 && parts[i] == "mem" {
                // Memory write
                // Ensure there's enough space for both the value and address
                if i + 1 < parts.len() {
                    if let (Some(value), Some(addr)) = (
                        u64::from_str_radix(parts[i - 1].trim_start_matches("0x"), 16).ok(),
                        u64::from_str_radix(parts[i + 1].trim_start_matches("0x"), 16).ok(),
                    ) {
                        memory_writes.push((addr, value));
                    }
                    i += 2; // Skip to next part
                } else {
                    i += 1; // Skip just the "mem" keyword and try next part
                }
            } else {
                i += 1; // Continue looking at the next part
            }
        }

        let parsed_state = ExecutionState {
            pc,
            instruction,
            register_updates,
            fregister_updates,
            memory_writes,
        };
        Some(parsed_state)
    }

    pub fn diff_execution_state(
        spike_state: Option<&ExecutionState>,
        sim_state: Option<&ExecutionState>,
    ) -> bool {
        if let Some(spike_state) = spike_state {
            if let Some(sim_state) = sim_state {
                if spike_state.pc != sim_state.pc {
                    info!(
                        "PC MISMATCH: Spike 0x{:x}, Emulator 0x{:x}\n",
                        spike_state.pc, sim_state.pc
                    );
                    return false;
                }

                let compact = (spike_state.instruction as u16 as u32) == spike_state.instruction;
                if (spike_state.instruction != sim_state.instruction && !compact)
                    || (compact && spike_state.instruction != sim_state.instruction as u16 as u32)
                {
                    info!(
                        "INSTRUCTION MISMATCH: Spike 0x{:x}, Emulator 0x{:x}, PC = {:#08x}\n",
                        spike_state.instruction, sim_state.instruction, spike_state.pc
                    );
                    return false;
                }

                // Compare register updates
                let spike_regs: std::collections::HashMap<_, _> =
                    spike_state.register_updates.iter().cloned().collect();
                let my_regs: std::collections::HashMap<_, _> =
                    sim_state.register_updates.iter().cloned().collect();

                for (&reg, &spike_val) in &spike_regs {
                    if let Some(&my_val) = my_regs.get(&reg) {
                        if spike_val != my_val {
                            info!(
                                "REGISTER x{} MISMATCH: Spike 0x{:x}, Emulator 0x{:x}, PC = {:#08x}\n",
                                reg, spike_val, my_val, spike_state.pc
                            );
                            return false;
                        }
                    } else {
                        info!(
                            "REGISTER x{} updated in Spike but not in Emulator, PC = {:#08x}\n",
                            reg, spike_state.pc
                        );
                        return false;
                    }
                }

                for (&reg, &my_val) in &my_regs {
                    if !spike_regs.contains_key(&reg) {
                        info!(
                            "REGISTER x{} updated in Emulator but not in Spike, PC = {:#08x}\n",
                            reg, spike_state.pc
                        );
                        return false;
                    }
                }

                // Compare float register updates
                let spike_fregs: std::collections::HashMap<_, _> =
                    spike_state.fregister_updates.iter().cloned().collect();
                let my_fregs: std::collections::HashMap<_, _> =
                    sim_state.fregister_updates.iter().cloned().collect();

                for (&reg, &spike_val) in &spike_fregs {
                    if let Some(&my_val) = my_fregs.get(&reg) {
                        if spike_val != my_val {
                            info!(
                                "REGISTER f{} MISMATCH: Spike 0x{:x}, Emulator 0x{:x}, PC = {:#08x}\n",
                                reg, spike_val, my_val, spike_state.pc
                            );
                            return false;
                        }
                    } else {
                        info!(
                            "REGISTER f{} updated in Spike but not in Emulator, PC = {:#08x}\n",
                            reg, spike_state.pc
                        );
                        return false;
                    }
                }

                for (&reg, &my_val) in &my_fregs {
                    if !spike_fregs.contains_key(&reg) {
                        info!(
                            "REGISTER f{} updated in Emulator but not in Spike, PC = {:#08x}\n",
                            reg, spike_state.pc
                        );
                        return false;
                    }
                }

                // Compare memory writes
                let spike_mem: std::collections::HashMap<_, _> =
                    spike_state.memory_writes.iter().cloned().collect();
                let my_mem: std::collections::HashMap<_, _> =
                    sim_state.memory_writes.iter().cloned().collect();

                for (&addr, &spike_val) in &spike_mem {
                    if let Some(&my_val) = my_mem.get(&addr) {
                        if spike_val != my_val {
                            info!(
                                    "MEMORY WRITE MISMATCH at 0x{:x}: Spike 0x{:x}, Emulator 0x{:x}, PC = {:#08x}\n",
                                    addr, spike_val, my_val, spike_state.pc
                                );
                            return false;
                        }
                    } else {
                        info!(
                            "MEMORY WRITE at 0x{:x} present in Spike but missing in Emulator, PC = {:#08x}\n",
                            addr, spike_state.pc
                        );
                        return false;
                    }
                }

                for (&addr, &my_val) in &my_mem {
                    if !spike_mem.contains_key(&addr) {
                        info!(
                            "MEMORY WRITE at 0x{:x} present in Emulator but missing in Spike, PC = {:#08x}\n",
                            addr, spike_state.pc
                        );
                        return false;
                    }
                }
            } else {
                info!(
                    "No corresponding sim state for PC = {:#016x}, instruction = {:#08x}\n",
                    spike_state.pc, spike_state.instruction
                );
                return false;
            }
        } else {
            info!("No spike state!\n");
            return false;
        }
        return true;
    }

    pub fn diff_execution_states(spike_log: &[ExecutionState], sim_log: &[ExecutionState]) -> bool {
        let min_len = spike_log.len().min(sim_log.len());

        for i in 0..min_len {
            let spike_state = &spike_log[i];
            let sim_state = &sim_log[i];
            if Diff::diff_execution_state(Some(spike_state), Some(sim_state)) == false {
                return false;
            }
        }

        if spike_log.len() != sim_log.len() {
            info!(
                "LOG LENGTH MISMATCH: Spike has {} entries, Emulator has {}\n",
                spike_log.len(),
                sim_log.len()
            );
            return false;
        }
        return true;
    }
}

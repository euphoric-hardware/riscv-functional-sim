use crate::Rom;

pub struct InstructionMemory {
    data: Vec<u32>
}

impl InstructionMemory {
    pub fn new_instruction_memory(rom: &Rom) -> Self {
        // TODO - implement rom header reading. For now, assume the rom is just instrctions
        let mut data: Vec<u32> = (0..0).collect();

        for address in (0..rom.get_length()) {
            data.push(rom.get_instruction(address));
        }

        return InstructionMemory{data}
    }

    pub fn read(&self, address: usize) -> u32 {
        return self.data[address];
    }
}
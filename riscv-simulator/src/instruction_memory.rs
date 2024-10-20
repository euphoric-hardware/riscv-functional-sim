use crate::Rom;

pub struct InstructionMemory {
    data: Vec<u8>
}

impl InstructionMemory {
    pub fn new_instruction_memory(rom: &Rom) -> Self {
        // TODO - implement rom header reading. For now, assume the rom is just instrctions
        let mut data: Vec<u8> = (0..0).collect();

        for address in 0..rom.get_length() {
            data.push(rom.read_byte(address));
        }

        return InstructionMemory{data}
    }

    pub fn read(&self, address: usize) -> u32 {
        return (self.data[address + 3 as usize] as u32 | (self.data[address + 2 as usize] as u32 ) << 8 | (self.data[address + 1 as usize] as u32) << 16 | (self.data[address as usize] as u32) << 24) as u32;
    }
}
import sys
import random
import struct

def generate_immediate_arith_instruction():
    op = 0x13 # set opcode to indicate immediate arithmetic operations
    rd = random.randint(0, 31)
    funct3 = random.randint(0, 7)
    rs1 = random.randint(0, 31)

    if funct3 == 5:
        imm = (random.randint(0, 1) * 0x20) << 5
        imm |= random.randint(0, 32)
    else:
        imm = random.randint(0, 0xfff)
    
    instruction = ((imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | op)
    print(hex(instruction))
    return instruction


def generate_instructions(n):
    return [generate_immediate_arith_instruction() for _ in range(n)]

if __name__ == "__main__":
    instructions = generate_instructions(int(sys.argv[2]))
    
    with open(sys.argv[1], "wb") as binary_file:
        for instruction in instructions:
            binary_file.write(struct.pack('>I', instruction))
    binary_file.close()
    print("Wrote", sys.argv[2], "instructions to", sys.argv[1] + "!")

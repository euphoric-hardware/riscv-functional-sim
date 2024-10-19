import sys
import random
import struct

def generate_immediate_arith_instruction(operation=-1):
    op = 0x13 # set opcode to indicate immediate arithmetic operations
    rd = random.randint(0, 31)
    funct3 = random.randint(0, 7) if operation == -1 else operation
    rs1 = random.randint(0, 31)

    if funct3 == 5:
        imm = (random.randint(0, 1) * 0x20) << 5
        imm |= random.randint(0, 32)
    else:
        imm = random.randint(0, 0xfff)
    
    instruction = ((imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | op)
    print(hex(instruction))
    return instruction

def generate_immediate_memory_instruction(operation=-1):
    op = 0x3 # set opcode to indicate load operations
    rd = random.randint(0, 31)

    funct3 = 3
    while funct3 == 3:
        funct3 = random.randint(0, 5)
    rs1 = random.randint(0, 31)

    imm = random.randint(0, 0xfff)
    
    instruction = ((imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | op)
    print(hex(instruction))
    return instruction

def generate_store_memory_instruction(operation=-1):
    op = 0x23 # set opcode to indicate store operations
    imm_lower = random.randint(0, 31)

    funct3 = 4
    while funct3 == 4:
        funct3 = random.randint(0, 2)
    rs1 = random.randint(0, 31)
    rs2 = random.randint(0, 31)

    imm_upper = random.randint(0, 0x7f)
    
    instruction = ((imm_upper << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (imm_lower << 7) | op)
    print(hex(instruction))
    return instruction

def generate_branch_instruction(operation=-1):
    op = 0x63 # set opcode to indicate store operations
    imm_lower = random.randint(2, 31)

    funct3 = 2
    while funct3 in (2, 3):
        funct3 = random.randint(0, 7)
    rs1 = random.randint(0, 31)
    rs2 = random.randint(0, 31)

    imm_upper = random.randint(0, 0x7f)
    
    instruction = ((imm_upper << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (imm_lower << 7) | op)
    print(hex(instruction))
    return instruction

def generate_instructions(args, n):
    results = []
    for i in range(n):
        i = random.randint(0, len(args) - 1)
        results.append(args[i]())
    return results
# def generate_jump_instruction(operation=-1):
#     op = 0x67 if random.randint(0, 1) else 0x6f
#     if op == 0x67:
#         # jalr
    
#     if op == 0x6f:
#         # jal


if __name__ == "__main__":
    instructions = generate_instructions([generate_immediate_arith_instruction], int(sys.argv[2]))
    
    with open(sys.argv[1], "wb") as binary_file:
        for instruction in instructions:
            binary_file.write(struct.pack('>I', instruction))
    binary_file.close()
    print("Wrote", sys.argv[2], "instructions to", sys.argv[1] + "!")

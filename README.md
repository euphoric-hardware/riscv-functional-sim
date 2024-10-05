# Generated Functional Simulation (SAIL-Spike)

---

## Goals

- Automate the generation of instruction interpretation logic
- It should have a basic top that works like Spike, but it should also be a library where users can write their own top
    - Ganged-simulation, trace generation, sampling (checkpointing), trace-execution mode(?)
- Cleanup the weird coroutine stuff in FESVR
    - [tokio](https://docs.rs/tokio/latest/tokio/index.html)
- High performance
    - Biggest performance bottleneck of functional simulators are in the instruction decode stage
    - Need to have a micro-op cache where we maintain decoded instructions

---

## Background information

### Threaded interpretation vs switch based interpretation

- [NEMU](https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=9923860&tag=1)
- Reference: [Dynamic dispatch vs computed gotos](https://stackoverflow.com/questions/58774170/how-to-speed-up-dynamic-dispatch-by-20-using-computed-gotos-in-standard-c)

---

## Development plans

### SAIL

- Parser: parse SAIL into some in-memory format
- Logic generation

### Funtional simulator steps

- [Spike](https://github.com/riscv-software-src/riscv-isa-sim)
    - Make sure you can run baremetal binaries in Spike (riscv-tests are a good start)
- Define bare-minimum architectural state and simple execution logic of each processor
    - [RISC-V user mode specification](https://five-embeddev.com/riscv-user-isa-manual/latest-latex/riscv-spec.html)
    - For the architectural state, start off with: program counter (PC), register file
        - In Spike, this is defined in `state_t` in `riscv/processor.h`
        - The RISC-V specification supports two datapath widths: 32 bits vs 64 bits
        - The register file (and other following architectural states) must be parameterized to support both bitwidths
        - The architectural state has to trivally serializable
            - Various use cases: sampled simulation, verification and debug
            - Should use Rust's type class derivation
    - Write interpretation logic for a processor that can execute RV32G/RV64G instructions (look in the RISC-V specification)
        - [RV32I](https://five-embeddev.com/riscv-user-isa-manual/latest-latex/rv32.html#rv32)
        - [RV64I](https://five-embeddev.com/riscv-user-isa-manual/latest-latex/rv64.html#rv64)
        - [RV32/64G](https://five-embeddev.com/riscv-user-isa-manual/latest-latex/gmaps.html#rv3264g-instruction-set-listings)
        - In Spike, this is defined in `processor_t::step` in `riscv/execute.cc`
        - Types of instructions
            - Integer computational instructions: add, sub, shift left/right
            - Control flow instructions: branch, jump
            - Hint instructions (ignore in this step)
            - Load and store instructions (ignore in this step)
            - Fence (ignore in this step)
        - Details
            - Fetch instruction at PC
            - Decode the fetched instruction
            - Interpret the instruction and update the register file state accordingly
            - Update PC
        - The input format can be a file containing instructions
- Support memory instructions
    - Define memory and connect it with the above processor to support load and store instructions
    - The memory shouldn't be as large as the actual physical memory that we are emulating
        - Programs normally doesn't use all available physical memory
        - In Spike, the physical memory is defined as `sparse_memory_map` in `devices.h`
        - Similarly, we can use a dictionary to implement physical memory
    - The memory state is also a part of the architectural state. Hence, this should be easily serializable as well
- Move on to the supervisor mode instructions
    - [RISC-V privileged specification](https://five-embeddev.com/riscv-priv-isa-manual/Priv-v1.12/riscv-privileged.html)
    - We will start implementing CSRs, interrupts, exceptions, virtual memory...
- Support interrupts
    - Interrupts and exceptions in RISC-V are executed by writing to control state registers (CSR)s
        - [sifive interrupt cookbook](https://sifive.cdn.prismic.io/sifive/0d163928-2128-42be-a75a-464df65e04e0_sifive-interrupt-cookbook.pdf)
    - CSRs are also a part of the architectural state (and hence should be serializable)
    - There are two types of interrupts: core local interrupts and platform level interrupts
        - Core local interrupts are handled by a device called CLINT in the SoC. Some examples are software interrupts, timer interrupts, and external interrupts
        - Platform level interrupts are handled by a device called PLIC in the SoC. It is used to by the cores to interact with IO devices
        - When a interrupt signal comming into the core goes high, the `mip` CSR is written and the core handles the interrupt
        - Detailed description of CSRs related to interrupts and exceptions are in page 6 of the "sifive interrupt cookbook"
    - Example use case of CLINT: target boot process. A typical target boot process looks like this:
        - The host machine loads the binary into the target system using FESVR. While this is happening the cores in the target are spinning, waiting for a interrupt
        - Once this is done, the host machine sends a message to the target where the endpoint address is the CLINT
        - The CLINT receives the message, raises the interrupt signal, the core PC jumps to the starting address of the program
    - Implement a CLINT
        - In Spike, this is defined in `riscv/clint.cc`
        - The state of CLINT is also a part of the architectural state!
    - A CLINT also as a range of addresses that can be used to send messages to it
        - A typical address of the CLINT is 0x2000000
        - The address of the CLINT should also be included in the DTS!
    - At this point, we can try hooking the emulation framework w/ FESVR to run simple binaries
- Support exception handling
    - On an exception, these CSRs must be written
        - `mepc`: PC that caused the exception
        - `mcause`: trap cause
        - `mtval`: trap value
    - Add these CSRs and check if exception handling works correctly
- Support virtual memory
    - To support virtual memory, we need to implement TLBs
    - `satp`
    - `pmp`
    - More on this later...
- Critical extensions
    - Support compressed instructions [C extension](https://five-embeddev.com/riscv-user-isa-manual/latest-latex/c.html#compressed)
    - Support atomic instructions [A extension](https://five-embeddev.com/riscv-user-isa-manual/latest-latex/a.html#atomics)

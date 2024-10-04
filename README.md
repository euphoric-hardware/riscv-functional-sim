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

### Funtional simulator

- Processor
    - Instruction fetch & decode
    - Register file, CSR, updates
    - Virtual memory & TLB
    - Memory interface
- Memory state
    - PMP regions
    - Virtual memory and paging
- Peripherals
    - Interrupts (PLIC & CLINT)
    - Bootrom
    - Block device
    - UART
- FESVR
    - Binary loading
    - Syscall proxy


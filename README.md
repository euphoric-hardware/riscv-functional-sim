# Generated Functional Simulation (SAIL-Spike)

## Background and Goals

We want to build a RISC-V instruction set simulator (ISS) from first principles.

### Support Many Modes of Operation

We want to support all these modes both as top and as a library.

#### Master

- ISS executes a binary directly
- As a library: still master, but can be controlled by custom top
  - Can dump traces into buffer for top to analyze
  - Can checkpoint / restore / rewind
  - Can emulate time from external source (e.g. sampled RTL simulation, uArch perf model)

#### Ganged / CoSim

- ISS executes a binary and emulates all arch state, but not as strict master. It receives arch events from RTL simulation and determines whether the next instruction group to commit in RTL sim is legal and matches the expected commit from the ISS.
- There is a big range of what RTL is verified in ganged simulation based on which SoC components are simulated in the ISS exactly vs simply 'believed' from RTL simulation.
  - For instance, a DMA engine can be modeled exactly in the ISS, or the transactions to/from the DMA engine in RTL can be simply replayed in the ISS (eliding verification of the DMA engine's behavior itself).

#### Slave

- ISS acts as a trace ingester from RTL sim / trace of another execution
- All SoC components and arch state are still modeled. The trace can contain partial information about the SoC (e.g. only the core / DRAM state can be reconstructed).
- In this mode, the ISS is used as a library and the top-level peeks the reconstructed arch state as needed (e.g. for trace-driven profiling / flamegraph construction)
- We can use this mode to do replay single-stepping of the SoC, a single instruction at a time

#### Symbolic execution

- The modeled arch state is a mix of concrete and symbolic state
- This works similar to the slave mode, except the state update rules are computed symbolically
- This is useful for information flow tracking and memory trace reconstruction, among other things

#### Other things

There are a bunch of other use-cases and features we wish to support that are quite iffy in the current spike + Chipyard world.

- **Exact SoC modeling**: all undefined / vague behaviors pinned down. All SoC components and their arch state are modeled.
  - An identical setup in the ISS that matches the SoC exactly
  - RTL that's generated should be driving the parameterization of the functional sim (not the other way around)
  - First-class support for passing a dts and bootrom into the functional sim from the RTL generator
- **Checkpoint / restore**: deser of arch state + testbench component / IO model state. No loss of information.
- **Trace analysis**: generic analysis pass writing using a generic ISA IR. Ability to dump execution traces into a trace buffer controlled and drained by a custom top.
- **Sampled simulation**: a custom top that leverages the above for sampled RTL simulation for accurate performance trace estimation.
- **Instruction generation**: for DV or fuzzing a RISC-V DUT.
- **Formal equivalence checking**: similar to [riscv-formal](https://github.com/YosysHQ/riscv-formal).
- **RTL generation**: targeting a simple single-cycle core model.
- **Coverage analysis**: given a trace, track code path coverage within the ISS + instruction-level coverage (see [RISC-V ISAC](https://riscv-isac.readthedocs.io/en/0.4.0/overview.html))
- **High performance disassembler**
  - Disassembles execution traces into Rust-native structures either based on instruction encoding type (R, I, ...) or semantic instruction type (arithmetic, memory access, control flow, etc.)
  - Leverage the host's SIMD ISA for high performance decoding
    - Use [x86 bitextract intrinsics](https://github.com/gnzlbg/bitintrf) to speed up instruction decode

### Unification of Testbench/IO Models

We should unify models between all simulation backends (ISS, RTL simulation, FPGA prototyping, FPGA-based emulation / Firesim, ASIC-based emulation) + reality (testchip bringup).
This includes: fesvr + IO models + everything on the edge of the RISC-V target.
The current state of fesvr + IO models is quite unified, but not sufficient since we need exact state checkpointing and restore (and ideally no more C++).

There are some challenges like accurate checkpointing + restore for stateful non-DUT components, especially if we use Rust's coroutines.
On the Chipyard RTL simulation side, we also need to make top-level ports explicit (no internal DPIs).

### EZ Custom Tops

We should have a basic top that works like Spike, but we should support library usage where users can write their own top.

Custom tops with spike are a pain.
We want to simplify it.
Dromajo does a better job, but we can do even better.

### High Performance

We're sure there are many tricks here (faster instruction decoding, caching, basic block-granularity execution) that are played by NEMU but not spike.
We anticipate we can build an ISS that can run at 500+ MIPS, which could obviate DBT.

### Principled Discrete Event Simulation

Spike uses an ad-hoc mechanism of multiple host threads and `switch_to()` calls to emulate parallel simulation threads (e.g. between switching between fesvr, IO models, and the target RISC-V core - each with their separate contexts and stacks).
Ideally, we can leverage an actual discrete event simulation framework (like [DAM](https://github.com/stanford-ppl/DAM-RS)) and remove these host thread switching hacks (or build something on top of [tokio](https://docs.rs/tokio/latest/tokio/index.html)).

One challenge is to integrate this with the Chipyard RTL simulation environment and Firesim.
This also needs to play nicely with serialization of IO/testbench model state, which seems very tricky, if not impossible.
Perhaps the only way to make this easy is to force all state to be in the RTL abstraction or serializable software datastructures and keep all the instant update rules as regular arbitrary Rust code.
This implies that state machines must be explicitly constructed however, which is a big annoyance.

### Generated ISS

Ideally we don't want to build a point implementation, but rather an ISS generator that consumes a formal spec of the ISA.
We would like to (eventually) avoid a hand-written ISA implementation (like in spike or qemu).
This is very idealistic and many prior attempts have been made (e.g. riscv-sail, pydrofoil), but none can achieve high performance and ease of integration with custom tops.

### Dynamic Binary Translation (DBT) Mode

For maximum performance, there is no substitute for host-ISA codegen (dynamic binary translation).
Since we don't need to support multiple ISAs, we can avoid an intermediary layer like in qemu (i.e. TCG-IR).
Since we're using Rust, it would be great to use the Cranelift IR and JIT!

### Prior Work

#### ISS

- spike ([riscv-isa-sim](https://github.com/riscv-software-src/riscv-isa-sim/))
  - Considered the 'golden model' for RISC-V
- [dromajo](https://github.com/chipsalliance/dromajo/tree/master)
  - Designed by Esperanto for RTL co-simulation for DV. Not actively maintained anymore.
- [NEMU](https://github.com/NJU-ProjectN/nemu)
  - Created by the comparch team at Nanjing University.
  - Contains various performance optimizations discussed in the Xiangshan paper (["Towards Developing High Performance RISC-V Processors Using Agile Methodology"](https://ieeexplore.ieee.org/stamp/stamp.jsp?tp=&arnumber=9923860))
  - Uses a threaded interpreter architecture (maybe with it's own internal bytecode which is interpreted)
  - Some links about threaded interpreters: [Link 1](https://www.complang.tuwien.ac.at/forth/threaded-code.html), [Link 2](https://stackoverflow.com/questions/58774170/how-to-speed-up-dynamic-dispatch-by-20-using-computed-gotos-in-standard-c), [Link 3](https://stackoverflow.com/questions/3848343/decode-and-dispatch-interpretation-vs-threaded-interpretation), [Link 4](https://stackoverflow.com/questions/75028678/is-it-impossible-to-write-thread-code-in-rust), [Link 5](https://users.rust-lang.org/t/how-can-i-approach-the-performance-of-c-interpreter-that-uses-computed-gotos/6261)
- [Greg Chadwick: Building a RISC-V simulator in Rust - Part 1](https://gregchadwick.co.uk/blog/building-rrs-pt1/)
  - [GregAC/rss](https://github.com/GregAC/rrs) (untouched in one year)
- [d0iasm/rvemu](https://github.com/d0iasm/rvemu) (rvemu: RISC-V Emulator) (untouched in one year)
  - Quite feature complete: "RV64GC ISA (RV64IMAFD, Zicsr, Zifencei, RV64C), privileged ISA, CSRs, virtual memory system (Sv39), peripheral devices (UART, CLINT, PLIC, Virtio), and device tree"
  - [Writing a RISC-V Emulator in Rust](https://book.rvemu.app/)
- [mateocabanal/riscvm](https://github.com/mateocabanal/riscvm)
  - RV64GC unprivileged userspace emulator (directly proxies syscalls, no OS boot support). Mostly a fun project.
- [siriusdemon/Rare](https://github.com/siriusdemon/Rare)
  - Rust RISC-V emulator based on [Asami's tutorial](https://book.rvemu.app/)
- [theonlymrcat/rivet](https://sr.ht/~theonlymrcat/rivet/)
  - RISC-V emulator written in Zig

#### DBT

- QEMU is the SOTA DBT simulator
- [Accelerate RISC-V Instruction Set Simulation by Tiered JIT Compilation (VMIL 2024)](https://dl.acm.org/doi/abs/10.1145/3689490.3690399)
- [Pydrofoil](https://github.com/pydrofoil/pydrofoil)
  - Uses the Sail RISC-V model and emulates it using PyPy
  - Uses the [ISLA backend for Sail](https://github.com/rems-project/isla?tab=readme-ov-file) to generate some [representation of the ISA](https://github.com/rems-project/isla-snapshots). This is all very unclear and iffy to me.
  - [Some notes](https://docs.pydrofoil.org/en/latest/background_optimizations.html) on pydrofoil's optimizations
  - [Talk: Pydrofoil: A fast RISC-V emulator generated from the Sail model, using PyPy's JIT](https://www.youtube.com/watch?v=dUHWhUdXFJg)
  - The complexity of this project [is insanely high](https://github.com/pydrofoil/pydrofoil/blob/main/pydrofoil/ir.py) since they are parsing and interpreting the Sail language itself in Python and then doing codegen on top of that
- [RVVM + libriscv](https://github.com/LekKit/RVVM/wiki#library-api)
  - "RVVM is a virtual machine / emulator for RISC-V guests"
  - "Emulation performance is higher than that of QEMU TCG, thanks to a tracing RVJIT with better designed IR and less guest instructions splitting, and usage of hardware floating-point unit."
  - Very cool

#### Architectural Description Languages / Generated ISS

Background:

- [How to improve the RISC-V specification by Alastair Reid](https://alastairreid.github.io/riscv-spec-issues/)
  - A great article about the pain of RISC-V specifications
- [ARM's Architecture Specification Language](https://developer.arm.com/Architectures/Architecture%20Specification%20Language)

Existing tools and languages (an overview of [existing machine readable specs](https://five-embeddev.com/quickref/machine-readable.html)):

- [Sail](https://github.com/riscv/sail-riscv)
  - Adopted as the formal spec for RISC-V
  - Painful to use
  - [ThinkOpenly / sail](https://github.com/ThinkOpenly/sail) - an attempt to JSON-ify Sail's IR and emit it (but no execution semantics)
    - [Demo: RISC-V Instruction Information Parsing and Storage for SAIL - Paul Clarke](https://www.youtube.com/watch?v=svMcOfxcy1Y)
- [Vienna ADL](https://arxiv.org/pdf/2402.09087)
  - [Cycle-Accurate Simulator Generator for the VADL Processor Description Language](https://repositum.tuwien.at/bitstream/20.500.12708/17053/1/Schuetzenhoefer%20Hermann%20-%202020%20-%20Cycle-Accurate%20simulator%20generator%20for%20the%20VADL...pdf)
  - [Optimized Processor Simulation with VADL](https://repositum.tuwien.at/bitstream/20.500.12708/157928/1/Mihaylov%20Hristo%20-%202023%20-%20Optimised%20Processor%20Simulation%20with%20VADL.pdf)
  - [A pred-LL(*) Parsable Typed Higher-Order Macro System for Architecture Description Languages (Video, GPCE 2023)](https://www.youtube.com/watch?v=jopIILxxNbQ)
  - VADL isn't open source, but the base rv64ui spec is available. They're working on OpenVADL, but it is a ways away.
  - [Efficient parsing of OpenVADL](http://www.complang.tuwien.ac.at/vadl/papers/NestlerFinal.pdf) (A BS thesis with a good description of VADL)
- [CodAL](https://codasip.com/2021/02/26/what-is-codal/)
  - Codasip's ADL - seems quite nice, but it is a custom language and the compiler for it is of course proprietary
- [riscv-unified-db](https://github.com/riscv-software-src/riscv-unified-db)
  - Derek Hower and Qualcomm people's attempt at building a formal spec for RISC-V that is easier to use than Sail
  - Based on [yaml files that encode every instruction](https://github.com/riscv-software-src/riscv-unified-db/blob/main/arch/README.adoc) and extensions
  - Some way to define concrete behaviors of undefined behavior in the spec
  - Instruction semantics are provided in IDL ([Interface Definition Language](https://www.omg.org/spec/IDL)) which is parsed/interpreted [in Ruby](https://github.com/RemedyIT/ridl)
  - They wish for this project to replace Sail, spike, and hand-written ISA specs
- [riscv-isa-data](https://github.com/five-embeddev/riscv-isa-data/tree/master)
  - Aspects of the RISC-V ISA as static YAML files. They appear to be manually written.
- [Versatile and Flexible Modelling of the RISC-V Instruction Set Architecture](https://agra.informatik.uni-bremen.de/doc/konf/TFP23_ST.pdf)
  - [libriscv - Extensible implementation of the RISC-V ISA based on FreeMonads - Haskell, Github](https://github.com/agra-uni-bremen/libriscv)
  - [BinSym](https://github.com/agra-uni-bremen/BinSym): Symbolic execution of RISC-V binary code based on formal instruction semantics
- [A Multipurpose Formal RISC-V Specification Without Creating New Tools (MIT people)](https://people.csail.mit.edu/bthom/riscv-spec.pdf)
- [MicroTESK](http://www.microtesk.org/)
  - This is an abandoned project
  - [Machine-Readable Specifications of RISC-V ISA](https://riscv.org/wp-content/uploads/2018/12/Machine-Readable-Specifications-of-RISC-V-ISA-Kamkin-Tatarnikov.pdf)

---

## Development plans

### Functional simulator steps (Safin, Ansh, Pramath)

- [Spike](https://github.com/riscv-software-src/riscv-isa-sim)
    - Make sure you can run baremetal binaries in Spike (riscv-tests are a good start)
- Define bare-minimum architectural state and simple execution logic of each processor
    - [RISC-V user mode specification](https://five-embeddev.com/riscv-user-isa-manual/latest-latex/riscv-spec.html)
    - For the architectural state, start off with: program counter (PC), register file
        - In Spike, this is defined in `state_t` in `riscv/processor.h`
        - The RISC-V specification supports two datapath widths: 32 bits vs 64 bits
            - The register file (and other following architectural states) must be parameterized to support both bitwidths
            - However, lets not worry too much about 32 bits at the moment. Get started with 64 bit architecture first
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

### Rearchitecting FESVR (Safin)

- Front end server (FESVR) background
    - [Chipyard FESVR documentation](https://chipyard.readthedocs.io/en/latest/Advanced-Concepts/Chip-Communication.html)
    - FESVR acts as a bridge between the host system (which is running the simulation) and the target system (simulated RISC-V SoC)
    - FESVR performs tasks such as loading the binary into the target system, handling target system calls, and exchanging data between the host and the target (e.g., print messages)
        - Program loading
            - Loads the RISC-V binary into the simulated system
            - Provides the simulation w/ necessary arguments
        - System call proxying
            - [riscv pk](https://github.com/riscv-software-src/riscv-pk)
            - There are cases when the programming running inside the simulator has to perform syscalls related to IO (e.g. prints, opening network sockets)
            - This program emulates these syscalls in the host machine
    - FESVR is shared across a wide range of simulation frameworks: Spike (functional sim), Chipyard sims (RTL sim), FireSim (FPGA Sim)
    - FESVR architecture
        - There are two threads when running simulations: the host thread and the target thread
        - The host thread performs the FESVR functionalities mentioned above: Program loading and syscall proxying
        - The target thread is responsible for executing the target binary in the simulated system
        - There is a buffer where each thread reads/writes messages
        - Example 1: lets say the host wants to write the binary into the target system's memory
            - Host thread instructs FESVR to write the binary to the target's DRAM address
            - Once this is done, host threads instructs FESVR to write to the CLINT to indicate that the binary loading is finished
            - Halt the host thread and switch to the target thread
            - Target thread starts executing the simulation
        - Example 2: lets say the target wants to print a message
            - Target thread performs a `printf`
            - The `printf` contains instructions that writes to a "magic address" that is connected to a target -> host buffer
            - Target thread halts and switches to the host thread
            - Host thread reads the message, emulates the `printf` behavior, and returns the control back to the target thread
- It would be nice if we can replace the host/target threads as coroutines
    - Write a custom `sim_t` in `riscv/sim.cc` so that it doesn't inherit `htif_t`, but uses `processor_t`, `mems`, `clint`, `plic` `bus`, etc for instruction execution
    - Rewrite FESVR in Rust using async libraries such as tokio
        - We can use elf reading libraries such as [rust-elf](https://github.com/cole14/rust-elf) or [elfio](https://docs.rs/elfio/latest/elfio/#)
        - Don't have to think about DTM based HTIF for now. Can just implement the TSI protocol based interface
    - Write rust bindings between the rewritten FESVR & the custom `sim_t` and see if we can run RISC-V binaries

---

## Architectural Definition Language

- Generating rust code: we can use the [syn](https://docs.rs/syn/latest/syn/) library to represent arbitrary Rust ASTs for code generation
- Scala embedded DSL for the specification language
    - Need a way of interpreting the language to generate code for functional simulation
        - An add instruction cannot be represented as an Scala add as we must interpret the operation for code gen
    - Need a clear separation of architectural state and update rules
        - For unprivileged instructions, this is straightforward
        - Privileged instructions are where this might become challenging
            - Virtual memory: this seems quite doable. Need to write rules for SATP & TLB updates
            - Interrupt & exception handling
            - Expressing the behaviors of PLIC & CLINT

### Approach 1: Use Chisel as the frontend, but build a custom interpreter

- Can reuse a lot of the Chisel constructs like `Vec`, `Bundle`, `UInt`
- Bridge the in-memory representation of CIRCT into FIRRTL2 and write FIRRTL passes that will emit components for the functional simulation
    - The compiler can take in the DTS of the SoC that you want to model, and compose the architectural states accordingly
    - What about undefined behaviors? If we bridge to FIRRTL2 in high FIRRTL, `DontCare`s aren't blasted into zeros so we can reinterpret this
- Potential downsides
    - Difficult (or impossible) to utilize the host type system which naturally leads to lower ergonomics
    - This also means that we have to describe every instruction by hand (as we cannot use type class derivation)
    - Also, if we were to just use Chisel, we would need to use stuff like annotations which will make the codebase messy and confusing
- One benefit of this approach is that it may be easier to generate performance models along with the functional model in the future
    - As we can describe branch predictor structures using existing Chisel, perhaps we can add additional passes in the interpretter to add these models as part of the functional simulator
    - Can be used for generating embeddings for sampled simulation or high level workload analysis based on traces
- Logistical thoughts
    - To get started initially, this might be easier as we can just simply write Chisel in a specific way
    - Bridging into FIRRTL2 is not fun, but very doable
    - The time to start writing the interpretter is low (probably less guidance required), but maybe the following progress may not be as fast than approach 2

### Approach 2: DSL for defining the architectural spec

- The interpretter implementation may be a bit more cleaner  as we can use type class derivation
- A high level sketch may look something like this:

```scala
case class ProcessorState(
    pc: UInt,
    rf: Vec[UInt],
    ...)

case class Instruction[T <: StateUpdateRule] {
    // Can use type class derivation to derive the update rule from using scala compiler
    def updaterule(p: ProcessorState)

    // Type class derivation logic for product types like `Add`
}

case class Add(rs1: UInt, rs2: UInt, rd: UInt, op: (UInt, UInt) => UInt) derives Instruction
```

- However, we must redefine basic datatypes such as `UInt`, `Vec`, `Bundle`
    - I don't think this is such a big deal. This shouldn't take too much time as we only want a limited set of primitives
    - Defining aggregate types may be a pain. But, do we really need aggregate types like in Chisel? For this particular DSL, I think it is perfectly fine to define aggregate types as a product type (for a HDL, I do think this is problematic due to ergonomics, but it really doesn't matter here)
    - It is also beneficial in that it is easier to enforce "correct behavior" to the spec writers by using the host language's type system
- Logistical thoughts
    - In the long term, I do prefer this than the above approach. Approach 1 just feels like more of an hack on top of Chisel rather than a clean implementation
    - However, this might feel quite difficult and they can loose motivation unless we guide them aggressively. We may even have to work on the initial implementation to get them going

---

# Configurability in functional simulation

- For cases when the functional simulation runs in ganged mode, trace-driven mode, or runahead mode (for sampling) we want the functional simulator device tree configuration to look identical to the actual SoC configuration
- Spike is problematic in that aligning the functional/RTL simulation configuration requires various hacks/modifications
    - Spike has its own bootrom which is the first source of divergence between RTL & functional simulation
    - For baremetal binaries, Spike boots without FESVR having to send a interrupt to the CLINT. This is another source of divergence that we would like to eliminate
    - API to add IO devices is not clean as device models are loaded in a dynamically linked libraries into simulation. However, there is no need for device models to be dynamically loaded in the first place

## How I would like the configuration system to look like

- There has to be two modes

### DTS generated mode

- The functional simulation configuration is "generated" from the device tree source (DTS) of the SoC that you want to model
- As mentioned above, this is useful for ganged simulation, trace-driven simulation, and runahead mode
- Parse the DTS file: [fdt](https://github.com/repnop/fdt)
- Generate a bus hierarchy according to the DTS

### Default mode

- The functional simulation uses a pre-determined DTS configuration with minimal IO device models (UART)
- The device models should not by dynamically linked libraries. Device models should be statically compiled into the binary and the top level should expose runtime flags to add/change device configurations
- Use case of this mode is for simple software verification/debugging
- We can think of the default mode as where there is no DTS provided and we are using a preconfigured DTS

### Implementation

- Need a way of registering all the possible device models and searching for matching ones from the DTS
- Need to implement a "bus" struct that has APIs for
    - Adding new devices on the bus and register its address range
    - Receive load/store requests and route them to the correct device
    - When it receives a request with an invalid address, return a response indicating that the request had a invalid address
- Possible devices includes: cores, DRAM, CLINT, PLIC, NIC, block device, uart, bootrom ...
- Future work: I would also like this "bus" struct to be able to defer certain transactions until there is a hint from the top level. This can be useful for ganged simulation

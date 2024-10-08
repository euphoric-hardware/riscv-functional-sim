#include <stdint.h>

extern volatile uint64_t fromhost;
extern volatile uint64_t tohost;

int main()
{
    uint64_t syscall_id = 64;   // write
    uint64_t arg0 = 1;          // fd
    uint64_t arg1 = 0xdeadbeef; // ptr
    uint64_t arg2 = 10;         // len

    volatile uint64_t magic_mem[8] __attribute__((aligned(64)));
    magic_mem[0] = syscall_id;
    magic_mem[1] = arg0;
    magic_mem[2] = arg1;
    magic_mem[3] = arg2;

    tohost = (uintptr_t)magic_mem;
    while (fromhost == 0)
        ;
    fromhost = 0;

    return magic_mem[0];
}
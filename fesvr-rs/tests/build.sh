bins=("elf-htif" "elf-implicit")
cc="riscv64-unknown-elf-gcc"
cflags="-mcmodel=medany -nostartfiles -T linker.ld"

for bin in "${bins[@]}"; do
    cd "$bin"
    echo "building $bin with $cc..."
    $cc $cflags -o "$bin" main.c
    cd ..
done

echo "done! run cargo test"
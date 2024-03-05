Compile by gcc:
CC=gcc CC_riscv32im_risc0_zkvm_elf=/opt/riscv/bin/riscv32-unknown-elf-gcc cargo build
or by clang:
CC=clang CFLAGS_riscv32im_risc0_zkvm_elf="-I/opt/riscv/riscv32-unknown-elf/include/ -target riscv32-unknown-elf -march=rv32im" cargo build

Both works now until you call c-kzg's function, by modify `methods/guest/src/main.rs`
```
error: linking with `rust-lld` failed: exit status: 1
hello_guest:   |
hello_guest:   = note: rust-lld: error: undefined symbol: free
hello_guest:           rust-lld: error: undefined symbol: calloc
hello_guest:           rust-lld: error: undefined symbol: malloc
hello_guest: warning: `hello_guest` (bin "hello_guest") generated 2 warnings
hello_guest: error: could not compile `hello_guest` (bin "hello_guest") due to 1 previous error; 2 warnings emitted
``````

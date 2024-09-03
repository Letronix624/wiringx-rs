# wiringX-rs

[wiringX](https://wiringx.org) bindings for Rust with extra Milk-V platforms included.

Using this library requires to have gcc, make, glibc, clang and the right toolchain for the target platform installed.
Please open an issue containing package manager specific install commands for this library to work out of the box.

The right RISC-V GCC toolchain can be built [here](https://github.com/riscv-collab/riscv-gnu-toolchain).

## Milk-V

To build and run your wiringX application on a Milk-V Duo Linux environment follow those steps:

1. Boot the Duo and connect to it using ssh using `ssh root@192.168.42.1` with password `milkv`.
   You can also run the `ssh-copy-id root@192.168.42.1` command to export your ssh key to the Duo.

2. On your Duo run `ln -s /lib/ld-musl-riscv64v0p7_xthead.so.1 /lib/ld-musl-riscv64.so.1` for the programs to work.

3. Install the [riscv-gnu-toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain), run the `configure` script with all default settings and no flags,
   then run `sudo make musl`

4. Open your `~/.cargo/config.toml` and add this to make it compile:

```toml
[target.riscv64gc-unknown-linux-musl]
linker = "riscv64-unknown-linux-musl-gcc"
rustflags = ["-C", "target-feature=-crt-static"]
```

> Your `sysroot` path may need to be declared
>
> ```
> export BINDGEN_EXTRA_CLANG_ARGS='--sysroot /path/to/toolchain/sysroot/'
> ```

5. Compile your Rust program to the `riscv64gc-unknown-linux-musl` architecture using wiringX in Rust using the nightly
   toolchain and build-std flags to unlock the standard library using this specific command:
   `cargo +nightly build --release --target=riscv64gc-unknown-linux-musl -Zbuild-std=std,core`

6. If the program has successfully compiled, scp it to your Duo using `scp -O target/riscv64gc-unknown-linux-musl/release/"binary_name" root@192.168.42.1:`

Do not forget to replace the "binary_name" with the actual name of your binary.

7. Run your binary.

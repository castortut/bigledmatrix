# STM32F1 Rust quickstart

Based on:
- https://github.com/rust-embedded/cortex-m-quickstart
- https://github.com/lupyuen/stm32-blue-pill-rust.git

Updated to use `stm32f1` and `stm32f1xx-hal` crates.

## Getting started

### Requirements:

- Rust target `thumbv7m-none-eabi` installed
- `arm-none-eabi-gdb` installed and in $PATH (available as part of https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain)
- OpenOCD installed

### Starting

Run OpenOCD in a terminal. Depending on the ST-Link used, the command line is something like this:
```
sudo openocd -f interface/stlink.cfg -f target/stm32f1x.cfg
```

Start the program through GDB:
```
cargo run
```

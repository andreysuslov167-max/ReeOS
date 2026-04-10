# ReeOS Documentation

## Overview

ReeOS is a minimal hobby operating system kernel written in Rust. It features VGA text mode output and keyboard input with a basic command-line interface.

## System Requirements

- x86_64 architecture
- BIOS boot (not UEFI)
- Minimum 16MB RAM

## Building

```bash
cargo run
```

# a must for testing!
Create a .cargo folder, create a config.toml file there, and write the following in it.
```bash
[unstable]
json-target-spec = true
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins"]
[build]
target = "kernel.json"
[target.'cfg(target_os = "none")']
runner = "bootimage runner"


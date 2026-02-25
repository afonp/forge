---
sidebar_position: 2
---

# installation

## cargo (recommended)

if you have rust installed, you can install forge directly from crates.io:

```bash
cargo install forge
```

## building from source

```bash
git clone https://github.com/afonp/forge.git
cd forge
cargo install --path .
```

## pre-built binaries

download the latest release for your platform from the [releases page](https://github.com/afonp/forge/releases).

available platforms:
- **linux**: x86_64, aarch64
- **macos**: x86_64, aarch64 (apple silicon)
- **windows**: x86_64

extract the archive and place the `forge` binary somewhere in your `$PATH`.

## requirements

- **rust** (for cargo install): 1.70+
- **g++**: required for compiling your c++ solutions via the generated makefile
- **make**: required for the generated makefile targets
- **git**: forge uses libgit2 internally, but having git installed is useful for manual operations

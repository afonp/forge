---
sidebar_position: 2
---

# installation

## cargo (recommended)

if you have rust installed:

```bash
cargo install forge-cp
```

## homebrew (macos / linux)

```bash
brew tap afonp/tap
brew install forge
```

## windows installer

download `forge-x86_64-pc-windows-msvc-setup.exe` from the [latest release](https://github.com/afonp/forge/releases/latest) and run it. it adds `forge` to your PATH automatically.

## pre-built binaries

download the archive for your platform from the [releases page](https://github.com/afonp/forge/releases/latest):

| platform | file |
|----------|------|
| linux x86_64 | `forge-x86_64-unknown-linux-gnu.tar.gz` |
| linux aarch64 | `forge-aarch64-unknown-linux-gnu.tar.gz` |
| macos intel | `forge-x86_64-apple-darwin.tar.gz` |
| macos apple silicon | `forge-aarch64-apple-darwin.tar.gz` |
| windows x86_64 | `forge-x86_64-pc-windows-msvc.zip` |

extract and place the `forge` binary somewhere in your `$PATH`.

## building from source

```bash
git clone https://github.com/afonp/forge.git
cd forge
cargo install --path .
```

## requirements

- **g++** — compiles your c++ solutions via the generated makefile
- **make** — runs the makefile targets

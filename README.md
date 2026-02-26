# forge

competitive programming exercise scaffolder with a c++ template system.

## quick start

```bash
# install
cargo install forge-cp

# install dependencies (g++, make) — auto-detects your os
forge setup

# create an exercise
forge new two-sum

# create a contest with multiple problems
forge new cf1900 a b c d
```

each exercise gets:

```
./two-sum/
├── solution.cpp    # c++ template with common data structures and algorithms
├── Makefile        # make / make run / make test / make debug
├── input.txt       # paste test input here
├── expected.txt    # paste expected output here
└── notes.md        # problem notes
```

everything is automatically committed to git.

## install

### cargo

```bash
cargo install forge-cp
```

### homebrew (macos / linux)

```bash
brew tap afonp/tap
brew install forge
```

### windows installer

download `forge-x86_64-pc-windows-msvc-setup.exe` from the [latest release](https://github.com/afonp/forge/releases/latest) and run it.

### pre-built binaries

grab the archive for your platform from [releases](https://github.com/afonp/forge/releases/latest), extract it, and put `forge` in your PATH.

## commands

```bash
forge new <name>                # create a single exercise
forge new <contest> a b c d     # create multiple problems for a contest
forge list                      # list exercises in the current directory
forge open <name>               # open in $EDITOR or vs code
forge clean <name>              # remove compiled binaries (make clean)
forge setup                     # install dependencies (g++, make)
```

### flags

```bash
forge new two-sum -c            # open in vs code after creation
forge new two-sum -o            # open in $EDITOR after creation
```

## working with exercises

```bash
cd two-sum
make          # compile
make run      # compile and run
make test     # compile and run with input.txt
make debug    # compile with sanitizers and run with input.txt
make clean    # remove binaries
```

the `debug` target enables address sanitizer, ub sanitizer, and the `dbg()` macro.

## c++ template

the built-in template includes:

- **types**: `ll`, `ull`, `ld`, `pii`, `pll`, `vi`, `vll`, `vvi`
- **constants**: `inf`, `linf`, `eps`, `mod`, `mod2`, `pi`
- **macros**: `all(x)`, `rep(i,a,b)`, `per(i,a,b)`, `each(x,v)`, `dbg(x)`
- **data structures**: graph (dijkstra, bfs), dsu, segment tree, fenwick tree
- **strings**: kmp, z-function
- **math**: gcd, lcm, modular exponentiation, modular inverse

works with both gcc and clang (macos).

customize it by editing `~/.cp/templates/template.cpp`.

## docs

full documentation at [forge.afpereira.me](https://forge.afpereira.me)

## license

MIT

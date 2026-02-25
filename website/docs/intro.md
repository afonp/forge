---
sidebar_position: 1
---

# introduction

forge is a competitive programming exercise scaffolder. it creates exercise folders with a c++ solution template, a makefile, test files, and problem notes — then automatically commits everything to git.

## why forge?

competitive programming involves a lot of repetitive setup: creating directories, copying templates, writing makefiles. forge automates all of it into a single command.

```bash
forge new two-sum
```

this creates:

```
~/cp/exercises/two-sum/
├── solution.cpp    # your solution (from the c++ template)
├── Makefile        # make / make run / make test / make debug
├── input.txt       # paste test input here
├── expected.txt    # paste expected output here
└── notes.md        # problem notes
```

and commits it to git automatically.

## what's in the template?

the built-in c++ template includes:

- **type aliases**: `ll`, `ull`, `ld`, `pii`, `pll`, `vi`, `vll`, `vvi`
- **constants**: `inf`, `linf`, `eps`, `mod`, `mod2`, `pi`
- **macros**: `all(x)`, `rep(i,a,b)`, `per(i,a,b)`, `each(x,v)`, `dbg(x)`
- **data structures**: graph (dijkstra, bfs), dsu, segment tree, fenwick tree
- **string algorithms**: kmp, z-function
- **math**: gcd, lcm, modular exponentiation, modular inverse

## quick start

```bash
# install
cargo install forge

# create your first exercise
forge new hello-world

# create a contest with multiple problems
forge new cf1900 a b c d
```

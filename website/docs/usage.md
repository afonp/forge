---
sidebar_position: 3
---

# usage

## create a single exercise

```bash
forge new two-sum
```

output:

```
[✓] created: ~/cp/exercises/two-sum
    solution.cpp   your solution
    Makefile       make / make run / make test / make debug
    input.txt      paste test input here
    expected.txt   paste expected output here
    notes.md       problem notes
[✓] committed: add exercise two-sum
```

## create a contest

```bash
forge new cf1900 a b c d
```

this creates `cf1900_a`, `cf1900_b`, `cf1900_c`, `cf1900_d` as separate exercises.

output:

```
[✓] created: ~/cp/exercises/cf1900_a
[✓] created: ~/cp/exercises/cf1900_b
[✓] created: ~/cp/exercises/cf1900_c
[✓] created: ~/cp/exercises/cf1900_d
[✓] committed: add contest cf1900 (a, b, c, d)
```

## open in editor after creation

```bash
# open in vs code
forge new two-sum -c

# open in $EDITOR
forge new two-sum -o
```

## list exercises

```bash
forge list
```

output:

```
  cf1900_a                       2024-01-15 14:30
  cf1900_b                       2024-01-15 14:30
  two-sum                        2024-01-15 14:25
[✓] 3 exercise(s)
```

## open an exercise

```bash
forge open two-sum
```

this opens the exercise folder in your editor. it tries `$EDITOR` first, then falls back to `code` (vs code), then prints the path.

## clean binaries

```bash
forge clean two-sum
```

runs `make clean` in the exercise directory and commits the change.

## working with exercises

once an exercise is created, use the makefile:

```bash
cd ~/cp/exercises/two-sum

# compile
make

# compile and run interactively
make run

# compile and run with input.txt as stdin
make test

# compile with debug flags (address sanitizer, ub sanitizer) and run with input.txt
make debug

# remove compiled binaries
make clean
```

the `debug` target compiles with `-g -fsanitize=address,undefined -DLOCAL`, which enables the `dbg()` macro and catches memory errors.

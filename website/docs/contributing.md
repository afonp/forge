---
sidebar_position: 6
---

# contributing

contributions are welcome! here's how to get started.

## setup

```bash
git clone https://github.com/afonp/forge.git
cd forge
cargo build
```

## project structure

```
src/
  main.rs              entry point, cli definition
  commands/
    mod.rs
    new.rs             scaffold logic
    list.rs            list exercises
    open.rs            open in editor
    clean.rs           run make clean
  template.rs          c++ template management
  git.rs               git helpers (init, stage, commit)
  utils.rs             shared helpers, colored output

assets/
  template.cpp         the c++ template (embedded at compile time)
```

## code style

- all rust code uses lowercase where possible
- all comments in english
- terminal output is lowercase
- struct/enum names follow rust conventions (PascalCase)

## running checks

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## submitting changes

1. fork the repository
2. create a branch for your change
3. make your changes
4. ensure `cargo fmt`, `cargo clippy`, and `cargo test` pass
5. open a pull request

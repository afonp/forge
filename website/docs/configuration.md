---
sidebar_position: 5
---

# configuration

## file locations

| path | description |
|------|-------------|
| `~/cp/exercises/` | all exercises live here |
| `~/.cp/templates/template.cpp` | the global c++ template |

on windows, `~` refers to `%USERPROFILE%` (usually `C:\Users\<username>`).

## customizing the template

the c++ template is stored at `~/.cp/templates/template.cpp`. forge creates this file automatically on first run from the built-in default.

to customize it, simply edit the file:

```bash
$EDITOR ~/.cp/templates/template.cpp
```

every new exercise will use your customized template from that point on. existing exercises are not affected.

to reset back to the default template, delete the file and forge will recreate it on the next `forge new`:

```bash
rm ~/.cp/templates/template.cpp
forge new test-reset
```

## git behavior

forge automatically manages a git repository inside `~/cp/exercises/`:

- on first run, if the directory is not a git repo, forge initializes one
- after `forge new`: all new files are staged and committed
- after `forge clean`: changes are staged and committed
- commit messages are always lowercase, e.g. `add exercise two-sum`

if git operations fail (permissions, missing config), forge warns but does not fail — your exercise is still created.

## editor integration

the `forge open` command and `-c`/`-o` flags use editors in this priority:

1. `-c` / `--code`: always opens in vs code (`code`)
2. `-o` / `--editor`: opens in `$EDITOR`, falls back to vs code
3. `forge open`: tries `$EDITOR`, then `code`, then prints the path

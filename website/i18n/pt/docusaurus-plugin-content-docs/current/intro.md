---
sidebar_position: 1
---

# introdução

forge é um scaffolder de exercícios de programação competitiva. cria pastas de exercício com um template c++ de solução, um makefile, ficheiros de teste e notas de problema — e faz commit de tudo automaticamente para git.

## porquê forge?

programação competitiva envolve muita configuração repetitiva: criar diretórios, copiar templates, escrever makefiles. forge automatiza tudo num único comando.

```bash
forge new two-sum
```

isto cria:

```
./two-sum/
├── solution.cpp    # a tua solução (do template c++)
├── Makefile        # make / make run / make test / make debug
├── input.txt       # cola o input de teste aqui
├── expected.txt    # cola o output esperado aqui
└── notes.md        # notas do problema
```

e faz commit para git automaticamente.

## o que está no template?

o template c++ inclui:

- **type aliases**: `ll`, `ull`, `ld`, `pii`, `pll`, `vi`, `vll`, `vvi`
- **constantes**: `inf`, `linf`, `eps`, `mod`, `mod2`, `pi`
- **macros**: `all(x)`, `rep(i,a,b)`, `per(i,a,b)`, `each(x,v)`, `dbg(x)`
- **estruturas de dados**: graph (dijkstra, bfs), dsu, segment tree, fenwick tree
- **algoritmos de strings**: kmp, z-function
- **matemática**: gcd, lcm, exponenciação modular, inverso modular

## início rápido

```bash
# instalar
cargo install forge-cp

# instalar dependências (g++, make)
forge setup

# criar o primeiro exercício
forge new hello-world

# criar um contest com vários problemas
forge new cf1900 a b c d
```

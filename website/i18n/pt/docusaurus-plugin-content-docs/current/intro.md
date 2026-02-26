---
sidebar_position: 1
---

# introducao

forge e um scaffolder de exercicios de programacao competitiva. cria pastas de exercicio com um template c++ de solucao, um makefile, ficheiros de teste e notas de problema — e faz commit de tudo automaticamente para git.

## porque forge?

programacao competitiva envolve muita configuracao repetitiva: criar diretorios, copiar templates, escrever makefiles. forge automatiza tudo num unico comando.

```bash
forge new two-sum
```

isto cria:

```
./two-sum/
├── solution.cpp    # a tua solucao (do template c++)
├── Makefile        # make / make run / make test / make debug
├── input.txt       # cola o input de teste aqui
├── expected.txt    # cola o output esperado aqui
└── notes.md        # notas do problema
```

e faz commit para git automaticamente.

## o que esta no template?

o template c++ inclui:

- **type aliases**: `ll`, `ull`, `ld`, `pii`, `pll`, `vi`, `vll`, `vvi`
- **constantes**: `inf`, `linf`, `eps`, `mod`, `mod2`, `pi`
- **macros**: `all(x)`, `rep(i,a,b)`, `per(i,a,b)`, `each(x,v)`, `dbg(x)`
- **estruturas de dados**: graph (dijkstra, bfs), dsu, segment tree, fenwick tree
- **algoritmos de strings**: kmp, z-function
- **matematica**: gcd, lcm, exponenciacao modular, inverso modular

## inicio rapido

```bash
# instalar
cargo install forge-cp

# criar o primeiro exercicio
forge new hello-world

# criar um contest com varios problemas
forge new cf1900 a b c d
```

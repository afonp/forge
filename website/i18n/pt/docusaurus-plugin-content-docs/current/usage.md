---
sidebar_position: 3
---

# utilizacao

## criar um exercicio

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

## criar um contest

```bash
forge new cf1900 a b c d
```

isto cria `cf1900_a`, `cf1900_b`, `cf1900_c`, `cf1900_d` como exercicios separados.

## abrir no editor apos criacao

```bash
# abrir no vs code
forge new two-sum -c

# abrir no $EDITOR
forge new two-sum -o
```

## listar exercicios

```bash
forge list
```

## abrir um exercicio

```bash
forge open two-sum
```

tenta `$EDITOR` primeiro, depois `code` (vs code), depois imprime o caminho.

## limpar binarios

```bash
forge clean two-sum
```

executa `make clean` no diretorio do exercicio e faz commit da alteracao.

## trabalhar com exercicios

uma vez criado o exercicio, usa o makefile:

```bash
cd ~/cp/exercises/two-sum

# compilar
make

# compilar e executar interativamente
make run

# compilar e executar com input.txt como stdin
make test

# compilar com flags de debug e executar com input.txt
make debug

# remover binarios compilados
make clean
```

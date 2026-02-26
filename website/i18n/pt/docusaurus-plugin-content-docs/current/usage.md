---
sidebar_position: 3
---

# utilização

## criar um exercício

```bash
forge new two-sum
```

output:

```
[✓] created: ./two-sum
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

isto cria `cf1900_a`, `cf1900_b`, `cf1900_c`, `cf1900_d` como exercícios separados.

## abrir no editor após criação

```bash
# abrir no vs code
forge new two-sum -c

# abrir no $EDITOR
forge new two-sum -o
```

## listar exercícios

```bash
forge list
```

## abrir um exercício

```bash
forge open two-sum
```

tenta `$EDITOR` primeiro, depois `code` (vs code), depois imprime o caminho.

## limpar binários

```bash
forge clean two-sum
```

executa `make clean` no diretório do exercício e faz commit da alteração.

## instalar dependências

```bash
forge setup
```

verifica se tens g++ e make instalados. se não tiveres, instala automaticamente.

## trabalhar com exercícios

uma vez criado o exercício, usa o makefile:

```bash
cd ./two-sum

# compilar
make

# compilar e executar interativamente
make run

# compilar e executar com input.txt como stdin
make test

# compilar com flags de debug e executar com input.txt
make debug

# remover binários compilados
make clean
```

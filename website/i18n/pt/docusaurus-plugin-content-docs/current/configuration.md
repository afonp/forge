---
sidebar_position: 5
---

# configuracao

## localizacao dos ficheiros

| caminho | descricao |
|---------|-----------|
| `~/cp/exercises/` | todos os exercicios ficam aqui |
| `~/.cp/templates/template.cpp` | o template c++ global |

no windows, `~` refere-se a `%USERPROFILE%` (normalmente `C:\Users\<username>`).

## personalizar o template

o template c++ esta em `~/.cp/templates/template.cpp`. forge cria este ficheiro automaticamente na primeira execucao a partir do default embutido.

para personalizar, basta editar o ficheiro:

```bash
$EDITOR ~/.cp/templates/template.cpp
```

cada novo exercicio usara o teu template personalizado a partir desse momento. exercicios existentes nao sao afetados.

para repor o template default, apaga o ficheiro e forge recria-o no proximo `forge new`:

```bash
rm ~/.cp/templates/template.cpp
forge new test-reset
```

## comportamento do git

forge gere automaticamente um repositorio git dentro de `~/cp/exercises/`:

- na primeira execucao, se o diretorio nao for um repo git, forge inicializa um
- apos `forge new`: todos os novos ficheiros sao staged e committed
- apos `forge clean`: as alteracoes sao staged e committed
- as mensagens de commit sao sempre em minusculas, e.g. `add exercise two-sum`

se as operacoes git falharem, forge avisa mas nao falha — o teu exercicio e criado na mesma.

## integracao com editor

o comando `forge open` e as flags `-c`/`-o` usam editores nesta prioridade:

1. `-c` / `--code`: abre sempre no vs code (`code`)
2. `-o` / `--editor`: abre no `$EDITOR`, fallback para vs code
3. `forge open`: tenta `$EDITOR`, depois `code`, depois imprime o caminho

---
sidebar_position: 5
---

# configuração

## localização dos ficheiros

| caminho | descrição |
|---------|-----------|
| `./` | os exercícios são criados no diretório atual |
| `~/.cp/templates/template.cpp` | o template c++ global |

no windows, `~` refere-se a `%USERPROFILE%` (normalmente `C:\Users\<username>`).

## personalizar o template

o template c++ está em `~/.cp/templates/template.cpp`. forge cria este ficheiro automaticamente na primeira execução a partir do default embutido.

para personalizar, basta editar o ficheiro:

```bash
$EDITOR ~/.cp/templates/template.cpp
```

cada novo exercício usará o teu template personalizado a partir desse momento. exercícios existentes não são afetados.

para repor o template default, apaga o ficheiro e forge recria-o no próximo `forge new`:

```bash
rm ~/.cp/templates/template.cpp
forge new test-reset
```

## comportamento do git

forge gere automaticamente um repositório git no diretório atual:

- na primeira execução, se o diretório não for um repo git, forge inicializa um
- após `forge new`: todos os novos ficheiros são staged e committed
- após `forge clean`: as alterações são staged e committed
- as mensagens de commit são sempre em minúsculas, e.g. `add exercise two-sum`

se as operações git falharem, forge avisa mas não falha — o teu exercício é criado na mesma.

## integração com editor

o comando `forge open` e as flags `-c`/`-o` usam editores nesta prioridade:

1. `-c` / `--code`: abre sempre no vs code (`code`)
2. `-o` / `--editor`: abre no `$EDITOR`, fallback para vs code
3. `forge open`: tenta `$EDITOR`, depois `code`, depois imprime o caminho

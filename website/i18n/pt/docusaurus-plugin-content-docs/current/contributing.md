---
sidebar_position: 6
---

# contribuir

contribuicoes sao bem-vindas! aqui esta como comecar.

## setup

```bash
git clone https://github.com/afonp/forge.git
cd forge
cargo build
```

## estrutura do projeto

```
src/
  main.rs              ponto de entrada, definicao do cli
  commands/
    mod.rs
    new.rs             logica de scaffold
    list.rs            listar exercicios
    open.rs            abrir no editor
    clean.rs           executar make clean
  template.rs          gestao do template c++
  git.rs               helpers git (init, stage, commit)
  utils.rs             helpers partilhados, output colorido

assets/
  template.cpp         o template c++ (embutido em compile time)
```

## estilo de codigo

- todo o codigo rust usa minusculas onde possivel
- todos os comentarios em ingles
- output do terminal em minusculas
- nomes de struct/enum seguem convencoes rust (PascalCase)

## executar verificacoes

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## submeter alteracoes

1. faz fork do repositorio
2. cria um branch para a tua alteracao
3. faz as tuas alteracoes
4. garante que `cargo fmt`, `cargo clippy` e `cargo test` passam
5. abre um pull request

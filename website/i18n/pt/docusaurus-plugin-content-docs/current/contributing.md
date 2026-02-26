---
sidebar_position: 6
---

# contribuir

contribuições são bem-vindas! aqui está como começar.

## setup

```bash
git clone https://github.com/afonp/forge.git
cd forge
cargo build
```

## estrutura do projeto

```
src/
  main.rs              ponto de entrada, definição do cli
  commands/
    mod.rs
    new.rs             lógica de scaffold
    list.rs            listar exercícios
    open.rs            abrir no editor
    clean.rs           executar make clean
    setup.rs           instalar dependências
  template.rs          gestão do template c++
  git.rs               helpers git (init, stage, commit)
  utils.rs             helpers partilhados, output colorido

assets/
  template.cpp         o template c++ (embutido em compile time)
```

## estilo de código

- todo o código rust usa minúsculas onde possível
- todos os comentários em inglês
- output do terminal em minúsculas
- nomes de struct/enum seguem convenções rust (PascalCase)

## executar verificações

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## submeter alterações

1. faz fork do repositório
2. cria um branch para a tua alteração
3. faz as tuas alterações
4. garante que `cargo fmt`, `cargo clippy` e `cargo test` passam
5. abre um pull request

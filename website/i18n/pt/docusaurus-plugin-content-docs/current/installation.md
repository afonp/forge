---
sidebar_position: 2
---

# instalacao

## cargo (recomendado)

se tens rust instalado, podes instalar forge diretamente do crates.io:

```bash
cargo install forge-cp
```

## compilar a partir do codigo fonte

```bash
git clone https://github.com/afonp/forge.git
cd forge
cargo install --path .
```

## binarios pre-compilados

faz download da ultima release para a tua plataforma na [pagina de releases](https://github.com/afonp/forge/releases).

plataformas disponiveis:
- **linux**: x86_64, aarch64
- **macos**: x86_64, aarch64 (apple silicon)
- **windows**: x86_64

extrai o arquivo e coloca o binario `forge` algures no teu `$PATH`.

## requisitos

- **rust** (para cargo install): 1.70+
- **g++**: necessario para compilar as tuas solucoes c++ via o makefile gerado
- **make**: necessario para os targets do makefile gerado
- **git**: forge usa libgit2 internamente, mas ter git instalado e util para operacoes manuais

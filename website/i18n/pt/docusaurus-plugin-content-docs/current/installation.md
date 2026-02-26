---
sidebar_position: 2
---

# instalacao

## cargo (recomendado)

se tens rust instalado:

```bash
cargo install forge-cp
```

## homebrew (macos / linux)

```bash
brew tap afonp/tap
brew install forge
```

## instalador windows

faz download de `forge-x86_64-pc-windows-msvc-setup.exe` da [ultima release](https://github.com/afonp/forge/releases/latest) e executa. adiciona `forge` ao teu PATH automaticamente.

## binarios pre-compilados

faz download do arquivo para a tua plataforma na [pagina de releases](https://github.com/afonp/forge/releases/latest):

| plataforma | ficheiro |
|------------|----------|
| linux x86_64 | `forge-x86_64-unknown-linux-gnu.tar.gz` |
| linux aarch64 | `forge-aarch64-unknown-linux-gnu.tar.gz` |
| macos intel | `forge-x86_64-apple-darwin.tar.gz` |
| macos apple silicon | `forge-aarch64-apple-darwin.tar.gz` |
| windows x86_64 | `forge-x86_64-pc-windows-msvc.zip` |

extrai e coloca o binario `forge` algures no teu `$PATH`.

## compilar a partir do codigo fonte

```bash
git clone https://github.com/afonp/forge.git
cd forge
cargo install --path .
```

## requisitos

- **g++** — compila as tuas solucoes c++ via o makefile gerado
- **make** — executa os targets do makefile

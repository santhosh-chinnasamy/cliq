# CLIQ

open frequently accessed memorable links from cli

> Cliq is like your bookmark manager from terminal. You can add your frequently accessed urls in `cliq.toml` file and open them from terminal. It also supports opening git remote repo if `.git` exists in the current directory.

## Installation

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/santhosh-chinnasamy/cliq?label=Latest%20Version)](https://github.com/santhosh-chinnasamy/cliq/releases/latest)

Download latest version from [release](https://github.com/santhosh-chinnasamy/cliq/releases) page and run below command

```bash
tar -xvf cliq-<VERSION>-<PLATFORM>.tar.gz -C /usr/local/bin
```

## Requirements

Create `cliq.toml` file under `$HOME/.config/cliq/` folder and add the following contents

```toml
[links]
google = "https://google.com"
hub = "https://github.com"
lab = "https://gitlab.com"
```

## Usage

```bash
cliq google # opens google
cliq hub # opens github
cliq lab # opens gitlab

 # opens git remote repo if .git exists. no need to add it in cliq.toml
cliq git
```

## Build and Install

```bash
cargo build
cargo install --path .
```

## Supports

- [x] Mac
- [x] Linux
- [ ] Windows

## Contribution

Please read the [contributing guidelines](CONTRIBUTING.md) to setup your development machine and proceed.

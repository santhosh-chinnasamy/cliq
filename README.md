# CLIQ

open frequently accessed memorable shorten urls from cli

## Installation

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

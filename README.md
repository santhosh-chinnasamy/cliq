# CLIQ

open frequently accessed memorable shorten urls from cli

## Build and Install

```bash
cargo build
cargo install --path .
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

## Supports

- [x] Mac
- [ ] Linux
- [ ] Windows



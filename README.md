# CLIK

open frequently accessed memorable shorten urls from cli

## Build and Install

```bash
cargo build
cargo install --path .
```

## Requirements

Create `clik.toml` file under `$HOME/.config/clik/` folder and add the following contents

```toml
[links]
google = "https://google.com"
hub = "https://github.com"
lab = "https://gitlab.com"
```

## Usage

```bash
clik google # opens google
clik hub # opens github
clik lab # opens gitlab
```

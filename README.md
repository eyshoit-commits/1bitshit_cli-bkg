# 1bitshit_cli-bkg

User interface layer. CLI commands and interactive TUI dashboard.

## Architecture

```
1bitshit_cli-bkg/
└── cmd/    # CLI + TUI application (bitshit-cli)
    ├── src/
    │   ├── main.rs          # Entry point, CLI argument routing
    │   ├── core/            # App state machine, bootstrapper
    │   ├── ui/              # Ratatui TUI components
    │   ├── cli/             # CLI subcommand handlers
    │   ├── assets/          # ASCII art logos, branding
    │   ├── theme.rs         # Terminal colors
    │   └── app_enums.rs     # Tab/Mode enums
    └── build.rs             # Windows resource embedding
```

## Binary

| Binary    | Description                |
| --------- | -------------------------- |
| `bitshit` | Main CLI + TUI application |

## CLI Commands

| Command                         | Description                      |
| ------------------------------- | -------------------------------- |
| `bitshit`                       | Launch interactive TUI dashboard |
| `bitshit serve`                 | Start background API server      |
| `bitshit run <model>`           | Direct headless inference        |
| `bitshit pull <model>`          | Download a model                 |
| `bitshit list`                  | List installed models            |
| `bitshit rm <model>`            | Remove a model                   |
| `bitshit booster`               | Hardware tuning                  |
| `bitshit benchmark`             | Performance benchmarks           |
| `bitshit skill install <name>`  | Install a skill                  |
| `bitshit plugin install <name>` | Install a plugin                 |
| `bitshit ingest`                | RAG document ingestion           |
| `bitshit setup`                 | Node profile setup               |
| `bitshit config set`            | Configuration management         |

## TUI Features

- **Chat App**: Streaming inference with mid-generation pivot (Ctrl+C)
- **Roster App**: Model management and download progress
- **Settings App**: Hardware and API configuration
- **Registry App**: Skill/plugin marketplace
- **Help App**: Command reference
- **Palette**: Quick command access

## Dependencies

- `bitshit-engine-core` (from engine)
- `bitshit-engine-api` (from engine)
- `bitshit-shared` (from kernel)
- `bitshit-driver-dispatcher` (from driver)
- `ratatui`, `crossterm`, `clap`, `figlet-rs`

## Build

```bash
cargo build -p bitshit-cli
```

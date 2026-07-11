# 1bitshit_cli-bkg

User interface. CLI commands and interactive TUI dashboard.

## What this module contains

- `cmd/` - CLI + TUI application (bitshit binary)

## Status

Alpha. Basic CLI and TUI work. Many commands are stubs.

## Structure

```
1bitshit_cli-bkg/
└── cmd/
    └── src/
        ├── main.rs          # Entry point
        ├── core/            # App state, bootstrapper
        ├── ui/              # Ratatui components
        ├── cli/             # CLI subcommands
        ├── assets/          # Logos, branding
        └── theme.rs         # Colors
```

## CLI Commands

| Command                 | Description              | Status       |
| ----------------------- | ------------------------ | ------------ |
| `bitshit`               | Launch TUI dashboard     | Working      |
| `bitshit run <model>`   | Direct inference         | Working      |
| `bitshit pull <model>`  | Download model           | Working      |
| `bitshit list`          | List models              | Working      |
| `bitshit rm <model>`    | Remove model             | Working      |
| `bitshit ps`            | List running processes   | Working      |
| `bitshit setup`         | Configuration wizard     | Working      |
| `bitshit booster`       | Hardware tuning          | Working      |
| `bitshit benchmark`     | Run benchmarks           | Experimental |
| `bitshit component`     | Component management     | Experimental |
| `bitshit ingest <file>` | File ingestion           | Experimental |
| `bitshit test_jit`      | JIT testing              | Experimental |
| `bitshit config`        | Configuration management | Experimental |

## What is experimental

- TUI streaming display
- Mid-generation pivot (Ctrl+C)
- Booster configuration
- Benchmark commands

## Build

```bash
cargo build -p bitshit-cli
```

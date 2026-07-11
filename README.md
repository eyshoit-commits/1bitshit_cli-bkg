# 1bitshit CLI (`bitshit-cli`)

The **user-facing console / dashboard** of the 1bitshit stack ("Neural OS"
Dashboard). Wraps the engine, driver and kernel behind a bare-metal CLI.

## Package
- `cmd/` → `bitshit-cli` — the `bitshit` binary (skills, benchmark, `run`, vault/identity, RAG ingest, model registry).

## Features
- Model registry browser, Hugging Face pull, local GGUF resolve, refresh.
- Neural Skills & sandboxed agents, vault management & node identity.
- `run <model>`, `benchmark`, RAG semantic chunking via ONNX gatekeepers.

## Dependencies
- **engine** (`bitshit-engine-core`, `bitshit-engine-api`)
- **driver** (`bitshit-driver-dispatcher`)
- **kernel** (`bitshit-shared`)
  as git deps:
  ```toml
  bitshit-engine-core      = { git = "https://github.com/bkgoder/1bitshit_engine-bkg", package = "bitshit-engine-core",      branch = "main" }
  bitshit-engine-api       = { git = "https://github.com/bkgoder/1bitshit_engine-bkg", package = "bitshit-engine-api",       branch = "main" }
  bitshit-driver-dispatcher = { git = "https://github.com/bkgoder/1bitshit_driver-bkg", package = "bitshit-driver-dispatcher", branch = "main" }
  bitshit-shared           = { git = "https://github.com/bkgoder/1bitshit_kernel-bkg", package = "bitshit-shared",           branch = "main" }
  ```
- Independent build:
  ```bash
  cargo build --release
  ```

## Position in the stack
```
kernel  <--  driver  <--  engine  <--  cli
```

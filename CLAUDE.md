# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Project Overview

Resurgence is a 3D game built with Rust and the Bevy game engine (v0.16.1).
It's in experimental phase, focusing on gameplay mechanics around terrain
interaction and resource management in a post-apocalyptic Earth reclamation
scenario.

## Code Standards

- **Always use Bevy 0.16.1 features and idioms**
- After every code change, run `cargo clippy` and `cargo fmt`
- After editing any markdown file, run `markdownlint-cli2 *.md **/*.md`

**Important**: Resolve all errors and warnings

## Common Commands

```bash

# Releasing:
# - Do native and WASM builds as a smoke test
# - Bump version in Cargo.toml
# - commit and tags with new version
# - Do NOT push code or tags, let the user do that

# Native build
cargo clippy      # Run linter - fix all warnings
cargo fmt         # Format code
cargo check       # Quick syntax check
cargo build       # Build the project
# Do NOT run the project, let the user do that

# Markdown commands
markdownlint-cli2 *.md **/*.md      # Lint all markdown files
markdownlint-cli2 specific.md      # Lint specific file

# GitHub workflow management
gh workflow list           # List all workflows
gh workflow run CI         # Manually trigger CI
gh workflow run Release    # Manually trigger release
gh workflow view CI        # Check CI status
gh workflow view Release   # Check release status

# WASM build commands
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --out-name resurgence --out-dir wasm --target web target/wasm32-unknown-unknown/release/resurgence.wasm
# Do NOT start the server, let the user do that
```

## Architecture Overview

### Core Structure

- **ECS Architecture**: Uses Bevy's Entity Component System pattern
- **Main Entry**: `src/main.rs` - Launcher system for accessing experiments
- **Experiments**: `src/experiments/` contains integrated experiment modules

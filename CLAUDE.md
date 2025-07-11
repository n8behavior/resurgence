# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with
code in this repository.

## Project Overview

Resurgence is a 3D game built with Rust and the Bevy game engine (v0.16.1).
It's in experimental phase, focusing on gameplay mechanics around terrain
interaction and resource management in a post-apocalyptic Earth reclamation
scenario.

## Code Standards

### Bevy Version Compliance

- **Always use Bevy 0.16.1 features and idioms**
- Prefer modern Bevy patterns:
  - Use `Single<>` for single-entity queries
  - Use new component syntax (e.g., `Mesh3d`, `MeshMaterial3d`)
  - Follow Bevy's latest ECS best practices

### Required Workflow

#### For Code Changes

After every code change, run these commands in order:

1. `cargo clippy` - Fix all warnings or add appropriate suppressions
2. `cargo fmt` - Ensure consistent formatting
3. `cargo run` - Test the changes

**Important**: Never run the game without first passing clippy and fmt checks.

#### After Pushing Code

After `git push`, always check CI/CD status:

1. `gh workflow view CI` - Monitor CI build and test status
2. `gh workflow view Release` - Check release pipeline status

**Important**: Ensure all workflows pass before considering the push complete.

#### For Markdown Changes

After editing any markdown file, run:

1. `markdownlint-cli2 *.md **/*.md` - Fix all markdown formatting issues
2. Review and fix any reported problems

**Important**: All markdown files must pass linting before committing.

## Common Commands

```bash
# Required workflow for code changes
cargo clippy && cargo fmt && cargo run

# Required workflow for markdown changes
markdownlint-cli2 *.md **/*.md

# After git push - monitor CI/CD
gh workflow view CI        # Check CI status
gh workflow view Release   # Check release status

# Individual commands
cargo clippy      # Run linter - fix all warnings
cargo fmt         # Format code
cargo check       # Quick syntax check
cargo build       # Build the project
cargo run         # Run the main game

# Markdown commands
markdownlint-cli2 *.md **/*.md      # Lint all markdown files
markdownlint-cli2 specific.md      # Lint specific file

# GitHub CLI workflow management
gh workflow list           # List all workflows
gh workflow run CI         # Manually trigger CI
gh workflow run Release    # Manually trigger release

# Run with optimizations (better performance)
cargo run --release

# WASM build commands
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir wasm --target web target/wasm32-unknown-unknown/release/resurgence.wasm
```

## Architecture Overview

### Core Structure

- **ECS Architecture**: Uses Bevy's Entity Component System pattern
- **Main Entry**: `src/main.rs` - Launcher system for accessing experiments
- **Experiments**: `src/experiments/` contains integrated experiment modules

### Key Systems

1. **Terrain System**:
   - Large subdivided mesh planes for terrain
   - Procedural generation using Perlin noise (see `terrain_proc_gen.rs`)
   - Height-based coloring system

2. **Camera System**:
   - Pan/orbit camera using `bevy_panorbit_camera`
   - Third-person following camera in terrain example

3. **Input System**:
   - Mouse raycast for terrain interaction
   - Keyboard controls for movement (WASD in examples)

### Game Mechanics (In Development)

- Growth spreading system (red patches on terrain)
- Resource collection from growth origins
- Strategic positioning and territory control

## Development Notes

- **Code Quality**: Always run `cargo clippy` and `cargo fmt` before testing
  changes
- **Markdown Quality**: Always run `markdownlint-cli2` on markdown files
  before committing
- **Bevy Idioms**: Use latest Bevy 0.16.1 patterns and features throughout
  the codebase
- **Optimization**: Dev builds use opt-level 1, dependencies use opt-level 3
  for faster iteration
- **Linker**: Configured to use clang with LLD for faster builds on Linux
- **WebAssembly**: Project has WASM support (see `wasm/` directory)
- **Experiments**: Follow experiment-driven development using launcher system
- **GitHub Organization**: Repository is under `n8behavior` org
- **CI/CD**: Always monitor workflow status after pushing code

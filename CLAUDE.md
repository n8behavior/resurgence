# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Resurgence is a 3D game built with Rust and the Bevy game engine (v0.16.1). It's in experimental phase, focusing on gameplay mechanics around terrain interaction and resource management in a post-apocalyptic Earth reclamation scenario.

## Common Commands

```bash
# Build the project
cargo build

# Run the main game
cargo run

# Run with optimizations (better performance)
cargo run --release

# Run specific examples
cargo run --example terrain_proc_gen

# Check code (no dedicated lint command found, use standard Rust tooling)
cargo check
cargo clippy
```

## Architecture Overview

### Core Structure
- **ECS Architecture**: Uses Bevy's Entity Component System pattern
- **Main Entry**: `src/main.rs` - Simple terrain with click-to-spawn growth mechanics
- **Examples**: `examples/` contains isolated experiments for testing mechanics

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

- **Optimization**: Dev builds use opt-level 1, dependencies use opt-level 3 for faster iteration
- **Linker**: Configured to use clang with LLD for faster builds on Linux
- **WebAssembly**: Project has WASM support (see `wasm/` directory)
- **Experiments**: Follow experiment-driven development - test mechanics in isolation before integration
# Experiments

Self-contained "mini-projects" to quickly validate core gameplay ideas and
mechanics. Each should be small enough to tackle in a day or two, yet focused
enough to teach something concrete about your game's feel, UX, or tech
feasibility.

## How to Test

### Latest Release (Recommended for general testing)

- **Visit**: <https://n8behavior.itch.io/resurgence>
- **Browser**: Works in any modern browser (Chrome, Firefox, Safari, Edge)
- **Note**: This is the stable release version

### Latest Code (For developers/bleeding edge)

The latest ideas are being developed as a series of Experiments run from the
launcher. You can run the launcher as native code or WASM.

1. **Select an experiment**: Click on any experiment button in the launcher UI
1. **Test the mechanics**: Follow the specific instructions for each experiment below
1. **Return to launcher**: Press `ESC` at any time to return to the main menu

#### Native

```bash
cargo run --release
```

#### WASM

```bash
cargo run --release --target wasm32-unknown-unknown
```

_Note: You need to `cargo install wasm-server-runner`, if already installed_

## Approach

1. Implement experiments in the integrated launcher system
2. Gather quick metrics or feedback from testers
3. Iterate or pivot based on what we learn
4. Layer these mechanics together in a composite prototype

---

## Implementing Experiments

To create a new experiment, choose one from the [experiments
list](./experiments.md) and follow these basic steps:

1. Create a new module, e.g. `./src/experiments/some_experiment.rs` and add it
   to `./src/experiments/mod.rs`
1. In **some_experiment.rs**, create a `struct` for the experiment and `impl
Experiment for SomeExperiment`
1. In **mod.rs**, add an `AppState` variant, e.g. `SomeExperiment`
1. In **mod.rs**, register your experiment in `all_experiments()`

---

## Implemented Experiments

### Procedural Terrain Generation ✅

- **Access:** Launch game → Click "Procedural Terrain Generation"
- **Controls:**
  - WASD - Move ship
  - Mouse - Orbit camera around ship
  - Space - Toggle wireframe view
  - ESC - Return to launcher
- **What to test:** How ridge-lines, plateaus, and canyons look at playable
  scale. Do the noise parameters create strategic chokepoints?
- **Focus areas:** Visual clarity of terrain features, performance with 3x3
  chunk system, camera smoothness
- **Known limitations:** Fixed 3x3 grid, no infinite terrain yet

---

### Crimson Sprawl (original attempt) ✅

- **Access:** Launch game → Click "Growth-Type Overlay Demo"
- **Controls:**
  - Left-click - Place growth origin on terrain
  - ESC - Return to launcher
- **What to test:** Visual clarity of growth spread patterns, color palette
  effectiveness, performance with multiple growth origins
- **Focus areas:**
  - Is the growth spread visually clear and intuitive?
  - Does the grid alignment feel natural or too rigid?
  - Is the growth rate too fast/slow?
  - How does the color transition (red to black) communicate age?
  - Can you easily distinguish overlapping growth areas?
- **Known limitations:** No growth removal, infinite spread, fixed camera
  view

---

## Future Experiments

### Strike-Radius Targeting UI

- **What:** Load a static map texture, overlay an adjustable circular
  crosshair. Allow mouse-drag to resize radius and preview impact area.
- **Why:** Nail down the interaction flow for "choose where to fire." Make
  sure it's precise but not fiddly.
- **Metric:** Time to set a target (seconds) and number of accidental
  mis-sized strikes in user tests.

### Simple Skirmish Encounter

- **What:** Place a player avatar and a handful of AI agents on a flat arena.
  Implement basic pathfinding (A\*), line-of-sight checks, and attack logic.
- **Why:** See if combat feels responsive, whether AI is challenging but not
  overwhelming, and how resource hits/registers look.
- **Metric:** Average fight duration and player hit-rate vs agent hit-rate.

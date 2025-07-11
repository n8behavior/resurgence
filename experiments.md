# Experiments

Self-contained "mini-projects" to quickly validate core gameplay ideas and
mechanics. Each should be small enough to tackle in a day or two, yet focused
enough to teach something concrete about your game's feel, UX, or tech
feasibility.

## How to Test

1. **Launch the game**: Run `cargo run` to start the experiment launcher
2. **Select an experiment**: Click on any experiment button in the launcher UI
3. **Test the mechanics**: Follow the specific instructions for each experiment below
4. **Return to launcher**: Press `ESC` at any time to return to the main menu
5. **Test WASM build**: Use
   `cargo build --release --target wasm32-unknown-unknown &&
   wasm-bindgen --out-dir wasm --target web
   target/wasm32-unknown-unknown/release/resurgence.wasm`
   then serve the `wasm/` directory

## Approach

1. Implement experiments in the integrated launcher system
2. Gather quick metrics or feedback from testers
3. Iterate or pivot based on what we learn
4. Layer these mechanics together in a composite prototype

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

### Growth-Type Overlay Demo ✅

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

---

## Resource Collection & Conversion

- **What:** Scatter "resource nodes" (rocks, growth pods) that can be clicked
  or collided with to harvest. Add a simple UI counter/inventory and a
  conversion step ("pod → material").
- **Why:** Test whether the collection loop (find → collect → convert) is
  satisfying or too grindy.
- **Metric:** Number of clicks or key-presses per unit resource, and time to
  process 100 units.

---

## Panel-Storyboard Generator

- **What:** Create a mock "timeline" UI where you feed it a sequence of
  in-game events (e.g., first encounter, harvest, defeat) and it spits out
  placeholders for graphic panels.
- **Why:** Prototype how players pick moments for their "graphic novel"
  review. Validate UX: is picking and re-rolling intuitive?
- **Metric:** Successful panel selection rate and user satisfaction in a quick
  hallway test.

---

## Projectile Arc & Impact Markers

- **What:** Implement a ballistic projectile (gravity + drag) that the player
  can lo-click and drag to aim, showing a dotted trajectory and landing
  marker.
- **Why:** See if the physics-based aiming feels natural, and whether the arc
  visualization gives players the right feedback.
- **Metric:** Accuracy of first-shot landings vs target, and user ability to
  adjust arc mid-test.

---

## Rugged Terrain Pathfinding

- **What:** Take your terrain slice and overlay a simple grid graph. Run A\*
  to find a path between two points, penalizing steep slopes. Visualize the
  path.
- **Why:** Determine whether automated navigation (or enemy AI) can handle
  your map complexity, and how steepness weighting needs tuning.
- **Metric:** Path length vs optimal straight-line, and failure rate on
  extreme maps.

---

## Interactive Material Inspection

- **What:** Spawn a "discovered material" node that, when clicked, brings up
  a 3D inspect panel showing stats/description.
- **Why:** Prototype your inspection UI flow—does it break immersion? Is the
  info panel clear and actionable?
- **Metric:** Time to find a material, inspect it, and return to play,
  measured in seconds.

---

## HUD & Input Mapping

- **What:** Build a minimal UI overlay showing health, resource count, and
  current tool. Implement keyboard/mouse binding menus and live remapping.
- **Why:** Verify that your core HUD layout doesn't obscure the world and
  that input customization works before you build more features.
- **Metric:** Percentage of first-time testers who can remap keys and read
  their status without a manual.

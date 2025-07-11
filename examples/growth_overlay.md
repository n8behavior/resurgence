# Growth-Type Overlay Demo

## 🌱 Growth System Experiment - v0.0.6

This experiment implements the **Growth-Type Overlay Demo** from our experiments
list. It's a prototype of the organic growth mechanics that will eventually
represent the alien ecology spreading across Earth.

## What's New

- **Grid-based growth system** with radial expansion
- **Visual aging** - growth patches transition from bright red (new) to dark
  red/black (mature)
- **Alpha-blended edges** - growth fades out naturally at the boundaries
- **Optimized simulation** running at 5Hz for smooth performance

## How to Play

1. **Left-click** anywhere on the green terrain to place a growth origin
2. Watch as the growth **spreads outward** in a circular pattern
3. Each growth spot **matures over time**, changing color from red to black
4. Growth patches further from the origin appear **more transparent**
5. Try creating **multiple origins** to see overlapping growth patterns!

## Controls

- **Left Mouse Button** - Place new growth origin
- That's it! This experiment focuses purely on the growth visualization

## What We're Testing

- Is the growth spread visually clear and intuitive?
- Does the grid alignment feel natural or too rigid?
- Is the growth rate too fast/slow?
- How does performance feel with multiple growth areas?

## Known Limitations

- No camera controls (fixed top-down view)
- No way to remove growth once placed
- Growth spreads infinitely (no maximum radius yet)

## Technical Implementation

- **ECS Architecture**: Uses Bevy's Entity Component System
- **Grid-aligned positioning**: 2x2 unit grid for organized growth
- **Radial expansion**: Growth spreads outward from origins
- **Distance-based alpha**: Transparency fades with distance from origin
- **Timer-based updates**: Consistent 5Hz simulation rate
- **Modern Bevy patterns**: Uses Single<> queries and latest component
  syntax

## Running the Example

```bash
cargo clippy && cargo fmt && cargo run --example growth_overlay
```

Please share feedback on the growth visuals and any performance issues you
encounter. This mechanic will eventually tie into resource collection and
territory control!

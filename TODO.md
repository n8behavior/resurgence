# Resurgence TODO List

> **Note**: Experiment details can be found in [experiments.md](experiments.md)

## Bugs

- [x] growth only goes from light red to red. It should continue through browns
      and all the way to black
- [x] CPU usage continues to climb even after growth fully consumes the
      terrain. I suspect we need to check the terrain bounds.
- [ ] growth colors look a bit faded, especially black. It only go to charcoal.
      need to grow to full black for now.

## Features

- [ ] Improve growth system.
  - why do we reset growth state when a new origin is added?
  - MAX_GROWTH_RADIUS is the best way to detect growth complete. I think we
    could determine if there is any free space left each origin can expand into.
    We have to check this anyway as we spawn new growth. I think that system
    could flag each origin as fully grown.
  - Also growth has two components, expanding and maturing.
- [ ] Comprehensive tracing for all systems to allow insight and debugging
- [ ] Add experiment descriptions to launcher UI
- [ ] Implement [strike radius targeting UI](experiments.md#strike-radius-targeting-ui)
- [ ] Add README screenshots of experiments
- [ ] Implement settings persistence
- [ ] Implement [simple skirmish encounter](experiments.md#simple-skirmish-encounter)
- [ ] Implement [resource collection & conversion](experiments.md#resource-collection--conversion)
- [ ] Implement [panel-storyboard generator](experiments.md#panel-storyboard-generator)
- [ ] Implement [projectile arc & impact markers](experiments.md#projectile-arc--impact-markers)
- [ ] Implement [rugged terrain pathfinding](experiments.md#rugged-terrain-pathfinding)
- [ ] Implement [interactive material inspection](experiments.md#interactive-material-inspection)
- [ ] Implement [HUD & input mapping](experiments.md#hud--input-mapping)
- [ ] Implement save/load system for experiments
- [ ] Add performance profiling tools
- [ ] Add experiment replay/recording system
- [ ] Implement multiplayer experiment support

## Considering

- [ ] Add hot-reload support for faster development
- [ ] Add categories/tags to experiments for organization
- [ ] Implement experiment search/filter in launcher
- [ ] Create contributing guidelines
- [ ] Add keyboard navigation to launcher
- [ ] Create experiment template generator script
- [ ] Add launcher UI animations/transitions
- [ ] Create automated screenshot system for docs
- [ ] Add VR/AR experiment support
- [ ] Create video tutorials for each experiment
- [ ] Fix WASM build size (currently 61MB)

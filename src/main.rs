use bevy::prelude::*;

mod experiments;
mod launcher;

use experiments::{
    AppState, Experiment, growth_overlay::GrowthOverlayExperiment,
    terrain_proc_gen::TerrainProcGenExperiment,
};
use launcher::LauncherPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(LauncherPlugin);

    // Add experiment systems
    GrowthOverlayExperiment::add_systems(&mut app);
    TerrainProcGenExperiment::add_systems(&mut app);

    app.run();
}

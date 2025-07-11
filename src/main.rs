use bevy::prelude::*;

mod experiments;
mod launcher;

use experiments::{AppState, all_experiments};
use launcher::LauncherPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins(LauncherPlugin);

    // Add all experiment systems from registry
    for experiment in all_experiments() {
        experiment.add_systems(&mut app);
    }

    app.run();
}

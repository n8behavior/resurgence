pub mod growth_overlay;
pub mod terrain_proc_gen;

use bevy::prelude::*;

/// Marker trait for experiment modules
pub trait Experiment {
    /// Returns the name of the experiment
    fn name() -> &'static str;

    /// Adds the experiment's systems to the app
    fn add_systems(app: &mut App) -> &mut App;
}

/// States for the app - launcher and individual experiments
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Launcher,
    GrowthOverlay,
    TerrainProcGen,
}

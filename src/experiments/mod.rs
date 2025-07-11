pub mod growth_overlay;
pub mod terrain_proc_gen;

use bevy::prelude::*;

use self::{growth_overlay::GrowthOverlayExperiment, terrain_proc_gen::TerrainProcGenExperiment};

/// Trait for experiment modules
pub trait Experiment {
    /// Returns the name of the experiment
    fn name(&self) -> &'static str;

    /// Returns the icon/emoji for the experiment
    fn icon(&self) -> &'static str;

    /// Returns the app state for this experiment
    fn app_state(&self) -> AppState;

    /// Adds the experiment's systems to the app
    fn add_systems<'a>(&self, app: &'a mut App) -> &'a mut App;
}

/// States for the app - launcher and individual experiments
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Launcher,
    GrowthOverlay,
    TerrainProcGen,
}

/// Registry of all available experiments
pub fn all_experiments() -> Vec<Box<dyn Experiment>> {
    vec![
        Box::new(GrowthOverlayExperiment),
        Box::new(TerrainProcGenExperiment),
        // Add new experiments here
    ]
}

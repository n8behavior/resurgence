pub mod crimson_sprawl;
pub mod ecs_tilemap;
pub mod terrain_proc_gen;

use bevy::prelude::*;

use self::{
    crimson_sprawl::CrimsonSprawlExperiment, ecs_tilemap::EcsTilemapPoc,
    terrain_proc_gen::TerrainProcGenExperiment,
};

/// Trait for experiment modules
pub trait Experiment {
    /// Returns the name of the experiment
    fn name(&self) -> &'static str;

    /// Returns the icon/emoji for the experiment
    fn icon(&self) -> &'static str;

    /// Returns the app state for this experiment
    fn app_state(&self) -> AppState;

    /// Adds the experiment's systems to the app
    fn app_setup<'a>(&self, app: &'a mut App) -> &'a mut App;
}

/// States for the app - launcher and individual experiments
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Launcher,
    CrimsonSprawl,
    TerrainProcGen,
    EcsTilemap,
}

/// Registry of all available experiments
pub fn all_experiments() -> Vec<Box<dyn Experiment>> {
    vec![
        Box::new(CrimsonSprawlExperiment),
        Box::new(TerrainProcGenExperiment),
        Box::new(EcsTilemapPoc),
        // Add new experiments here
    ]
}

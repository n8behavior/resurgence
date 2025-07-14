use bevy::prelude::*;

use super::{AppState, Experiment};

pub struct EcsTilemapPoc;

impl Experiment for EcsTilemapPoc {
    fn name(&self) -> &'static str {
        "ECS Tilemap POC"
    }

    fn icon(&self) -> &'static str {
        "\u{e0c6}"
    }

    fn app_state(&self) -> super::AppState {
        AppState::EcsTilemap
    }

    fn app_setup<'a>(&self, app: &'a mut App) -> &'a mut App {
        app
    }
}

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

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
        app.add_plugins(TilemapPlugin)
            .add_systems(OnEnter(AppState::EcsTilemap), startup)
    }
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let map_size = TilemapSize { x: 32, y: 32 };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let texture_handle: Handle<Image> = asset_server.load("textures_16x16.png");
    let tile_size = TilemapTileSize { x: 16f32, y: 16f32 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Square;

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        anchor: TilemapAnchor::Center,
        ..default()
    });
}

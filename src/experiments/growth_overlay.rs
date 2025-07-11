use bevy::math::prelude::InfinitePlane3d;
use bevy::prelude::*;
use bevy::render::mesh::Mesh3d;
use bevy::time::{Timer, TimerMode};
use bevy::window::PrimaryWindow;

use super::{AppState, Experiment};

// Grid and positioning constants
const GRID_SIZE: f32 = 2f32;
const SPOT_SIZE: f32 = 2f32;
const POSITION_TOLERANCE: f32 = 1f32;
const TERRAIN_HEIGHT_OFFSET: f32 = 0.01f32;

// Terrain setup constants
const TERRAIN_SIZE: f32 = 200f32;
const CAMERA_HEIGHT: f32 = 50f32;
const GROUND_COLOR: (f32, f32, f32) = (0.3f32, 0.5f32, 0.3f32);
const DIRECTIONAL_LIGHT_POS: (f32, f32, f32) = (0f32, 10f32, 0f32);

// Update timing constants
const GROWTH_UPDATE_FREQUENCY: f32 = 0.2f32; // 5Hz

// Game balance constants
const DEFAULT_GROWTH_RATE: f32 = 0.05f32; // How fast spots mature (0-1 per second)
const DEFAULT_RADIUS_EXPANSION_RATE: f32 = 0.2f32; // How fast growth spreads (units per second)
const DEFAULT_MAX_GROWTH_DISTANCE: f32 = 20f32; // Distance for alpha fade effect
const DEFAULT_MIN_ALPHA: f32 = 0.2f32; // Minimum transparency for distant growth
const DEFAULT_INITIAL_GROWTH_AGE: f32 = 0f32; // Starting age for new growth spots
const DEFAULT_INITIAL_RADIUS: f32 = 0f32; // Starting radius for new growth origins
const DEFAULT_MAX_GROWTH_AGE: f32 = 1f32; // Maximum age (fully mature)
const MAX_GROWTH_RADIUS: f32 = 120f32; // Maximum radius to prevent infinite expansion

// Visual constants
const GROWTH_BASE_COLOR: (f32, f32, f32) = (1f32, 0f32, 0f32); // Red color for growth spots

// Visual aging constants (red to black interpolation)
const GROWTH_VISUAL_AGE_THRESHOLD: f32 = 1f32; // Age when visual updates stop

#[derive(Component)]
pub struct GrowthOrigin;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Growth {
    pub age: f32,         // 0.0 to 1.0 (fully mature)
    pub growth_rate: f32, // How fast it ages per second
}

#[derive(Resource)]
pub struct GrowthSpreadTimer(pub Timer);

#[derive(Resource)]
pub struct RadiusExpansionTimer(pub Timer);

#[derive(Resource)]
pub struct GameConfig {
    pub growth_rate: f32,           // How fast spots mature (0-1 per second)
    pub radius_expansion_rate: f32, // How fast growth spreads (units per second)
    pub max_growth_distance: f32,   // Distance for alpha fade effect
    pub min_alpha: f32,             // Minimum transparency for distant growth
    pub initial_growth_age: f32,    // Starting age for new growth spots
    pub max_growth_age: f32,        // Maximum age (fully mature)
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            growth_rate: DEFAULT_GROWTH_RATE,
            radius_expansion_rate: DEFAULT_RADIUS_EXPANSION_RATE,
            max_growth_distance: DEFAULT_MAX_GROWTH_DISTANCE,
            min_alpha: DEFAULT_MIN_ALPHA,
            initial_growth_age: DEFAULT_INITIAL_GROWTH_AGE,
            max_growth_age: DEFAULT_MAX_GROWTH_AGE,
        }
    }
}

#[derive(Resource)]
pub struct GrowthRadius {
    pub origins: Vec<(Vec3, f32)>, // (position, current_radius)
    pub expansion_complete: bool,   // True when no more expansion is possible
}

#[derive(Resource, Default)]
pub struct GrowthState {
    pub is_complete: bool, // True when all growth is fully mature and expansion is done
}

pub struct GrowthOverlayExperiment;

impl Experiment for GrowthOverlayExperiment {
    fn name(&self) -> &'static str {
        "Growth-Type Overlay Demo"
    }

    fn icon(&self) -> &'static str {
        "🌱"
    }

    fn app_state(&self) -> AppState {
        AppState::GrowthOverlay
    }

    fn add_systems<'a>(&self, app: &'a mut App) -> &'a mut App {
        app.insert_resource(GrowthSpreadTimer(Timer::from_seconds(
            GROWTH_UPDATE_FREQUENCY,
            TimerMode::Repeating,
        )))
        .insert_resource(RadiusExpansionTimer(Timer::from_seconds(
            GROWTH_UPDATE_FREQUENCY,
            TimerMode::Repeating,
        )))
        .insert_resource(GameConfig::default())
        .insert_resource(GrowthRadius {
            origins: Vec::new(),
            expansion_complete: false,
        })
        .insert_resource(GrowthState::default())
        .add_systems(OnEnter(AppState::GrowthOverlay), setup_growth_experiment)
        .add_systems(
            Update,
            (
                spawn_growth_origin.run_if(mouse_just_clicked),
                age_growth.run_if(growth_not_complete),
                update_growth_visuals.run_if(growth_not_complete),
                expand_growth_radius.run_if(growth_not_complete),
                spawn_growth_in_radius.run_if(growth_not_complete),
                check_growth_completion.run_if(growth_not_complete),
                exit_experiment_on_escape,
            )
                .run_if(in_state(AppState::GrowthOverlay)),
        )
        .add_systems(OnExit(AppState::GrowthOverlay), cleanup_growth_experiment)
    }
}

fn mouse_just_clicked(mouse: Res<ButtonInput<MouseButton>>) -> bool {
    mouse.just_pressed(MouseButton::Left)
}

fn growth_not_complete(growth_state: Res<GrowthState>) -> bool {
    !growth_state.is_complete
}

fn exit_experiment_on_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Launcher);
    }
}

fn snap_to_grid(position: Vec3) -> Vec3 {
    Vec3::new(
        (position.x / GRID_SIZE).round() * GRID_SIZE,
        position.y, // Keep Y unchanged for terrain height
        (position.z / GRID_SIZE).round() * GRID_SIZE,
    )
}

fn spawn_growth_at_position(
    position: Vec3,
    origin_pos: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    existing_growth: &Query<&Transform, With<Growth>>,
    config: &GameConfig,
) -> bool {
    // Check if there's already growth at this position (within tolerance)
    for existing_transform in existing_growth.iter() {
        let distance = existing_transform.translation.distance(position);
        if distance < POSITION_TOLERANCE {
            return false; // Don't spawn, position is occupied
        }
    }

    // Calculate alpha based on distance (closer = more opaque, farther = more transparent)
    let distance_from_origin = position.distance(origin_pos);
    let alpha = (1f32 - (distance_from_origin / config.max_growth_distance).min(1f32))
        .max(config.min_alpha);

    let mesh = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(SPOT_SIZE, SPOT_SIZE)
            .subdivisions(0),
    );
    let patch_handle = meshes.add(mesh);
    let patch_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(
            GROWTH_BASE_COLOR.0,
            GROWTH_BASE_COLOR.1,
            GROWTH_BASE_COLOR.2,
            alpha,
        ),
        alpha_mode: bevy::prelude::AlphaMode::Blend,
        ..default()
    });

    commands.spawn((
        Mesh3d(patch_handle),
        MeshMaterial3d(patch_mat),
        Transform::from_translation(position),
        GlobalTransform::default(),
        GrowthOrigin,
        Growth {
            age: config.initial_growth_age,
            growth_rate: config.growth_rate,
        },
    ));

    true // Successfully spawned
}

fn setup_growth_experiment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Simple flat terrain plane
    let terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(TERRAIN_SIZE, TERRAIN_SIZE)
            .subdivisions(0),
    );
    let ground_handle = meshes.add(terrain);
    let ground_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(GROUND_COLOR.0, GROUND_COLOR.1, GROUND_COLOR.2),
        ..default()
    });
    commands.spawn((
        Mesh3d(ground_handle),
        MeshMaterial3d(ground_mat),
        Ground,
        Transform::IDENTITY,
        GlobalTransform::default(),
    ));

    // Directional light
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::new(
            DIRECTIONAL_LIGHT_POS.0,
            DIRECTIONAL_LIGHT_POS.1,
            DIRECTIONAL_LIGHT_POS.2,
        ))
        .looking_at(Vec3::ZERO, Vec3::Z),
        GlobalTransform::default(),
    ));

    // Fixed top-down 2D camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0f32, CAMERA_HEIGHT, 0f32))
            .looking_at(Vec3::ZERO, Vec3::Z),
        GlobalTransform::default(),
    ));
}

#[allow(clippy::type_complexity)]
fn cleanup_growth_experiment(
    mut commands: Commands,
    entities: Query<
        Entity,
        Or<(
            With<Growth>,
            With<Ground>,
            With<Camera3d>,
            With<DirectionalLight>,
        )>,
    >,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_growth_origin(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    ground_tf: Single<&GlobalTransform, With<Ground>>,
    existing_growth: Query<&Transform, With<Growth>>,
    config: Res<GameConfig>,
    mut growth_radius: ResMut<GrowthRadius>,
    mut growth_state: ResMut<GrowthState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let (cam, cam_tf) = camera.into_inner();
    let Ok(ray) = cam.viewport_to_world(cam_tf, cursor_pos) else {
        return;
    };

    if let Some(distance) =
        ray.intersect_plane(ground_tf.translation(), InfinitePlane3d::new(Vec3::Y))
    {
        let world_point = ray.origin + ray.direction * distance;
        let grid_aligned_point = snap_to_grid(world_point);
        let final_position = grid_aligned_point + Vec3::Y * TERRAIN_HEIGHT_OFFSET;

        // Register new growth origin
        growth_radius
            .origins
            .push((final_position, DEFAULT_INITIAL_RADIUS));
        
        // Reset expansion complete flag when new origin is added
        growth_radius.expansion_complete = false;
        // Reset global growth state when new origin is added
        growth_state.is_complete = false;

        spawn_growth_at_position(
            final_position,
            final_position, // This is the origin for user-clicked spots
            &mut commands,
            &mut meshes,
            &mut materials,
            &existing_growth,
            &config,
        );
    }
}

fn age_growth(time: Res<Time>, config: Res<GameConfig>, mut growth_q: Query<&mut Growth>) {
    // Only process growth entities that aren't fully mature
    for mut growth in growth_q.iter_mut() {
        if growth.age < config.max_growth_age {
            growth.age += growth.growth_rate * time.delta_secs();
            // Clamp to max age to prevent overshooting
            if growth.age > config.max_growth_age {
                growth.age = config.max_growth_age;
            }
        }
    }
}

fn update_growth_visuals(
    mut growth_q: Query<(&Growth, &mut MeshMaterial3d<StandardMaterial>), With<Growth>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (growth, mesh_mat) in growth_q.iter_mut() {
        if let Some(material) = materials.get_mut(&mesh_mat.0) {
            // Interpolate from red (new) to black (mature)
            // Red -> Brown -> Dark Brown -> Black
            let age_normalized = (growth.age / GROWTH_VISUAL_AGE_THRESHOLD).clamp(0.0, 1.0);
            
            let (r, g, b) = if age_normalized < 0.5 {
                // First half: Red (1,0,0) -> Brown (0.6,0.3,0.1)
                let t = age_normalized * 2.0; // 0.0 to 1.0
                let r = 1.0 - t * 0.4; // 1.0 -> 0.6
                let g = t * 0.3;       // 0.0 -> 0.3
                let b = t * 0.1;       // 0.0 -> 0.1
                (r, g, b)
            } else {
                // Second half: Brown (0.6,0.3,0.1) -> Black (0,0,0)
                let t = (age_normalized - 0.5) * 2.0; // 0.0 to 1.0
                let r = 0.6 - t * 0.6; // 0.6 -> 0.0
                let g = 0.3 - t * 0.3; // 0.3 -> 0.0
                let b = 0.1 - t * 0.1; // 0.1 -> 0.0
                (r, g, b)
            };
            
            material.base_color = Color::srgb(r, g, b);
        }
    }
}

fn expand_growth_radius(
    time: Res<Time>,
    config: Res<GameConfig>,
    mut timer: ResMut<RadiusExpansionTimer>,
    mut growth_radius: ResMut<GrowthRadius>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    // Calculate expansion for this tick
    let expansion_amount = config.radius_expansion_rate * GROWTH_UPDATE_FREQUENCY;
    
    for (_pos, radius) in growth_radius.origins.iter_mut() {
        if *radius < MAX_GROWTH_RADIUS {
            *radius += expansion_amount;
            // Cap at maximum radius
            if *radius > MAX_GROWTH_RADIUS {
                *radius = MAX_GROWTH_RADIUS;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_growth_in_radius(
    time: Res<Time>,
    config: Res<GameConfig>,
    mut timer: ResMut<GrowthSpreadTimer>,
    growth_radius: Res<GrowthRadius>,
    existing_growth: Query<&Transform, With<Growth>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    let mut spawn_data = Vec::new();

    // For each origin, find grid positions within its current radius
    for &(origin_pos, radius) in &growth_radius.origins {
        let max_grid_distance = (radius / GRID_SIZE).floor() as i32;

        // Check all grid positions within radius
        for x in -max_grid_distance..=max_grid_distance {
            for z in -max_grid_distance..=max_grid_distance {
                let grid_pos = Vec3::new(
                    origin_pos.x + (x as f32) * GRID_SIZE,
                    origin_pos.y,
                    origin_pos.z + (z as f32) * GRID_SIZE,
                );

                // Check if position is within terrain bounds
                let terrain_half_size = TERRAIN_SIZE / 2.0;
                if grid_pos.x.abs() > terrain_half_size || grid_pos.z.abs() > terrain_half_size {
                    continue; // Skip positions outside terrain
                }

                let distance_from_origin = grid_pos.distance(origin_pos);

                // Only spawn if within radius and not too close to origin
                if distance_from_origin <= radius && distance_from_origin >= GRID_SIZE {
                    // Check if position is already occupied
                    let mut occupied = false;
                    for existing_transform in existing_growth.iter() {
                        if existing_transform.translation.distance(grid_pos) < POSITION_TOLERANCE {
                            occupied = true;
                            break;
                        }
                    }

                    if !occupied {
                        spawn_data.push((grid_pos, origin_pos));
                    }
                }
            }
        }
    }

    // Spawn new growth at calculated positions
    for (pos, origin_pos) in spawn_data {
        spawn_growth_at_position(
            pos,
            origin_pos,
            &mut commands,
            &mut meshes,
            &mut materials,
            &existing_growth,
            &config,
        );
    }
}

fn check_growth_completion(
    growth_radius: Res<GrowthRadius>,
    growth_q: Query<&Growth>,
    config: Res<GameConfig>,
    mut growth_state: ResMut<GrowthState>,
) {
    // Check if all origins are at max radius
    let expansion_complete = growth_radius.origins.iter()
        .all(|(_, radius)| *radius >= MAX_GROWTH_RADIUS);
    
    // Check if all growth entities are fully mature
    let all_growth_mature = growth_q.iter()
        .all(|growth| growth.age >= config.max_growth_age);
    
    // Growth is complete when both expansion and aging are done
    if expansion_complete && all_growth_mature {
        growth_state.is_complete = true;
    }
}

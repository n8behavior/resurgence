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
const DEFAULT_GROWTH_RATE: f32 = 0.5f32; // How fast spots mature (0-1 per second)
const DEFAULT_RADIUS_EXPANSION_RATE: f32 = 1f32; // How fast growth spreads (units per second)
const DEFAULT_INITIAL_GROWTH_AGE: f32 = 0f32; // Starting age for new growth spots
const DEFAULT_INITIAL_RADIUS: f32 = 0f32; // Starting radius for new growth origins
const DEFAULT_MAX_GROWTH_AGE: f32 = 1f32; // Maximum age (fully mature)
const MAX_GROWTH_RADIUS: f32 = 120f32; // Maximum radius to prevent infinite expansion

// Visual constants
const GROWTH_BASE_COLOR: (f32, f32, f32) = (1f32, 0f32, 0f32); // Red color for growth spots

// Visual aging constants (red to black interpolation)
const GROWTH_VISUAL_AGE_THRESHOLD: f32 = 1f32; // Age when visual updates stop

// Component for Crimson Sprawl colonies
#[derive(Component)]
pub struct CrimsonColony {
    pub radius: f32,
    pub expansion_complete: bool,
    pub expansion_rate: f32,
    pub maturation_rate: f32,
    pub max_radius: f32,
}

impl Default for CrimsonColony {
    fn default() -> Self {
        Self {
            radius: DEFAULT_INITIAL_RADIUS,
            expansion_complete: false,
            expansion_rate: DEFAULT_RADIUS_EXPANSION_RATE,
            maturation_rate: DEFAULT_GROWTH_RATE,
            max_radius: MAX_GROWTH_RADIUS,
        }
    }
}

#[derive(Component)]
pub struct GrowthPatch {
    pub _colony_entity: Entity,
    pub age: f32,             // 0.0 to 1.0 (fully mature)
    pub maturation_rate: f32, // How fast it ages per second
}

#[derive(Component)]
pub struct Ground;

#[derive(Resource)]
pub struct GrowthUpdateTimer(pub Timer);

// Patch spacing is consistent across all colonies
const PATCH_SPACING: f32 = GRID_SIZE;

#[derive(Resource, Default)]
pub struct GrowthState {
    pub is_complete: bool, // True when all growth is fully mature and expansion is done
}

pub struct CrimsonSprawlExperiment;

impl Experiment for CrimsonSprawlExperiment {
    fn name(&self) -> &'static str {
        "Crimson Sprawl (original attempt)"
    }

    fn icon(&self) -> &'static str {
        "\u{e22f}"
    }

    fn app_state(&self) -> AppState {
        AppState::CrimsonSprawl
    }

    fn app_setup<'a>(&self, app: &'a mut App) -> &'a mut App {
        app.insert_resource(GrowthUpdateTimer(Timer::from_seconds(
            GROWTH_UPDATE_FREQUENCY,
            TimerMode::Repeating,
        )))
        .insert_resource(GrowthState::default())
        .add_systems(OnEnter(AppState::CrimsonSprawl), setup_crimson_experiment)
        .add_systems(
            Update,
            (
                // Timer system runs every frame to track time (runs first)
                tick_growth_timer.run_if(growth_not_complete),
                // Systems that need 60fps responsiveness
                spawn_crimson_colony.run_if(mouse_just_clicked),
            )
                .run_if(in_state(AppState::CrimsonSprawl)),
        )
        .add_systems(
            Update,
            // Systems that only need 5Hz updates (12x performance improvement)
            // These run after timer ticking to ensure proper condition evaluation
            (
                patch_maturation_system,
                update_patch_visuals,
                crimson_expansion_system,
                crimson_spreading_system,
                check_crimson_completion,
            )
                .run_if(
                    in_state(AppState::CrimsonSprawl)
                        .and(growth_not_complete)
                        .and(growth_timer_just_finished),
                ),
        )
        .add_systems(OnExit(AppState::CrimsonSprawl), cleanup_crimson_experiment)
    }
}

fn mouse_just_clicked(mouse: Res<ButtonInput<MouseButton>>) -> bool {
    mouse.just_pressed(MouseButton::Left)
}

fn growth_not_complete(growth_state: Res<GrowthState>) -> bool {
    !growth_state.is_complete
}

fn growth_timer_just_finished(timer: Res<GrowthUpdateTimer>) -> bool {
    timer.0.just_finished()
}

fn tick_growth_timer(mut timer: ResMut<GrowthUpdateTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

fn snap_to_grid(position: Vec3) -> Vec3 {
    Vec3::new(
        (position.x / GRID_SIZE).round() * GRID_SIZE,
        position.y, // Keep Y unchanged for terrain height
        (position.z / GRID_SIZE).round() * GRID_SIZE,
    )
}

#[allow(clippy::too_many_arguments)]
fn spawn_crimson_patch(
    position: Vec3,
    colony_entity: Entity,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    existing_patches: &Query<&Transform, With<GrowthPatch>>,
    colony: &CrimsonColony,
) -> bool {
    // Check if there's already a patch at this position (within tolerance)
    for existing_transform in existing_patches.iter() {
        let distance = existing_transform.translation.distance(position);
        if distance < POSITION_TOLERANCE {
            return false; // Don't spawn, position is occupied
        }
    }

    let mesh = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(SPOT_SIZE, SPOT_SIZE)
            .subdivisions(0),
    );
    let patch_handle = meshes.add(mesh);

    let patch_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(
            GROWTH_BASE_COLOR.0,
            GROWTH_BASE_COLOR.1,
            GROWTH_BASE_COLOR.2,
        ),
        ..default()
    });

    commands.spawn((
        Mesh3d(patch_handle),
        MeshMaterial3d(patch_mat),
        Transform::from_translation(position),
        GlobalTransform::default(),
        GrowthPatch {
            _colony_entity: colony_entity,
            age: DEFAULT_INITIAL_GROWTH_AGE,
            maturation_rate: colony.maturation_rate,
        },
    ));

    true // Successfully spawned
}

fn setup_crimson_experiment(
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
fn cleanup_crimson_experiment(
    mut commands: Commands,
    entities: Query<
        Entity,
        Or<(
            With<GrowthPatch>,
            With<CrimsonColony>,
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
fn spawn_crimson_colony(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    ground_tf: Single<&GlobalTransform, With<Ground>>,
    existing_patches: Query<&Transform, With<GrowthPatch>>,
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

        // Create colony origin entity
        let colony_entity = commands
            .spawn((
                CrimsonColony::default(),
                Transform::from_translation(final_position),
                GlobalTransform::default(),
            ))
            .id();

        // Get the colony component we just created
        let colony = CrimsonColony::default();

        // Reset global growth state
        growth_state.is_complete = false;

        // Spawn the first patch at the origin position
        spawn_crimson_patch(
            final_position,
            colony_entity,
            &mut commands,
            &mut meshes,
            &mut materials,
            &existing_patches,
            &colony,
        );
    }
}

fn patch_maturation_system(time: Res<Time>, mut patch_q: Query<&mut GrowthPatch>) {
    // Only process patches that aren't fully mature
    for mut patch in patch_q.iter_mut() {
        if patch.age < DEFAULT_MAX_GROWTH_AGE {
            patch.age += patch.maturation_rate * time.delta_secs();
            // Clamp to max age to prevent overshooting
            if patch.age > DEFAULT_MAX_GROWTH_AGE {
                patch.age = DEFAULT_MAX_GROWTH_AGE;
            }
        }
    }
}

fn update_patch_visuals(
    mut patch_q: Query<(&GrowthPatch, &mut MeshMaterial3d<StandardMaterial>), With<GrowthPatch>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (patch, mesh_mat) in patch_q.iter_mut() {
        if let Some(material) = materials.get_mut(&mesh_mat.0) {
            // Interpolate from red (new) to black (mature)
            // Red -> Brown -> Dark Brown -> Black
            let age_normalized = (patch.age / GROWTH_VISUAL_AGE_THRESHOLD).clamp(0.0, 1.0);

            let (r, g, b) = if age_normalized < 0.5 {
                // First half: Red (1,0,0) -> Brown (0.6,0.3,0.1)
                let t = age_normalized * 2.0; // 0.0 to 1.0
                let r = 1.0 - t * 0.4; // 1.0 -> 0.6
                let g = t * 0.3; // 0.0 -> 0.3
                let b = t * 0.1; // 0.0 -> 0.1
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

fn crimson_expansion_system(mut colony_q: Query<&mut CrimsonColony>) {
    for mut colony in colony_q.iter_mut() {
        // Only expand colonies that aren't complete
        if !colony.expansion_complete && colony.radius < colony.max_radius {
            let expansion_amount = colony.expansion_rate * GROWTH_UPDATE_FREQUENCY;
            colony.radius += expansion_amount;
            // Cap at maximum radius and mark as complete if reached
            if colony.radius >= colony.max_radius {
                colony.radius = colony.max_radius;
                colony.expansion_complete = true;
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn crimson_spreading_system(
    colony_q: Query<(Entity, &CrimsonColony, &Transform)>,
    existing_patches: Query<&Transform, With<GrowthPatch>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut spawn_data = Vec::new();

    // For each colony, find grid positions within its current radius
    for (colony_entity, colony, transform) in colony_q.iter() {
        if colony.expansion_complete {
            continue; // Skip colonies that are already complete
        }

        let origin_pos = transform.translation;
        let radius = colony.radius;
        let max_grid_distance = (radius / PATCH_SPACING).floor() as i32;

        // Check all grid positions within radius
        for x in -max_grid_distance..=max_grid_distance {
            for z in -max_grid_distance..=max_grid_distance {
                let grid_pos = Vec3::new(
                    origin_pos.x + (x as f32) * PATCH_SPACING,
                    origin_pos.y,
                    origin_pos.z + (z as f32) * PATCH_SPACING,
                );

                // Check if position is within terrain bounds
                let terrain_half_size = TERRAIN_SIZE / 2.0;
                if grid_pos.x.abs() > terrain_half_size || grid_pos.z.abs() > terrain_half_size {
                    continue; // Skip positions outside terrain
                }

                let distance_from_origin = grid_pos.distance(origin_pos);

                // Only spawn if within radius and not too close to origin
                if distance_from_origin <= radius && distance_from_origin >= PATCH_SPACING {
                    // Check if position is already occupied
                    let mut occupied = false;
                    for existing_transform in existing_patches.iter() {
                        if existing_transform.translation.distance(grid_pos) < POSITION_TOLERANCE {
                            occupied = true;
                            break;
                        }
                    }

                    if !occupied {
                        spawn_data.push((grid_pos, colony_entity, colony));
                    }
                }
            }
        }
    }

    // Spawn new patches at calculated positions
    for (pos, colony_entity, colony) in spawn_data {
        spawn_crimson_patch(
            pos,
            colony_entity,
            &mut commands,
            &mut meshes,
            &mut materials,
            &existing_patches,
            colony,
        );
    }
}

fn check_crimson_completion(
    colony_q: Query<&CrimsonColony>,
    patch_q: Query<&GrowthPatch>,
    mut growth_state: ResMut<GrowthState>,
) {
    // Check if all colonies have completed expansion
    let all_colonies_complete = colony_q.iter().all(|colony| colony.expansion_complete);

    // Check if all patches are fully mature
    let all_patches_mature = patch_q
        .iter()
        .all(|patch| patch.age >= DEFAULT_MAX_GROWTH_AGE);

    // Growth is complete when both expansion and maturation are done
    if all_colonies_complete && all_patches_mature {
        growth_state.is_complete = true;
    }
}

use bevy::math::prelude::InfinitePlane3d;
use bevy::prelude::*;
use bevy::render::mesh::Mesh3d;
use bevy::time::{Timer, TimerMode};
use bevy::window::PrimaryWindow;

#[derive(Component)]
struct GrowthOrigin;

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Growth {
    age: f32,                    // 0.0 to 1.0 (fully mature)
    growth_rate: f32,            // How fast it ages per second
    can_spread: bool,            // Has it spread yet?
    distance_from_origin: f32,   // Distance from nearest origin point
    origin_position: Vec3,       // Position of the origin that spawned this growth
}

#[derive(Resource)]
struct GrowthSpreadTimer(Timer);

#[derive(Resource)]
struct GrowthRadius {
    origins: Vec<(Vec3, f32)>, // (position, current_radius)
    expansion_rate: f32,       // How fast radius grows per second
}

fn mouse_just_clicked(mouse: Res<ButtonInput<MouseButton>>) -> bool {
    mouse.just_pressed(MouseButton::Left)
}

fn snap_to_grid(position: Vec3, grid_size: f32) -> Vec3 {
    Vec3::new(
        (position.x / grid_size).round() * grid_size,
        position.y, // Keep Y unchanged for terrain height
        (position.z / grid_size).round() * grid_size,
    )
}

fn spawn_growth_at_position(
    position: Vec3,
    origin_pos: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    existing_growth: &Query<&Transform, With<Growth>>,
) -> bool {
    // Check if there's already growth at this position (within tolerance)
    const POSITION_TOLERANCE: f32 = 1f32;

    for existing_transform in existing_growth.iter() {
        let distance = existing_transform.translation.distance(position);
        if distance < POSITION_TOLERANCE {
            return false; // Don't spawn, position is occupied
        }
    }

    let distance_from_origin = position.distance(origin_pos);
    
    // Calculate alpha based on distance (closer = more opaque, farther = more transparent)
    let max_distance = 20f32; // Max visible growth distance
    let alpha = (1f32 - (distance_from_origin / max_distance).min(1f32)).max(0.2f32);
    
    let mesh = Mesh::from(Plane3d::default().mesh().size(2f32, 2f32).subdivisions(0));
    let patch_handle = meshes.add(mesh);
    let patch_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(1f32, 0f32, 0f32, alpha),
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
            age: 0f32,
            growth_rate: 0.2f32,
            can_spread: true,
            distance_from_origin,
            origin_position: origin_pos,
        },
    ));

    true // Successfully spawned
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GrowthSpreadTimer(Timer::from_seconds(
            0.5f32,
            TimerMode::Repeating,
        )))
        .insert_resource(GrowthRadius {
            origins: Vec::new(),
            expansion_rate: 2f32, // Radius grows 2 units per second
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_growth_origin.run_if(mouse_just_clicked),
                age_growth,
                update_growth_visuals,
                expand_growth_radius,
                spread_growth_radial,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Simple flat terrain plane
    let terrain_size = 200f32;
    let terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(terrain_size, terrain_size)
            .subdivisions(0),
    );
    let ground_handle = meshes.add(terrain);
    let ground_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3f32, 0.5f32, 0.3f32),
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
        Transform::from_translation(Vec3::new(0f32, 10f32, 0f32)).looking_at(Vec3::ZERO, Vec3::Z),
        GlobalTransform::default(),
    ));

    // Fixed top-down 2D camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0f32, 50f32, 0f32)).looking_at(Vec3::ZERO, Vec3::Z),
        GlobalTransform::default(),
    ));
}

fn spawn_growth_origin(
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    ground_tf: Single<&GlobalTransform, With<Ground>>,
    existing_growth: Query<&Transform, With<Growth>>,
    mut growth_radius: ResMut<GrowthRadius>,
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
        let grid_aligned_point = snap_to_grid(world_point, 2f32);
        let final_position = grid_aligned_point + Vec3::Y * 0.01f32;
        
        // Register new growth origin
        growth_radius.origins.push((final_position, 0f32));
        
        spawn_growth_at_position(
            final_position,
            final_position, // This is the origin for user-clicked spots
            &mut commands,
            &mut meshes,
            &mut materials,
            &existing_growth,
        );
    }
}

fn age_growth(time: Res<Time>, mut growth_q: Query<&mut Growth>) {
    for mut growth in growth_q.iter_mut() {
        if growth.age < 1f32 {
            growth.age = (growth.age + growth.growth_rate * time.delta_secs()).min(1f32);
        }
    }
}

fn update_growth_visuals(
    mut growth_q: Query<(&Growth, &mut MeshMaterial3d<StandardMaterial>), Changed<Growth>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (growth, mesh_mat) in growth_q.iter_mut() {
        if let Some(material) = materials.get_mut(&mesh_mat.0) {
            // Interpolate from red (new) to black (mature)
            // Red -> Dark Red -> Brown -> Black
            let r = 1f32 - growth.age * 0.5f32; // 1.0 -> 0.5
            let g = 0f32;
            let b = 0f32;
            material.base_color = Color::srgb(r, g, b);
        }
    }
}

fn expand_growth_radius(time: Res<Time>, mut growth_radius: ResMut<GrowthRadius>) {
    let expansion_amount = growth_radius.expansion_rate * time.delta_secs();
    for (_pos, radius) in growth_radius.origins.iter_mut() {
        *radius += expansion_amount;
    }
}

fn spread_growth_radial(
    time: Res<Time>,
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

    let grid_size = 2f32;
    let mut spawn_data = Vec::new();

    // For each origin, find grid positions within its current radius
    for &(origin_pos, radius) in &growth_radius.origins {
        let max_grid_distance = (radius / grid_size).floor() as i32;
        
        // Check all grid positions within radius
        for x in -max_grid_distance..=max_grid_distance {
            for z in -max_grid_distance..=max_grid_distance {
                let grid_pos = Vec3::new(
                    origin_pos.x + (x as f32) * grid_size,
                    origin_pos.y,
                    origin_pos.z + (z as f32) * grid_size,
                );
                
                let distance_from_origin = grid_pos.distance(origin_pos);
                
                // Only spawn if within radius and not too close to origin
                if distance_from_origin <= radius && distance_from_origin >= grid_size {
                    // Check if position is already occupied
                    let mut occupied = false;
                    for existing_transform in existing_growth.iter() {
                        if existing_transform.translation.distance(grid_pos) < 1f32 {
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
        );
    }
}


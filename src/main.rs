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
    age: f32,         // 0.0 to 1.0 (fully mature)
    growth_rate: f32, // How fast it ages per second
    can_spread: bool, // Has it spread yet?
}

#[derive(Resource)]
struct GrowthSpreadTimer(Timer);

fn mouse_just_clicked(mouse: Res<ButtonInput<MouseButton>>) -> bool {
    mouse.just_pressed(MouseButton::Left)
}

fn spawn_growth_at_position(
    position: Vec3,
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

    let mesh = Mesh::from(Plane3d::default().mesh().size(2f32, 2f32).subdivisions(0));
    let patch_handle = meshes.add(mesh);
    let patch_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(1f32, 0f32, 0f32),
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
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                spawn_growth_origin.run_if(mouse_just_clicked),
                age_growth,
                update_growth_visuals,
                spread_growth,
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
        spawn_growth_at_position(
            world_point + Vec3::Y * 0.01f32,
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

fn spread_growth(
    time: Res<Time>,
    mut timer: ResMut<GrowthSpreadTimer>,
    mut growth_q: Query<(&Transform, &mut Growth)>,
    existing_growth: Query<&Transform, With<Growth>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    let spread_distance = 2f32; // Distance between growth spots (matches spot size)
    let directions = [
        Vec3::new(spread_distance, 0f32, 0f32),  // East
        Vec3::new(-spread_distance, 0f32, 0f32), // West
        Vec3::new(0f32, 0f32, spread_distance),  // North
        Vec3::new(0f32, 0f32, -spread_distance), // South
    ];

    let mut spawn_positions = Vec::new();

    // Find mature growth that can spread
    for (transform, mut growth) in growth_q.iter_mut() {
        if growth.age >= 1f32 && growth.can_spread {
            growth.can_spread = false; // Mark as having spread

            // Calculate spawn positions
            for &dir in &directions {
                let new_pos = transform.translation + dir;
                spawn_positions.push(new_pos);
            }
        }
    }

    // Spawn new growth at calculated positions
    for pos in spawn_positions {
        spawn_growth_at_position(
            pos,
            &mut commands,
            &mut meshes,
            &mut materials,
            &existing_growth,
        );
    }
}

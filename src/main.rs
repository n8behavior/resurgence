use bevy::math::prelude::InfinitePlane3d;
use bevy::prelude::*;
use bevy::render::mesh::{Mesh3d, PlaneMeshBuilder};
use bevy::window::PrimaryWindow;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

#[derive(Component)]
struct GrowthOrigin;

#[derive(Component)]
struct Ground;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_growth_origin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane 20×20
    let mesh_size = 1000f32;
    let subdivisions = 200u32;

    let terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(mesh_size, mesh_size)
            .subdivisions(subdivisions),
    );
    let ground_handle = meshes.add(terrain);
    let ground_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.3, 0.5, 0.3),
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
        Transform::from_translation(Vec3::new(5.0, 8.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // 3D camera + orbit controls
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 5.0, 10.0)).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        PanOrbitCamera::default(),
    ));
}

fn spawn_growth_origin(
    mouse_input: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<PanOrbitCamera>>,
    ground_q: Query<&GlobalTransform, With<Ground>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let (camera, cam_tf) = if let Ok(pair) = camera_q.single() {
        pair
    } else {
        return;
    };
    let Ok(ray) = camera.viewport_to_world(cam_tf, cursor_pos) else {
        return;
    };

    let ground_tf = if let Ok(gt) = ground_q.single() {
        gt
    } else {
        return;
    };

    if let Some(distance) =
        ray.intersect_plane(ground_tf.translation(), InfinitePlane3d::new(Vec3::Y))
    {
        let world_point = ray.origin + ray.direction * distance;

        // Spawn a 1×1 red plane patch, slightly above the ground
        let mesh = Mesh::from(
            Plane3d::default()
                .mesh()
                .size(1f32, 1f32)
                .subdivisions(0u32),
        );
        let patch_handle = meshes.add(mesh);
        let patch_mat = materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            ..default()
        });
        commands.spawn((
            Mesh3d(patch_handle),
            MeshMaterial3d(patch_mat),
            Transform::from_translation(world_point + Vec3::Y * 0.01),
            GlobalTransform::default(),
            GrowthOrigin,
        ));
    }
}

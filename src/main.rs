use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        RenderPlugin,
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
    },
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

fn main() {
    App::new()
        // 1) Replace DefaultPlugins’ RenderPlugin to enable POLYGON_MODE_LINE
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
            // 2) Register the wireframe pipeline
            WireframePlugin::default(),
            PanOrbitCameraPlugin,
        ))
        // 3) Configure global vs. per-entity wireframe
        .insert_resource(WireframeConfig {
            global: false, // only draw wireframes where you add `Wireframe`
            default_color: Color::WHITE.into(),
        })
        // 4) Your startup systems
        .add_systems(Startup, (setup_camera, setup_terrain, setup_light))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 20.0, 75.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        GlobalTransform::default(),
        PanOrbitCamera::default(),
    ));
}

fn setup_light(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 50_000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::YXZ,
            std::f32::consts::FRAC_PI_4,
            -std::f32::consts::FRAC_PI_4,
            0.0,
        )),
        GlobalTransform::default(),
        Visibility::Visible,
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10));
    let material = materials.add(StandardMaterial {
        base_color: bevy::color::palettes::css::SILVER.into(),
        ..default()
    });

    commands.spawn((
        // use the new PBR-equivalent components
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        // 5) turn on wireframe for this mesh
        Wireframe,
    ));
}

use bevy::{
    color::palettes::tailwind::{AMBER_800, GREEN_400},
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        RenderPlugin,
        mesh::VertexAttributeValues,
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
    },
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use noise::{BasicMulti, NoiseFn, Perlin};

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
        .add_systems(Update, toggle_wireframe)
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
    let terrain_height = 70f32;
    let noise = BasicMulti::<Perlin>::default();

    let mut terrain = Mesh::from(
        Plane3d::default()
            .mesh()
            .size(1000f32, 1000f32)
            .subdivisions(200),
    );
    if let Some(VertexAttributeValues::Float32x3(positions)) =
        terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    {
        for pos in positions.iter_mut() {
            let val = noise.get([(pos[0] / 300f32) as f64, (pos[2] / 300f32) as f64]);
            pos[1] = val as f32 * terrain_height; // safe: Perlin is -1 to 1
        }
        let colors: Vec<[f32; 4]> = positions
            .iter()
            .map(|[_, g, _]| {
                let g = *g / terrain_height * 2f32;
                if g > 0.8f32 {
                    (Color::LinearRgba(LinearRgba {
                        red: 20f32,   // bloom above 1
                        green: 20f32, // bloom above 1
                        blue: 20f32,  // bloom above 1
                        alpha: 1f32,
                    }))
                    .to_linear()
                    .to_f32_array()
                } else if g > 0.3f32 {
                    Color::from(AMBER_800).to_linear().to_f32_array()
                } else if g < -0.8f32 {
                    Color::BLACK.to_linear().to_f32_array()
                } else {
                    Color::from(GREEN_400).to_linear().to_f32_array()
                }
            })
            .collect();
        terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    }
    terrain.compute_normals();
    let mesh = meshes.add(terrain);
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
        Terrain,
    ));
}
#[derive(Component)]
struct Terrain;

fn toggle_wireframe(
    mut commands: Commands,
    landscapes_wireframe: Query<Entity, (With<Terrain>, With<Wireframe>)>,
    landscapes: Query<Entity, (With<Terrain>, Without<Wireframe>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for terrain in &landscapes {
            commands.entity(terrain).insert(Wireframe);
        }
        for terrain in &landscapes_wireframe {
            commands.entity(terrain).remove::<Wireframe>();
        }
    }
}

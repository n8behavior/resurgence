use bevy::{
    color::palettes::{
        css::BLUE,
        tailwind::{AMBER_800, GREEN_400},
    },
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    platform::collections::HashMap,
    prelude::*,
    render::mesh::VertexAttributeValues,
};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use noise::{BasicMulti, NoiseFn, Perlin};

use super::{AppState, Experiment};

// Terrain constants
const TERRAIN_HEIGHT: f32 = 70f32;
const MESH_SIZE: f32 = 1000f32;
const SUBDIVISIONS: u32 = 200u32;
const NOISE_SEED: u32 = 900u32;
const NOISE_SCALE: f64 = 300.0;

#[derive(Component)]
pub struct TerrainMesh;

#[derive(Component)]
pub struct Ship;

#[derive(Component)]
pub struct ShipCamera;

#[derive(Resource)]
pub struct TerrainStore(pub HashMap<IVec2, Handle<Mesh>>);

pub struct TerrainProcGenExperiment;

impl Experiment for TerrainProcGenExperiment {
    fn name(&self) -> &'static str {
        "Procedural Terrain Generation"
    }

    fn icon(&self) -> &'static str {
        "\u{e2a6}" // Font Awesome mountain icon
    }

    fn app_state(&self) -> AppState {
        AppState::TerrainProcGen
    }

    fn add_systems<'a>(&self, app: &'a mut App) -> &'a mut App {
        app.add_plugins(WireframePlugin::default())
            .insert_resource(WireframeConfig {
                global: false, // only draw wireframes where you add `Wireframe`
                default_color: Color::WHITE,
            })
            .insert_resource(TerrainStore(HashMap::default()))
            .add_plugins(PanOrbitCameraPlugin)
            .add_systems(OnEnter(AppState::TerrainProcGen), setup_terrain_experiment)
            .add_systems(
                Update,
                (
                    toggle_wireframe,
                    control_ship,
                    control_ship_camera,
                    exit_experiment_on_escape,
                )
                    .run_if(in_state(AppState::TerrainProcGen)),
            )
            .add_systems(OnExit(AppState::TerrainProcGen), cleanup_terrain_experiment)
    }
}

fn exit_experiment_on_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(AppState::Launcher);
    }
}

fn setup_terrain_experiment(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Setting up terrain generation experiment...");

    // Setup light, ship, camera, and terrain
    setup_light(&mut commands);
    setup_ship(&mut commands, meshes, materials);
    setup_camera(&mut commands);
    setup_terrain(&mut commands);

    info!("Terrain experiment loaded! Controls:");
    info!("  WASD - Move ship");
    info!("  Space - Toggle wireframe");
    info!("  Mouse - Orbit camera");
    info!("  ESC - Return to launcher");
}

fn setup_ship(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_mesh = meshes.add(Sphere::new(2f32).mesh());
    let blue_material = materials.add(StandardMaterial {
        base_color: BLUE.into(),
        ..default()
    });

    commands.spawn((
        Mesh3d(sphere_mesh),
        MeshMaterial3d(blue_material),
        Transform::from_xyz(0f32, 20f32, 0f32),
        Ship,
    ));
}

fn setup_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 20.0, 75.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        GlobalTransform::default(),
        PanOrbitCamera::default(),
        ShipCamera,
    ));
}

fn setup_light(commands: &mut Commands) {
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

fn setup_terrain(commands: &mut Commands) {
    // Spawn a 3x3 grid of terrain chunks
    commands.queue(SpawnTerrain(IVec2::new(-1, -1)));
    commands.queue(SpawnTerrain(IVec2::new(-1, 0)));
    commands.queue(SpawnTerrain(IVec2::new(-1, 1)));
    commands.queue(SpawnTerrain(IVec2::new(0, -1)));
    commands.queue(SpawnTerrain(IVec2::new(0, 0)));
    commands.queue(SpawnTerrain(IVec2::new(0, 1)));
    commands.queue(SpawnTerrain(IVec2::new(1, -1)));
    commands.queue(SpawnTerrain(IVec2::new(1, 0)));
    commands.queue(SpawnTerrain(IVec2::new(1, 1)));
}

pub struct SpawnTerrain(pub IVec2);

impl Command for SpawnTerrain {
    fn apply(self, world: &mut World) {
        if world
            .get_resource_mut::<TerrainStore>()
            .expect("TerrainStore to be available")
            .0
            .get(&self.0)
            .is_some()
        {
            warn!("Mesh already exists");
            return;
        }

        let noise = BasicMulti::<Perlin>::new(NOISE_SEED);

        let mut terrain = Mesh::from(
            Plane3d::default()
                .mesh()
                .size(MESH_SIZE, MESH_SIZE)
                .subdivisions(SUBDIVISIONS),
        );

        if let Some(VertexAttributeValues::Float32x3(positions)) =
            terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            for pos in positions.iter_mut() {
                let val = noise.get([
                    (pos[0] as f64 + (MESH_SIZE as f64 * self.0.x as f64)) / NOISE_SCALE,
                    (pos[2] as f64 + (MESH_SIZE as f64 * self.0.y as f64)) / NOISE_SCALE,
                ]);
                pos[1] = val as f32 * TERRAIN_HEIGHT; // safe: Perlin is -1 to 1
            }

            // Generate colors based on height
            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[_, g, _]| {
                    let g = *g / TERRAIN_HEIGHT * 2f32;
                    if g > 0.8f32 {
                        // High peaks - bright white (with bloom)
                        (Color::LinearRgba(LinearRgba {
                            red: 20f32,   // bloom above 1
                            green: 20f32, // bloom above 1
                            blue: 20f32,  // bloom above 1
                            alpha: 1f32,
                        }))
                        .to_linear()
                        .to_f32_array()
                    } else if g > 0.3f32 {
                        // Mid-level terrain - amber/brown
                        Color::from(AMBER_800).to_linear().to_f32_array()
                    } else if g < -0.8f32 {
                        // Deep valleys - black
                        Color::BLACK.to_linear().to_f32_array()
                    } else {
                        // Low areas - green
                        Color::from(GREEN_400).to_linear().to_f32_array()
                    }
                })
                .collect();
            terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        }

        terrain.compute_normals();

        let mesh_handle = world
            .get_resource_mut::<Assets<Mesh>>()
            .expect("meshes to be available")
            .add(terrain);

        let material = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("StandardMaterial db to be available")
            .add(Color::WHITE);

        world.spawn((
            Mesh3d(mesh_handle.clone()),
            MeshMaterial3d(material),
            Transform::from_xyz(
                self.0.x as f32 * MESH_SIZE,
                0f32,
                self.0.y as f32 * MESH_SIZE,
            ),
            GlobalTransform::default(),
            TerrainMesh,
        ));

        // Store the mesh handle
        world
            .get_resource_mut::<TerrainStore>()
            .expect("TerrainStore to be available")
            .0
            .insert(self.0, mesh_handle);
    }
}

fn toggle_wireframe(
    mut commands: Commands,
    landscapes_wireframe: Query<Entity, (With<TerrainMesh>, With<Wireframe>)>,
    landscapes: Query<Entity, (With<TerrainMesh>, Without<Wireframe>)>,
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

fn control_ship(inputs: Res<ButtonInput<KeyCode>>, mut ships: Query<&mut Transform, With<Ship>>) {
    let mut direction = Vec2::new(0f32, 0f32);
    if inputs.pressed(KeyCode::KeyW) {
        direction.y -= 1f32;
    }
    if inputs.pressed(KeyCode::KeyS) {
        direction.y += 1f32;
    }
    if inputs.pressed(KeyCode::KeyA) {
        direction.x -= 1f32;
    }
    if inputs.pressed(KeyCode::KeyD) {
        direction.x += 1f32;
    }
    for mut ship in &mut ships {
        ship.translation.x += direction.x * 1f32;
        ship.translation.z += direction.y * 1f32;
    }
}

fn control_ship_camera(
    ship: Single<&Transform, (With<Ship>, Without<ShipCamera>)>,
    mut orbit: Single<&mut PanOrbitCamera, With<ShipCamera>>,
) {
    orbit.target_focus = Vec3::new(ship.translation.x, ship.translation.y, ship.translation.z)
}

#[allow(clippy::type_complexity)]
fn cleanup_terrain_experiment(
    mut commands: Commands,
    entities: Query<
        Entity,
        Or<(
            With<TerrainMesh>,
            With<Ship>,
            With<Camera3d>,
            With<DirectionalLight>,
        )>,
    >,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn();
    }
}

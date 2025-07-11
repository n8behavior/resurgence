use bevy::prelude::*;

fn main() {
    println!("🌱 Resurgence - Post-Apocalyptic Earth Reclamation");
    println!();
    println!("Available experiments:");
    println!("  cargo run --example growth_overlay    # Growth-Type Overlay Demo");
    println!("  cargo run --example terrain_proc_gen  # Procedural Terrain Generation");
    println!();
    println!("Run experiments to test individual game mechanics.");
    println!("See examples/experiments.md for the full experiment roadmap.");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Basic camera for the empty scene
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0f32, 10f32, 10f32)).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // Welcome text (will appear in console)
    info!("Welcome to Resurgence! Check the console for available experiments.");
}

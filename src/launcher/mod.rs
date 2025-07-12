use bevy::prelude::*;

use crate::experiments::{AppState, all_experiments};

// UI Colors
const BUTTON_COLOR: Color = Color::srgb(0.2, 0.2, 0.2);
const BUTTON_HOVER_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
const BUTTON_PRESSED_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const TEXT_COLOR: Color = Color::WHITE;
const TITLE_COLOR: Color = Color::srgb(0.8, 0.9, 1.0);

#[derive(Component)]
pub struct LauncherUI;

#[derive(Component)]
pub struct ExperimentButton {
    pub target_state: AppState,
}

pub struct LauncherPlugin;

impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Launcher), setup_launcher)
            .add_systems(
                Update,
                (handle_button_interactions, update_button_colors)
                    .run_if(in_state(AppState::Launcher)),
            )
            .add_systems(OnExit(AppState::Launcher), cleanup_launcher);
    }
}

fn setup_launcher(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load Nerd Font for icons
    let icon_font = asset_server.load("fonts/UbuntuMonoNerdFont-Regular.ttf");
    // Camera for UI
    commands.spawn(Camera2d);

    // Root UI container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            LauncherUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("🌱 Resurgence Experiments"),
                TextLayout::new_with_justify(JustifyText::Center),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(TITLE_COLOR),
                Node {
                    margin: UiRect::bottom(Val::Px(40.0)),
                    ..default()
                },
            ));

            // Subtitle
            parent.spawn((
                Text::new("Post-Apocalyptic Earth Reclamation"),
                TextLayout::new_with_justify(JustifyText::Center),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::bottom(Val::Px(60.0)),
                    ..default()
                },
            ));

            // Generate buttons for all experiments
            for experiment in all_experiments() {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(600.0),
                            height: Val::Px(100.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..default()
                        },
                        BackgroundColor(BUTTON_COLOR),
                        BorderRadius::all(Val::Px(10.0)),
                        ExperimentButton {
                            target_state: experiment.app_state(),
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new(format!("{} {}", experiment.icon(), experiment.name())),
                            TextFont {
                                font: icon_font.clone(),
                                font_size: 24.0,
                                ..default()
                            },
                            TextColor(TEXT_COLOR),
                        ));
                    });
            }

            // Instructions
            parent.spawn((
                Text::new("Click an experiment to launch it • ESC to return to launcher"),
                TextLayout::new_with_justify(JustifyText::Center),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(40.0)),
                    ..default()
                },
            ));
        });
}

fn handle_button_interactions(
    button_query: Query<(&Interaction, &ExperimentButton)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button) in button_query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(button.target_state);
        }
    }
}

fn update_button_colors(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), With<ExperimentButton>>,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        *background_color = match interaction {
            Interaction::Pressed => BUTTON_PRESSED_COLOR.into(),
            Interaction::Hovered => BUTTON_HOVER_COLOR.into(),
            Interaction::None => BUTTON_COLOR.into(),
        };
    }
}

fn cleanup_launcher(
    mut commands: Commands,
    launcher_entities: Query<Entity, With<LauncherUI>>,
    camera_entities: Query<Entity, With<Camera2d>>,
) {
    for entity in launcher_entities.iter() {
        commands.entity(entity).despawn();
    }
    for entity in camera_entities.iter() {
        commands.entity(entity).despawn();
    }
}

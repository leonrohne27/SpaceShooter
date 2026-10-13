use crate::AppState;
use crate::button::*;
use bevy::{
    prelude::*,
    ui_widgets::{Activate, observe},
};

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
struct PlayButton;
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), spawn_menu);
        app.add_systems(OnExit(AppState::Menu), despawn_menu);
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        MenuUI,
        ImageNode::new(asset_server.load("Backgrounds/darkPurple.png")),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        ZIndex(-1),
    ));

    commands
        .spawn((
            MenuUI,
            Node {
                width: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                top: Val::Px(20.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Space Shooter"),
                TextFont {
                    font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
                    font_size: FontSize::Px(50.0),
                    ..default()
                },
            ));
        });

    commands
        .spawn((
            MenuUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(10.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                button(&asset_server, "Play"),
                PlayButton,
                observe(play_clicked),
            ));
            parent.spawn(button(&asset_server, "Settings"));
            parent.spawn(button(&asset_server, "Quit Game"));
        });
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn play_clicked(activate: On<Activate>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}

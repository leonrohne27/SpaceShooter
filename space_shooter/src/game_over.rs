use crate::AppState;
use crate::button::*;
use crate::health::Health;
use crate::player::Player;
use crate::score::Score;
use bevy::{
    prelude::*,
    ui_widgets::{Activate, observe},
};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_player_dead.run_if(in_state(AppState::InGame)));
        app.add_systems(OnEnter(AppState::GameOver), spawn_game_over_screen);
    }
}

// Player has 0 HP -> switch to Game Over
fn check_player_dead(
    player_query: Query<&Health, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok(health) = player_query.single() {
        if health.current == 0 {
            next_state.set(AppState::GameOver);
        }
    }
}

fn spawn_game_over_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    let font = asset_server.load("Bonus/kenvector_future_thin.ttf");

    commands.spawn((
        DespawnOnExit(AppState::GameOver),
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
            DespawnOnExit(AppState::GameOver),
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
                Text::new("Game Over"),
                TextFont {
                    font: font.clone().into(),
                    font_size: FontSize::Px(50.0),
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.2, 0.2)),
            ));
            parent.spawn((
                Text::new(format!("Score: {}", score.points)),
                TextFont {
                    font: font.into(),
                    font_size: FontSize::Px(30.0),
                    ..default()
                },
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));
            parent.spawn((
                button(&asset_server, "Play Again"),
                observe(play_again_clicked),
            ));
            parent.spawn((
                button(&asset_server, "Main Menu"),
                observe(main_menu_clicked),
            ));
        });
}

fn play_again_clicked(_activate: On<Activate>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::InGame);
}

fn main_menu_clicked(_activate: On<Activate>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Menu);
}

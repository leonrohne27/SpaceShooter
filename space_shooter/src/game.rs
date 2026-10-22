use crate::AppState;
use bevy::prelude::*;
use crate::health::Health;
use crate::player::Player;

pub struct GamePlugin;

#[derive(Component)]
pub struct HealthUI;
#[derive(Component)]
pub struct HeartIcon{
    pub index: u8,
}
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), load_game);
    }
}

fn load_game(mut commands: Commands, asset_server: Res<AssetServer>, player_query: Query<&Health, With<Player>>,) {

    let max_hp = if let Ok(health) = player_query.single() {
        health.max as u8
    } else {
        5 // Fallback, falls kein Player gefunden wurde
    };
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            top: Val::Px(20.0),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Text::new("This is a new Game"),
                TextFont {
                    font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
                    font_size: FontSize::Px(50.0),
                    ..default()
                },
            ));
            parent.spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(5.0),
                ..default()
            })
                .with_children(|heart_container| {
                    for i in 0..max_hp {
                        heart_container.spawn((
                            ImageNode::new(asset_server.load("PNG/UI/playerLife1_blue.png")),
                            Node {
                                width: Val::Px(32.0),
                                height: Val::Px(32.0),
                                ..default()
                            },
                            HeartIcon { index: i },
                        ));
                    }
                });
        });
}

pub fn update_health_ui_system(
    player_query: Query<&Health, With<Player>>,
    mut text_query: Query<&mut Text, With<HealthUI>>,
) {
    if let Ok(health) = player_query.single() {
        for mut text in text_query.iter_mut() {
            **text = format!("HP: {:.0} / {:.0}", health.current, health.max);
        }
    }
}

use crate::AppState;
use crate::player::Player;
use bevy::prelude::*;

pub struct HealthPlugin;

#[derive(Component, Clone, Copy, Debug)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct HealthUI;

#[derive(Component)]
pub struct HeartIcon {
    pub index: u8,
}

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_hearts);
        app.add_systems(
            Update,
            update_health_ui_system.run_if(in_state(AppState::InGame)),
        );
    }
}

impl Health {
    pub fn new(max: f32) -> Self {
        Self { current: max, max }
    }
}

fn spawn_hearts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Health, With<Player>>,
) {
    let max_hp = if let Ok(health) = player_query.single() {
        health.max as u8
    } else {
        5 // Fallback, falls kein Player gefunden wurde
    };
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            bottom: Val::Px(5.0),
            ..default()
        },))
        .with_children(|parent| {
            parent
                .spawn(Node {
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

fn update_health_ui_system(
    player_query: Query<&Health, With<Player>>,
    mut text_query: Query<&mut Text, With<HealthUI>>,
) {
    if let Ok(health) = player_query.single() {
        for mut text in text_query.iter_mut() {
            **text = format!("HP: {:.0} / {:.0}", health.current, health.max);
        }
    }
}

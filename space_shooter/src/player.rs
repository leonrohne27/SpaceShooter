use crate::AppState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), load_player);
        app.add_systems(Update, player_movement.run_if(in_state(AppState::InGame)));
    }
}

fn load_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        Sprite::from_image(asset_server.load("PNG/playerShip1_blue.png")),
        Transform::from_xyz(0.0, -200.0, 1.0),
    ));
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
    window: Query<&mut Window>,
) {
    let window = window.single().unwrap();
    let right = window.resolution.width() / 2.0 - 50.0;
    let left = -right;
    for mut transform in &mut query {
        if keys.pressed(KeyCode::ArrowRight) {
            transform.translation.x += 600. * time.delta_secs();
        }
        if keys.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= 600. * time.delta_secs();
        }

        if transform.translation.x > right {
            transform.translation.x = right;
        }

        if transform.translation.x < left {
            transform.translation.x = left;
        }
    }
}

use crate::AppState;
use crate::enemy::{Enemy, EnemyShootTimer};
use crate::player::Player;
use bevy::prelude::*;

pub struct BulletPlugin;

#[derive(Component)]
pub struct PlayerBullet;

#[derive(Component)]
pub struct EnemyBullet;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_bullet_shot,
                enemy_bullet_shot,
                player_bullet_movement,
                enemy_bullet_movement,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn player_bullet_shot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<&mut Transform, With<Player>>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player_query.single().unwrap();

    commands.spawn((
        PlayerBullet,
        Sprite::from_image(asset_server.load("PNG/Lasers/laserBlue05.png")),
        Transform::from_xyz(
            player_transform.translation.x,
            player_transform.translation.y + 40.0,
            1.0,
        ),
    ));
}

pub fn enemy_bullet_shot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemy_query: Query<(&Transform, &EnemyShootTimer), With<Enemy>>,
) {
    for (transform, shoot_timer) in enemy_query.iter() {
        if shoot_timer.0.just_finished() {
            commands.spawn((
                EnemyBullet,
                Sprite::from_image(asset_server.load("PNG/Lasers/laserRed05.png")),
                Transform::from_xyz(transform.translation.x, transform.translation.y - 40.0, 1.0),
            ));
        }
    }
}

fn player_bullet_movement(
    mut bullet_query: Query<&mut Transform, With<PlayerBullet>>,
    time: Res<Time>,
) {
    for mut bullet in bullet_query.iter_mut() {
        bullet.translation.y += 1000.0 * time.delta_secs();
    }
}

fn enemy_bullet_movement(
    mut bullet_query: Query<&mut Transform, With<EnemyBullet>>,
    time: Res<Time>,
) {
    for mut bullet in bullet_query.iter_mut() {
        bullet.translation.y -= 600.0 * time.delta_secs();
    }
}

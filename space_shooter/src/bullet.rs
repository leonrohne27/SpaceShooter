use crate::AppState;
use crate::player::Player;
use bevy::prelude::*;

pub struct BulletPlugin;

#[derive(Component)]
struct Bullet;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (bullet_shot, bullet_movement).run_if(in_state(AppState::InGame)),
        );
    }
}

fn bullet_shot(
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
        Bullet,
        Sprite::from_image(asset_server.load("PNG/Lasers/laserBlue05.png")),
        Transform::from_xyz(
            player_transform.translation.x,
            player_transform.translation.y + 40.0,
            1.0,
        ),
    ));
}

fn bullet_movement(mut bullet_query: Query<&mut Transform, With<Bullet>>, time: Res<Time>) {
    for mut bullet in &mut bullet_query {
        bullet.translation.y += 1000. * time.delta_secs();
    }
}

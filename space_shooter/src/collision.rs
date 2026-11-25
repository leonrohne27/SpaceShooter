use crate::AppState;
use crate::asteroid::Asteroid;
use crate::bullet::EnemyBullet;
use crate::bullet::PlayerBullet;
use crate::enemy::Enemy;
use crate::health::Health;
use crate::player::Player;
use crate::score::Score;
use bevy::prelude::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_enemy_bullet_collision,
                player_asteroid_collision,
                player_bullet_enemy_collision,
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

// Enemy Bullet hits player -> -1 HP
fn player_enemy_bullet_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    bullet_query: Query<(Entity, &Transform), With<EnemyBullet>>,
) {
    let Ok((player_transform, mut health)) = player_query.single_mut() else {
        return;
    };

    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let distance = player_transform
            .translation
            .truncate()
            .distance(bullet_transform.translation.truncate());

        if distance < 35.0 {
            health.current = health.current.saturating_sub(1);

            // Despawn bullet after hit
            commands.entity(bullet_entity).despawn();
        }
    }
}

// Asteroid hits a player -> -1 HP
fn player_asteroid_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    asteroid_query: Query<(Entity, &Transform), With<Asteroid>>,
) {
    let Ok((player_transform, mut health)) = player_query.single_mut() else {
        return;
    };

    for (asteroid_entity, asteroid_transform) in asteroid_query.iter() {
        let distance = player_transform
            .translation
            .truncate()
            .distance(asteroid_transform.translation.truncate());

        if distance < 40.0 {
            health.current = health.current.saturating_sub(1);

            // Despawn asteroid after hit
            commands.entity(asteroid_entity).despawn();
        }
    }
}

// Bullet from Player hits an Enemy -> Enemy despawn
fn player_bullet_enemy_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<PlayerBullet>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut score: ResMut<Score>,
) {
    for (bullet_entity, bullet_transform) in bullet_query.iter() {
        let bullet_pos = bullet_transform.translation.truncate();

        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let enemy_pos = enemy_transform.translation.truncate();

            if bullet_pos.distance(enemy_pos) < 35.0 {
                // Despawn enemy and bullet if hit
                commands.entity(enemy_entity).despawn();
                commands.entity(bullet_entity).despawn();

                score.points += 1000;

                break;
            }
        }
    }
}

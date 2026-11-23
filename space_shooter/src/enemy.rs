use crate::AppState;
use bevy::prelude::*;
use rand::prelude::*;

pub struct EnemyPlugin;

#[derive(Resource)]
pub struct EnemySpawnTimer(Timer);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyShootTimer(pub Timer);

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::rng();

        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            rng.random_range(2.0..3.0),
            TimerMode::Once,
        )));
        app.add_systems(Update, (spawn_enemy, enemy_shoot_timer));
    }
}

const X_POSITIONS: [f32; 5] = [-250.0, -125.0, 0.0, 125.0, 250.0];

const Y_POSITIONS: [f32; 3] = [250.0, 130.0, 10.0];

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    let mut free_positions: Vec<Vec2> = Vec::new();

    for &y in Y_POSITIONS.iter() {
        for &x in X_POSITIONS.iter() {
            let potential_position = Vec2::new(x, y);
            let occupied = enemy_query.iter().any(|transform| {
                let enemy_pos = transform.translation.truncate();

                potential_position.distance(enemy_pos) < 100.0
            });

            if !occupied {
                free_positions.push(potential_position);
            }
        }

        if !free_positions.is_empty() {
            break;
        }
    }

    if free_positions.is_empty() {
        return;
    }

    let mut rng = rand::rng();

    let spawn_pos = free_positions[rng.random_range(0..free_positions.len())];

    commands.spawn((
        Enemy,
        EnemyShootTimer(Timer::from_seconds(1.5, TimerMode::Repeating)),
        Sprite::from_image(asset_server.load("PNG/Enemies/enemyRed1.png")),
        Transform::from_xyz(spawn_pos.x, spawn_pos.y, 1.0),
    ));
    timer.0 = Timer::from_seconds(rng.random_range(2.0..3.0), TimerMode::Once);
}

fn enemy_shoot_timer(time: Res<Time>, mut query: Query<&mut EnemyShootTimer, With<Enemy>>) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

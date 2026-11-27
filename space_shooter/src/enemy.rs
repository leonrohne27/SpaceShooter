use crate::AppState;
use crate::difficulty::Difficulty;
use bevy::prelude::*;
use rand::prelude::*;

pub struct EnemyPlugin;

#[derive(Resource)]
pub struct EnemySpawnTimer(Timer);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyShootTimer(pub Timer);

/// The formation slot an enemy occupies (its final position).
#[derive(Component)]
pub struct EnemySlot(pub Vec2);

/// Marks an enemy that is still flying in from above the screen.
#[derive(Component)]
pub struct EnemyEntering;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::rng();

        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(
            rng.random_range(2.0..3.0),
            TimerMode::Once,
        )));
        app.add_systems(
            Update,
            (spawn_enemy, enemy_fly_in, enemy_shoot_timer).run_if(in_state(AppState::InGame)),
        );
    }
}

/// One slot per column across the full screen width -> at most 5 enemies at once.
const X_POSITIONS: [f32; 5] = [-250.0, -125.0, 0.0, 125.0, 250.0];

/// Height of the enemy row, with some distance to the top window edge.
const ENEMY_ROW_Y: f32 = 150.0;

/// How far above the top window edge enemies start.
const SPAWN_OFFSET_ABOVE_SCREEN: f32 = 60.0;
/// How quickly enemies approach their slot (higher = faster); they slow down as they arrive.
const FLY_IN_SPEED: f32 = 6.0;

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    difficulty: Res<Difficulty>,
    enemy_query: Query<&EnemySlot, With<Enemy>>,
    window: Query<&Window>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    // Restart the timer right away, so spawning keeps trying even while the row is full.
    // The interval gets shorter the longer the run lasts.
    let mut rng = rand::rng();
    let (min_interval, max_interval) = difficulty.spawn_interval();
    timer.0 = Timer::from_seconds(
        rng.random_range(min_interval..max_interval),
        TimerMode::Once,
    );

    // Check against the target slots, so enemies still flying in count as occupying theirs.
    let free_positions: Vec<Vec2> = X_POSITIONS
        .iter()
        .map(|&x| Vec2::new(x, ENEMY_ROW_Y))
        .filter(|pos| !enemy_query.iter().any(|slot| pos.distance(slot.0) < 100.0))
        .collect();

    if free_positions.is_empty() {
        return;
    }

    let spawn_pos = free_positions[rng.random_range(0..free_positions.len())];

    let Ok(window) = window.single() else {
        return;
    };
    let start_y = window.resolution.height() / 2.0 + SPAWN_OFFSET_ABOVE_SCREEN;

    commands.spawn((
        Enemy,
        EnemySlot(spawn_pos),
        EnemyEntering,
        EnemyShootTimer(Timer::from_seconds(
            difficulty.shoot_interval(),
            TimerMode::Repeating,
        )),
        Sprite::from_image(asset_server.load("PNG/Enemies/enemyRed1.png")),
        Transform::from_xyz(spawn_pos.x, start_y, 1.0),
    ));
}

/// Moves newly spawned enemies down into their slot, slowing down as they arrive.
fn enemy_fly_in(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &EnemySlot, &mut Transform), With<EnemyEntering>>,
) {
    let t = 1.0 - (-FLY_IN_SPEED * time.delta_secs()).exp();

    for (entity, slot, mut transform) in query.iter_mut() {
        let target_y = slot.0.y;
        transform.translation.y += (target_y - transform.translation.y) * t;

        if (transform.translation.y - target_y).abs() < 1.0 {
            transform.translation.y = target_y;
            commands.entity(entity).remove::<EnemyEntering>();
        }
    }
}

/// Only enemies that have reached their slot start shooting.
fn enemy_shoot_timer(
    time: Res<Time>,
    mut query: Query<&mut EnemyShootTimer, (With<Enemy>, Without<EnemyEntering>)>,
) {
    for mut timer in query.iter_mut() {
        timer.0.tick(time.delta());
    }
}

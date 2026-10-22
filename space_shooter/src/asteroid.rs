use crate::AppState;
use bevy::prelude::*;
use rand::distr::weighted::WeightedIndex;
use rand::prelude::*;

pub struct AsteroidPlugin;

#[derive(Resource)]
pub struct AsteroidSpawner {
    timer: Timer,
}

#[derive(Component)]
struct Asteroid;

#[derive(Debug, Clone, Copy, Component)]
enum AsteroidSize {
    Small,
    Medium,
    Large,
}

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::rng();
        app.insert_resource(AsteroidSpawner {
            // initial Timer until first asteroids spawn
            timer: Timer::from_seconds(rng.random_range(2.0..3.0), TimerMode::Once),
        });
        app.add_systems(
            Update,
            (spawn_asteroid, move_asteroid).run_if(in_state(AppState::InGame)),
        );
    }
}

fn choose_size() -> AsteroidSize {
    let choices: [AsteroidSize; 3] = [
        AsteroidSize::Small,
        AsteroidSize::Medium,
        AsteroidSize::Large,
    ];
    // smaller ones should spawn more often than bigger ones
    let weights = [5, 2, 1];
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = rand::rng();

    // sizes should be random but taking weights into account
    let chosen_size = choices[dist.sample(&mut rng)];
    chosen_size
}

fn spawn_asteroid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut spawner: ResMut<AsteroidSpawner>,
) {
    spawner.timer.tick(time.delta());

    if !spawner.timer.just_finished() {
        return;
    }

    let mut rng = rand::rng();

    // amount of asteroids should vary
    let astroid_spawn_count = rng.random_range(2..=3);

    for _ in 0..astroid_spawn_count {
        let size = choose_size();

        let texture: Handle<Image> = match size {
            AsteroidSize::Small => asset_server.load("PNG/Meteors/meteorBrown_small1.png"),
            AsteroidSize::Medium => asset_server.load("PNG/Meteors/meteorBrown_med3.png"),
            AsteroidSize::Large => asset_server.load("PNG/Meteors/meteorBrown_big2.png"),
        };

        // asteroids should spawn on a random x coordinate within the screen
        let x_coordinate = rng.random_range(-300.0..=300.0);

        commands.spawn((
            Asteroid,
            Sprite::from_image(texture.clone()),
            Transform::from_xyz(x_coordinate, 300.0, 1.0),
        ));
    }

    // asteroids spawn time should vary
    spawner.timer = Timer::from_seconds(rng.random_range(0.3..1.2), TimerMode::Once)
}

fn move_asteroid(mut asteroid_query: Query<&mut Transform, With<Asteroid>>, time: Res<Time>) {
    for mut asteroid in &mut asteroid_query {
        asteroid.translation.y -= 500. * time.delta_secs();
    }
}

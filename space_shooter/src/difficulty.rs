use crate::AppState;
use bevy::prelude::*;

/// Seconds of play until the maximum difficulty is reached.
const TIME_TO_MAX_DIFFICULTY: f32 = 120.0;

/// Seconds between enemy spawns (min, max) at the start and at maximum difficulty.
const SPAWN_INTERVAL_START: (f32, f32) = (2.0, 3.0);
const SPAWN_INTERVAL_MAX: (f32, f32) = (0.5, 0.9);

/// Seconds between shots of a single enemy at the start and at maximum difficulty.
const SHOOT_INTERVAL_START: f32 = 1.5;
const SHOOT_INTERVAL_MAX: f32 = 0.6;

pub struct DifficultyPlugin;

/// Tracks how long the current run has been going and derives the difficulty from it.
#[derive(Resource, Default)]
pub struct Difficulty {
    elapsed: f32,
}

impl Plugin for DifficultyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Difficulty>();
        app.add_systems(OnEnter(AppState::InGame), reset_difficulty);
        app.add_systems(Update, tick_difficulty.run_if(in_state(AppState::InGame)));
    }
}

impl Difficulty {
    /// 0.0 at the start of a run, rising to 1.0 at maximum difficulty.
    pub fn level(&self) -> f32 {
        (self.elapsed / TIME_TO_MAX_DIFFICULTY).clamp(0.0, 1.0)
    }

    /// Range (min, max) for the random time until the next enemy spawn.
    pub fn spawn_interval(&self) -> (f32, f32) {
        let t = self.level();
        (
            lerp(SPAWN_INTERVAL_START.0, SPAWN_INTERVAL_MAX.0, t),
            lerp(SPAWN_INTERVAL_START.1, SPAWN_INTERVAL_MAX.1, t),
        )
    }

    /// Time between two shots of a newly spawned enemy.
    pub fn shoot_interval(&self) -> f32 {
        lerp(SHOOT_INTERVAL_START, SHOOT_INTERVAL_MAX, self.level())
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn reset_difficulty(mut difficulty: ResMut<Difficulty>) {
    difficulty.elapsed = 0.0;
}

fn tick_difficulty(time: Res<Time>, mut difficulty: ResMut<Difficulty>) {
    difficulty.elapsed += time.delta_secs();
}

mod asteroid;
mod bullet;
mod button;
mod collision;
mod enemy;
mod game;
mod health;
mod menu;
mod player;
mod score;

use asteroid::AsteroidPlugin;
use bevy::prelude::*;
use bullet::BulletPlugin;
use collision::CollisionPlugin;
use enemy::EnemyPlugin;
use game::GamePlugin;
use health::HealthPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

#[derive(States, Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
enum AppState {
    #[default]
    Menu,
    InGame,
}

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "SpaceShooter".into(),
                resolution: (600, 600).into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins(MenuPlugin)
        .add_plugins(GamePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(HealthPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CollisionPlugin)
        .add_plugins(ScorePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

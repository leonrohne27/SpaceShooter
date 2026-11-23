use crate::AppState;
use bevy::prelude::*;

pub struct ScorePlugin;

#[derive(Resource)]
pub struct Score {
    pub points: u32,
}

#[derive(Resource)]
pub struct ScoreTimer(pub Timer);

#[derive(Component)]
pub struct ScoreUI;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score { points: 0 });
        app.insert_resource(ScoreTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));

        app.add_systems(OnEnter(AppState::InGame), spawn_score_ui);
        app.add_systems(
            Update,
            (score_over_time, update_score_ui).run_if(in_state(AppState::InGame)),
        );
    }
}

// Display the Score
fn spawn_score_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
            font_size: FontSize::Px(25.0),
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ScoreUI,
    ));
}

fn update_score_ui(score: Res<Score>, mut query: Query<&mut Text, With<ScoreUI>>) {
    if !score.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        **text = format!("Score: {}", score.points);
    }
}

// Points for surviving -> +100 for every sec
fn score_over_time(time: Res<Time>, mut score: ResMut<Score>, mut timer: ResMut<ScoreTimer>) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        score.points += 100;
    }
}

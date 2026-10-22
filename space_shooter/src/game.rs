use crate::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(OnEnter(AppState::InGame), load_game);
    }
}

// fn load_game(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands
//         .spawn((Node {
//             width: Val::Percent(100.0),
//             position_type: PositionType::Absolute,
//             justify_content: JustifyContent::Center,
//             top: Val::Px(20.0),
//             ..default()
//         },))
//         .with_children(|parent| {
//             parent.spawn((
//                 Text::new("This is a new Game"),
//                 TextFont {
//                     font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
//                     font_size: FontSize::Px(50.0),
//                     ..default()
//                 },
//             ));
//         });
// }

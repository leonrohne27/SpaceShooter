use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_menu);
    }
}

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            top: Val::Px(20.0),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Space Shooter"),
                TextFont {
                    font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
                    font_size: FontSize::Px(50.0),
                    ..default()
                },
            ));
        });

    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(15.0),
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Play"),
                TextFont {
                    font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
                    font_size: FontSize::Px(40.0),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("Settings"),
                TextFont {
                    font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
                    font_size: FontSize::Px(40.0),
                    ..default()
                },
            ));
            parent.spawn((
                Text::new("Quit Game"),
                TextFont {
                    font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
                    font_size: FontSize::Px(40.0),
                    ..default()
                },
            ));
        });
}

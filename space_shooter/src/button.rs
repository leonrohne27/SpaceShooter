use bevy::{
    color::palettes::basic::*,
    picking::hover::Hovered,
    prelude::*,
    ui::Pressed,
    ui_widgets::{Activate, Button, observe},
};

#[derive(Component)]
pub struct PlayButton;

pub fn button(asset_server: &AssetServer, text: &'static str) -> impl Bundle {
    (
        Button,
        Node {
            width: px(3000),
            height: px(65),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Text::new(text),
            TextFont {
                font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
                font_size: FontSize::Px(33.0),
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}

use bevy::{
    picking::hover::Hovered,
    prelude::*,
    ui_widgets::Button};

pub fn button(asset_server: &AssetServer, text: &'static str) -> impl Bundle {
    (
        Button,
        Hovered::default(),
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

pub fn button_on_hover(event: On<Insert,Hovered>, mut buttons: Query<(&Hovered, &mut BackgroundColor), With<Button>>,){
    if let Ok((hovered, mut color)) = buttons.get_mut(event.entity) {
        if hovered.get() {
            *color= Color::srgb(0.25, 0.25, 0.25).into();
        } else {
            *color = Color::srgb(0.15, 0.15, 0.15).into();
        };
    }
    
}

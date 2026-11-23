use bevy::{
    picking::hover::Hovered, prelude::*, ui_widgets::Button};

pub fn button(asset_server: &AssetServer, text: &'static str) -> impl Bundle {
    (
        Button,
        Hovered::default(),
        Node {
            width: px(300),
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

pub fn button_on_hover(time: Res<Time>, buttons: Query<(&Hovered, &Children), With<Button>>, mut text_query: Query<&mut TextColor, With<Text>>){
    let hue = (time.elapsed_secs()*120.0) % 360.0;
    let rainbow = Color::hsl(hue, 1.0,0.6);

    for (hovered, children) in &buttons {
        let Ok(mut text_color) = text_query.get_mut(children[0]) else {
            continue;
        };
        if hovered.get() {
            *text_color = TextColor(rainbow);
        } else {
            *text_color = TextColor(Color::WHITE);
        };
    }
}

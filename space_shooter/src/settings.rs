use bevy::{
    input_focus::{FocusCause, InputFocus, tab_navigation::{TabIndex},}, prelude::*, ui_widgets::{Activate,
        MenuAction, MenuButton, MenuEvent, MenuItem, MenuPopup,
        Slider, SliderRange, SliderThumb, SliderValue, ValueChange,
    observe}, window::{MonitorSelection, PrimaryWindow, WindowMode},
};

use crate::{AppState, button::button};

#[derive(Component)]
struct SettingUI;

pub struct SettingsPlugin;

#[derive(Component)]
struct DisplayModeMenu;
#[derive(Component)]
struct DisplayModeButton;

#[derive(Component)]
struct AudioSlider;
#[derive(Component)]
struct AudioSliderThumb;
#[derive(Component)]
struct AudioSliderLabel;


impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), spawn_settings);
        app.add_systems(OnExit(AppState::Settings), despawn_settings);
    }
}


fn spawn_settings(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let current_mode = windows
        .single()
        .map(|window| window_mode_label(&window.mode))
        .unwrap_or("Window");
    let value = 0.5;

    commands.spawn((
        SettingUI,
        ImageNode::new(asset_server.load("Backgrounds/darkPurple.png")),
        Node {
            width: percent(100),
            height: percent(100),
            position_type: PositionType::Absolute,
            ..default()
        },
        ZIndex(-1),
    ));

    commands
        .spawn((
            SettingUI,
            Node {
                width: percent(100),
                position_type: PositionType::Absolute,
                top: px(20),
                bottom: px(0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: px(12),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Settings"),
                TextFont {
                    font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
                    font_size: FontSize::Px(50.0),
                    ..default()
                },
            ));

            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: px(16),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Display Mode: "),
                        TextFont {
                            font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
                            font_size: FontSize::Px(22.0),
                            ..default()
                        },
                    ));

                    row.spawn((
                        DisplayModeMenu,
                        Node::default(),
                        observe(display_mode_menu_event),
                        children![(
                            button(&asset_server, current_mode),
                            DisplayModeButton,
                            MenuButton,
                            TabIndex(0),
                        )],
                    ));
                });
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    column_gap: px(16),
                    ..default()
                })
                .with_children(|row| {
                    row.spawn((
                        Text::new("Volume:"),
                        TextFont {
                            font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
                            font_size: FontSize::Px(22.0),
                            ..default()
                        },
                    ));

                    row.spawn((
                        AudioSlider,
                        Slider::default(),
                        SliderRange::new(0.0, 1.0),
                        SliderValue(value),
                        observe(audio_slider_changed),
                        Node {
                            width: px(320),
                            height: px(8),
                            ..default()
                        },
                        children![
                            (Node{
                                width: percent(100),
                                height: px(8),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.35, 0.35, 0.40)),
                            ),
                                (Node {
                                    position_type: PositionType::Absolute,
                                    left: px(0),
                                    right: px(20),
                                    height: px(20),
                                    ..default()
                                },
                            children![(
                                SliderThumb,
                                AudioSliderThumb,
                                Node {
                                    position_type: PositionType::Absolute,
                                    left: percent(value * 100.0),
                                    width: px(20),
                                    height: px(20),
                                    ..default()
                                },
                                BackgroundColor(Color::WHITE),
                            )],
                            ),
                        ],
                        
                    ));
                    row.spawn((
                        AudioSliderLabel,
                        Text::new(format!("{:.0}%", value * 100.0)),
                        TextFont {
                            font: asset_server.load("Bonus/kenvector_future_thin.ttf").into(),
                            font_size: FontSize::Px(22.0),
                            ..default()
                        },
                    ));
                });
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: px(24),
                    left: px(0),
                    width: percent(100),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(
                    button(&asset_server, "Back"),
                    observe(back_clicked),
                )],
            ));
        });
}
            

fn despawn_settings(mut commands: Commands, query: Query<Entity, With<SettingUI>>){
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn display_mode_menu_event(event: On<MenuEvent>,
    menu: Single<(Entity, &Children), With<DisplayModeMenu>>,
    popups: Query<Entity, With<MenuPopup>>,
    buttons: Query<Entity, With<MenuButton>>,
    assets: Res<AssetServer>,
    mut focus: ResMut<InputFocus>,
    mut commands: Commands,
){
    let (anchor, children) = menu.into_inner();
    let popup = children.iter().find_map(|child| popups.get(child).ok());

    match event.action {
        MenuAction::Open(_) => {
            if popup.is_none(){
                spawn_mode_menu(anchor,&assets, commands);
            }
        }
        MenuAction::Toggle => match popup {
            Some(popup) => commands.entity(popup).despawn(),
            None => spawn_mode_menu(anchor, &assets, commands),
        },
        MenuAction::CloseAll => {
            if let Some(popup) = popup {
                commands.entity(popup).despawn();
            }
        }
        MenuAction::FocusRoot => {
            if let Some(button) = children.iter().find_map(|child| buttons.get(child).ok()) {
                focus.set(button, FocusCause::Navigated);
            }
        }
    }

}

fn spawn_mode_menu(anchor: Entity, assets: &AssetServer, mut commands: Commands,){
    commands.entity(anchor).with_children(|parent| {
        parent.spawn((
            MenuPopup::default(),
            Node {
                position_type: PositionType::Absolute,
                top: percent(100),
                left: px(0),
                width: percent(100),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            GlobalZIndex(100),
            BackgroundColor(Color::srgb(0.20, 0.20, 0.25)),
            children![
                display_mode_item(WindowMode::Windowed, assets),
                display_mode_item(WindowMode::BorderlessFullscreen(MonitorSelection::Current), assets),
               // display_mode_item(WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current), assets),
            ]
        ));
    });
}

fn display_mode_item(mode: WindowMode, assets: &AssetServer) -> impl Bundle {
    let label = window_mode_label(&mode);
      (
        MenuItem,
        TabIndex(0),
        Node {
            padding: UiRect::all(px(10)),
            ..default()
        },
        children![(Text::new(label),
            TextFont {
                font: assets.load("Bonus/kenvector_future_thin.ttf").into(),
                font_size: FontSize::Px(22.0),
                ..default()
            },
            TextColor(Color::WHITE),
        )],
        observe(move |_: On<Activate>,
            mut windows: Query<&mut Window, With<PrimaryWindow>>,
            buttons: Query<&Children, With<DisplayModeButton>>,
            mut texts: Query<&mut Text>| {
                let Ok(mut window) = windows.single_mut() else {
                    return;
                };
                window.mode = mode.clone();

                if let Ok(children) = buttons.single() {
                    if let Some(text_entity) = children.iter().next() {
                        if let Ok(mut text) = texts.get_mut(text_entity) {
                            *text = Text::new(label);
                        }
                    }
                }
            },
        ),
    )
}

fn window_mode_label(mode: &WindowMode) -> &'static str {
    match mode {
        WindowMode::Windowed => "Window",
        WindowMode::BorderlessFullscreen(_) => "Borderless fullscreen",
        WindowMode::Fullscreen(..) => "Fullscreen",
    }
}

fn back_clicked(_event: On<Activate>, mut next_state: ResMut<NextState<AppState>>){
    next_state.set(AppState::Menu);
}

fn audio_slider_changed(event: On<ValueChange<f32>>,
    mut commands: Commands, 
    mut thumbs: Query<&mut Node, With<AudioSliderThumb>>, 
    mut labels: Query<&mut Text, With<AudioSliderLabel>>,){

    let value = event.value;

    commands
        .entity(event.source)
        .insert(SliderValue(value));

    if let Ok(mut thumb) = thumbs.single_mut() {
        thumb.left = percent(value * 100.0);
    }

    if let Ok(mut label) = labels.single_mut(){
        *label = Text::new(format!("{:.0}%", value * 100.0));
        
    }
}
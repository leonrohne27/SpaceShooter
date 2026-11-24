use crate::{button::button, AppState};
use bevy::{
    app::AppExit,
    prelude::*,
    ui_widgets::{observe, Activate},
};

pub struct EscMenuPlugin;

#[derive(Component)]
struct EscMenuUI;

#[derive(Resource, Default)]
pub struct EscMenuState {
    open: bool,
}

impl Plugin for EscMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EscMenuState>()
            .add_systems(
                Update,
                toggle_escmenu.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnExit(AppState::InGame), despawn_escmenu);
    }
}

fn toggle_escmenu(
    keys: Res<ButtonInput<KeyCode>>,
    mut menu: ResMut<EscMenuState>,
    mut time: ResMut<Time<Virtual>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    query: Query<Entity, With<EscMenuUI>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }

    menu.open = !menu.open;

    if menu.open {
        time.pause();
        spawn_escmenu(commands, assets);
    } else {
        time.unpause();

        for entity in &query {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_escmenu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            EscMenuUI,
            Node {
                position_type: PositionType::Absolute,
                width: percent(100),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: px(16),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.03, 0.10, 0.95)),
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Paused"),
                TextFont {
                    font: asset_server
                        .load("Bonus/kenvector_future_thin.ttf")
                        .into(),
                    font_size: FontSize::Px(50.0),
                    ..default()
                },
            ));

            parent.spawn((
                button(&asset_server, "Back to Menu"),
                observe(back_to_menu),
            ));

            parent.spawn((
                button(&asset_server, "Quit"),
                observe(quit_clicked),
            ));
        });
}

fn back_to_menu(
    _event: On<Activate>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::Menu);
}

fn quit_clicked(
    _event: On<Activate>,
    mut exit: MessageWriter<AppExit>,
) {
    exit.write(AppExit::Success);
}

fn despawn_escmenu(
    mut commands: Commands,
    query: Query<Entity, With<EscMenuUI>>,
    mut menu: ResMut<EscMenuState>,
    mut time: ResMut<Time<Virtual>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    menu.open = false;
    time.unpause();
}

pub fn gameplay_running(
    state: Res<State<AppState>>,
    menu: Res<EscMenuState>,
) -> bool {
    *state.get() == AppState::InGame && !menu.open
}
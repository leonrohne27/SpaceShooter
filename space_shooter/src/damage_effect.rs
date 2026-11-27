use crate::AppState;
use crate::health::Health;
use crate::player::Player;
use bevy::prelude::*;

/// How long the red edge glow lasts after taking damage (seconds).
const FLASH_DURATION: f32 = 0.4;
/// Maximum opacity of the outermost glow layer.
const FLASH_MAX_ALPHA: f32 = 0.4;
/// Thickness of each glow layer in pixels.
const LAYER_WIDTH: f32 = 6.0;
/// Relative strength of each layer, from the screen edge inwards.
/// Several stacked layers with decreasing opacity give a soft glow.
const LAYER_STRENGTHS: [f32; 5] = [1.0, 0.65, 0.4, 0.2, 0.08];

pub struct DamageEffectPlugin;

#[derive(Resource)]
struct DamageFlash {
    timer: Timer,
}

#[derive(Component)]
struct DamageFlashLayer {
    strength: f32,
}

impl Plugin for DamageEffectPlugin {
    fn build(&self, app: &mut App) {
        let mut timer = Timer::from_seconds(FLASH_DURATION, TimerMode::Once);
        // Start finished so nothing is shown until the first hit.
        timer.finish();

        app.insert_resource(DamageFlash { timer });
        app.add_systems(Startup, spawn_flash_overlay);
        app.add_systems(
            Update,
            (
                detect_damage.run_if(in_state(AppState::InGame)),
                update_flash_overlay,
            )
                .chain(),
        );
    }
}

fn spawn_flash_overlay(mut commands: Commands) {
    for (i, strength) in LAYER_STRENGTHS.iter().enumerate() {
        let inset = Val::Px(i as f32 * LAYER_WIDTH);
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                top: inset,
                bottom: inset,
                left: inset,
                right: inset,
                border: UiRect::all(Val::Px(LAYER_WIDTH)),
                ..default()
            },
            BorderColor::all(Color::NONE),
            GlobalZIndex(100),
            Pickable::IGNORE,
            DamageFlashLayer {
                strength: *strength,
            },
        ));
    }
}

/// Starts the flash whenever the player's health drops.
fn detect_damage(
    player_query: Query<&Health, With<Player>>,
    mut flash: ResMut<DamageFlash>,
    mut last_health: Local<Option<u8>>,
) {
    let Ok(health) = player_query.single() else {
        *last_health = None;
        return;
    };

    if let Some(previous) = *last_health {
        if health.current < previous {
            flash.timer.reset();
        }
    }
    *last_health = Some(health.current);
}

/// Fades the red edge out over the flash duration.
fn update_flash_overlay(
    time: Res<Time>,
    mut flash: ResMut<DamageFlash>,
    mut layers: Query<(&DamageFlashLayer, &mut BorderColor)>,
) {
    flash.timer.tick(time.delta());

    let remaining = 1.0 - flash.timer.fraction();
    let intensity = remaining * remaining;

    for (layer, mut border) in &mut layers {
        let alpha = FLASH_MAX_ALPHA * layer.strength * intensity;
        *border = BorderColor::all(Color::srgba(1.0, 0.0, 0.0, alpha));
    }
}

mod menu;

use bevy::prelude::*;
use menu::MenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MenuPlugin)
        .run();
}

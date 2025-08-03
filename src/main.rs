use bevy::app::App;
use bevy::prelude::*;
mod menu;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(menu::DisplayQuality::Medium)
        .insert_resource(menu::Volume(7))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((splash::spash_plugin, menu::menu_plugin, game::game_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

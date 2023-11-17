// Bevy recommended for better clippy use
#![allow(clippy::type_complexity)]

use bevy::{prelude::*, winit::WinitSettings};
mod main_screen;
mod setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup::setup)
        .add_systems(Update, (bevy::window::close_on_esc, main_screen::button_system))
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;



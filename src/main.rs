// Bevy recommended for better clippy use
#![allow(clippy::type_complexity)]

use bevy::{prelude::*, prelude::Display, winit::WinitSettings};
mod main_screen;
mod setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, (main_screen::setup))
        .add_systems(
            Update, (
                bevy::window::close_on_esc, 
                main_screen::buttons_handler::<Display>,
                main_screen::buttons_handler::<Visibility>,
                main_screen::text_hover,
            ),
        )
        .run();
}

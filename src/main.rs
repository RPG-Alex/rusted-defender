// Bevy recommended for better clippy use
#![allow(clippy::type_complexity)]

use bevy::{
    prelude::*, 
};
mod ui;
mod setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, 
            ui::main_screen)
        .add_systems(
            Update, (
                ui::buttons_handler::<Display>,
                ui::buttons_handler::<Visibility>,
                ui::text_hover,
                bevy::window::close_on_esc,
                ui::set_background_size_to_window,
            ),
        )
        .run();
}


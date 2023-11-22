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

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;


// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
//         .insert_resource(WinitSettings::desktop_app())
//         .add_systems(Startup, setup)
//         .add_systems(
//             Update,
//             (
//                 buttons_handler::<Display>,
//                 buttons_handler::<Visibility>,
//                 text_hover,
//             ),
//         )
//         .run();
// }
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, {bevy::window::close_on_esc})
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((SpriteBundle {
        sprite: Sprite { 
            ..default()
         },
         texture: asset_server.load("backgrounds/splash.png"),
         transform: Transform::from_xyz(0.0, 0.0, -10.0),
         ..default()
    }));
}
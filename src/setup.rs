use bevy::{prelude::*, winit::WinitSettings};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
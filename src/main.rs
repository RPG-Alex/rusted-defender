//! Renders a 2D scene containing a single, moving sprite.
use bevy::{prelude::*, audio::{VolumeLevel, self}, input::keyboard::KeyboardInput};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (sprite_auto_movement,sprite_control))
        .run();
}


///Create enum for distinguishing between sprites
#[derive(Component, PartialEq)]
enum SpriteType {
    Player, 
    Enemy,
    Background,
}


///enum for defining direction (Cardinal style)
#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Adding the background
    commands.spawn(SpriteBundle {
        texture: asset_server.load("backgrounds/rusted-defender-home-screen.png"),
        transform: Transform::from_xyz(0., 0., -10.), // Ensure Z-coordinate is behind other entities
        ..default()
    }).insert(SpriteType::Background);
    //player sprite
    commands.spawn((
        SpriteBundle {
            //Adjust Sprite Size
            sprite: Sprite {
                custom_size: Some(Vec2::new(200.0, 200.0)),
                ..default()
            },
            texture: asset_server.load("sprites/mech-sprite.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    )).insert(SpriteType::Player);
    //enemy sprite
    commands.spawn((
        SpriteBundle {
            //Adjust Sprite Size
            sprite: Sprite {
                custom_size: Some(Vec2::new(200.0, 200.0)),
                ..default()
            },
            texture: asset_server.load("sprites/enemy-sprite-1.png"),
            transform: Transform::from_xyz(200., 200., 0.),
            ..default()
        },
        Direction::Right,
    )).insert(SpriteType::Enemy);

    commands.spawn(AudioBundle {
        source: asset_server.load("sound/background-track-1.ogg"),
        settings: PlaybackSettings { mode: (audio::PlaybackMode::Loop), volume: (audio::Volume::new_relative(1.0)), speed: (1.0), paused: (false) },
        ..default()
    });
}


/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_auto_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut sprite, mut transform) in &mut sprite_position {
        match *sprite {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
        }

        // Check the boundaries and update the direction
        match *sprite {
            Direction::Up if transform.translation.y >= 200. => *sprite = Direction::Right,
            Direction::Right if transform.translation.x >= 200. => *sprite = Direction::Down,
            Direction::Down if transform.translation.y <= -200. => *sprite = Direction::Left,
            Direction::Left if transform.translation.x <= -200. => *sprite = Direction::Up,
            _ => {}
        }
        
        //println!("{:?}", transform);

        

    }
    
}

fn sprite_control(mut sprite_position: Query<(&mut Transform, &mut SpriteType)>, keyboard_input: Res<Input<KeyCode>>){
        
    for (mut transform, sprite_type) in &mut sprite_position {
        if *sprite_type == SpriteType::Player {
            if keyboard_input.pressed(KeyCode::Left) {
                transform.translation.x -= 10.0;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                transform.translation.x += 10.0
            }
            if keyboard_input.pressed(KeyCode::Down) {
                transform.translation.y -= 10.0;
            }
            if keyboard_input.pressed(KeyCode::Up) {
                transform.translation.y += 10.0
            }
        }

    }

}
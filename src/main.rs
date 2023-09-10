//! Renders a 2D scene containing a single, moving sprite.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

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
    });

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/mech-sprite.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Direction::Left,
    ));
}


/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
        }

        // Check the boundaries and update the direction
        match *logo {
            Direction::Up if transform.translation.y >= 200. => *logo = Direction::Right,
            Direction::Right if transform.translation.x >= 200. => *logo = Direction::Down,
            Direction::Down if transform.translation.y <= -200. => *logo = Direction::Left,
            Direction::Left if transform.translation.x <= -200. => *logo = Direction::Up,
            _ => {}
        }
        
        //println!("{:?}", transform);

        

    }
    
}


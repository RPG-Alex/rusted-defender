//Import the necessary bevy features
use bevy::{
    prelude::*, 
    audio, sprite::collide_aabb, math::Vec3Swizzles,
};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (sprite_auto_movement,sprite_control, sprites_collide,bevy::window::close_on_esc))
        .run();
}


///Create enum for distinguishing between sprites
#[derive(Component, PartialEq)]
enum SpriteType {
    Player, 
    Enemy,
    Background,
    Projectile,
}

//creats a struct for the player
#[derive(Component, PartialEq)]
enum Direction{
   Left,
   Right,
   Up,
   Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Adding the background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            // This sets the background image size. Not dynamic. Needs to be changed to adjust to viewport size
            custom_size: Some(Vec2::new(1920.0, 1080.0)),
            ..default()
        },
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
            texture: asset_server.load("sprites/rusted-avenger.png"),
            transform: Transform::from_xyz(-200.0, -200., 0.),
            ..default()
        },Direction::Left
    )).insert(SpriteType::Player);

    //player fireball
    commands.spawn((
        SpriteBundle{
            sprite: Sprite {
                custom_size: Some(Vec2::new(50.0,50.0)),
                ..default()
        },
        texture: asset_server.load("objects/rusty-fireball.png"),
        transform: Transform::from_xyz(-200.0, -200., 0.),
        visibility: Visibility::Visible,
        ..default()
    })).insert(SpriteType::Projectile);

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
/// //disabling function for time being
fn sprite_auto_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform, &SpriteType)>) {
    for (mut sprite, mut transform, sprite_type) in &mut sprite_position {
        //checks firs for the enemy enum
        if *sprite_type == SpriteType::Enemy {
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
        }
        
        //println!("{:?}", transform);

    
    }
    
}

///Function used for passing user inputs to contrl sprite(s)
fn sprite_control(mut sprite_position: Query<(&mut Transform, &SpriteType, &mut Direction, &mut Visibility)>, keyboard_input: Res<Input<KeyCode>>, mut windows: Query<&mut Window>){
    //Adding logic for detecting window size (probably won't live here long term)
    let window = windows.single_mut();
    let window_width = window.width()/2.0;
    let window_height = window.height()/2.0;
   
    

    for (mut transform, sprite_type, mut direction, mut visibility) in &mut sprite_position {
        //Define Player Movement
        if sprite_type == &SpriteType::Player {
            
            if keyboard_input.pressed(KeyCode::Left) {
                if *direction == Direction::Left{
                    transform.rotate_y(3.14159);
                    *direction = Direction::Right;
                }
                transform.translation.x -= 10.0;
                if transform.translation.x < -window_width {
                    transform.translation.x = window_width;
                }
            }
            if keyboard_input.pressed(KeyCode::Right) {    
                if *direction == Direction::Right{
                    transform.rotate_y(3.14159);
                    *direction = Direction::Left;
                }

                transform.translation.x += 10.0;
                if transform.translation.x > window_width {
                    transform.translation.x = -window_width;
                }
            }
            if keyboard_input.pressed(KeyCode::Down) {
                transform.translation.y -= 10.0;
                if transform.translation.y < -window_height{
                    transform.translation.y = window_height;
                }
            }
            if keyboard_input.pressed(KeyCode::Up) {
                transform.translation.y += 10.0;
                if transform.translation.y > window_height {
                    transform.translation.y = -window_height;
                }
            }
        }
        //this is not working yet. Need to figure out issue
        if keyboard_input.pressed(KeyCode::Space) && sprite_type == &SpriteType::Projectile {
                println!("Detected we are a projectile");
                transform.translation.x -= 10.0;
                transform.translation.y += 200.0;
        }
    }

}


// First version of simple function for detecting collisions, under construction.
// This is really messy and I"m not sure I fully understand why its working
fn sprites_collide(mut sprite_position: Query<(&mut Transform, &SpriteType, &Sprite)>,mut windows: Query<&mut Window>) {
    let mut player_position: Option<Transform> = None;
    let mut enemy_position: Option<Transform> = None;
    let mut player_size: Option<Vec2> = None;
    let mut enemy_size: Option<Vec2> = None;

    for (transform, sprite_type, sprite) in &mut sprite_position.iter() {
        let size = sprite.custom_size.unwrap_or_else(|| sprite.rect.as_ref().map_or(Vec2::new(0.0,0.0), |r| r.size()));
        match *sprite_type {
            SpriteType::Player => {
                player_position = Some(*transform);
                player_size = Some(size);
            }
            SpriteType::Enemy => {
                enemy_position = Some(*transform);
                enemy_size = Some(size);
            }
            _ => {}
        }
    }

    if let (Some(player_pos), Some(player_s), Some(enemy_pos), Some(enemy_s)) = (player_position, player_size, enemy_position, enemy_size){
        let player_min = player_pos.translation.truncate() - player_s / 3.0;
        let player_max = player_pos.translation.truncate() + player_s / 3.0;

        let enemy_min = enemy_pos.translation.truncate() - enemy_s / 3.0;
        let enemy_max = enemy_pos.translation.truncate() + enemy_s / 3.0;

        
        if aabb_collision(player_min, player_max, enemy_min, enemy_max) {
            for (mut transform, sprite_type, _) in sprite_position.iter_mut(){
                if *sprite_type == SpriteType::Player {
                    
                    //using this two places. This needs to be made into a function I can call
                    let window = windows.single_mut();
                    let window_width = window.width()/2.0;
                    let window_height = window.height()/2.0;
                        
                    transform.translation = Vec3::new(window_width, window_height, transform.translation.z);
                }
            }
        }
    }
}

// aabb_collision function (probably bevy has something better for collision)
fn aabb_collision(
    min_a: Vec2, max_a: Vec2,
    min_b: Vec2, max_b: Vec2
) -> bool {
    if min_a.x > max_b.x || max_a.x < min_b.x {
        return false;
    } if min_a.y > max_b.y || max_a.y < min_b.y {
        return false;
    }
    true
}
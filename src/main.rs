//Import the necessary bevy features
use bevy::{
    prelude::*, 
    audio,
    sprite::collide_aabb::collide,
};
use rand::prelude::*;
use std::time::Duration;

//Constants for the game
const SPRITE_SIZE: Vec2 = Vec2::new(100.0,100.0);
const PROJECTILE_SIZE: Vec2 = Vec2::new(25.0, 25.0);
//Charging projectil increases size
const PROJECTILE_CHARGED_SIZE: Vec2 = Vec2::new(200.0, 200.0);
//  Movement Speed might change depending on game values
const MOVEMENT_SPEED: f32 = 1.0;
//  Projectile speed might change depending on game feedback
const PROJECTILE_SPEED: f32 = 1500.0;
//


// Game Structures
#[derive(Component)]
struct ChargeCounter {
    time: Timer,
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
#[derive(Component, Clone, Copy, PartialEq)]
enum Direction{
   Left,
   Right,
   Up,
   Down,
}

// Attributes structure for all sprites to simplify interacting with them.
/*
    TODO: Refactor Code to impelement this struct!
*/
#[derive(Component, PartialEq)]
struct SpriteAttributes {
    id: u8,
    sprite_type: SpriteType,
    direction: Direction,
    movement_speed: f32,
    size: Vec2,
    visible: bool,    
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (sprite_auto_movement,sprite_control, sprites_collide, bevy::window::close_on_esc),)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn(Camera2dBundle::default());
    
    // Adding the background
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            // This sets the background image size. Not dynamic. Needs to be changed to adjust to viewport size
            custom_size: Some(Vec2::new(100.0,100.0)),
            ..default()
        },
        texture: asset_server.load("backgrounds/home-screen-background-small.png"),
        transform: Transform::from_xyz(0., 0., -10.), // Ensure Z-coordinate is behind other entities
        ..default()}, Direction::Up
    )).insert(SpriteAttributes{
        id:0,
        sprite_type: SpriteType::Background,
        direction: Direction::Up,
        movement_speed: 0.0,
        size: SPRITE_SIZE,
        visible: true,
    });
    //player sprite
    commands.spawn((
        SpriteBundle {
            //Adjust Sprite Size
            sprite: Sprite {
                custom_size: Some(SPRITE_SIZE),
                ..default()
            },
            texture: asset_server.load("sprites/rusted-avenger.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },Direction::Left
    )).insert(SpriteAttributes{
        id: 1,
        sprite_type: SpriteType::Player,
        direction: Direction::Left,
        movement_speed: MOVEMENT_SPEED,
        size: SPRITE_SIZE,
        visible: true,
    });

    // Spawn projectiles
    // commands
    // .spawn(SpriteBundle {
    //     sprite: Sprite {
    //         custom_size: Some(PROJECTILE_SIZE),
    //         ..Default::default()
    //     },
    //     texture: asset_server.load("objects/rusty-fireball.png"),
    //     ..Default::default()
    // })
    // .insert( SpriteAttributes {
    //     id: 2, 
    //     sprite_type: SpriteType::Projectile,
    //     direction: Direction::Left,
    //     movement_speed: PROJECTILE_SPEED,
    //     size: PROJECTILE_SIZE,
    //     visible: false,
    // });
    
    //enemy sprite
    commands.spawn((
        SpriteBundle {
            //Adjust Sprite Size
            sprite: Sprite {
                custom_size: Some(SPRITE_SIZE),
                ..default()
            },
            texture: asset_server.load("sprites/enemy-sprite-1.png"),
            transform: Transform::from_xyz(200., 200., 0.),
            ..default()
        },
    )).insert( SpriteAttributes{
        id: 3, 
        sprite_type: SpriteType::Enemy,
        direction: Direction::Right,
        movement_speed: MOVEMENT_SPEED,
        size: SPRITE_SIZE,
        visible: true,
    });

    commands.spawn(AudioBundle {
        source: asset_server.load("sound/background-track-1.ogg"),
        settings: PlaybackSettings { mode: (audio::PlaybackMode::Loop), volume: (audio::Volume::new_relative(1.0)), speed: (1.0), paused: (false) },
        ..default()
    });
}


/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
/// //disabling function for time being
fn sprite_auto_movement(
    time: Res<Time>, 
    mut sprite_info: Query<(&mut Transform, &mut SpriteAttributes)>
    ) {
    let mut rng = rand::thread_rng();
    for (mut location, mut sprite_attributes) in &mut sprite_info {
        //checks first for the enemy enum
        if sprite_attributes.sprite_type == SpriteType::Enemy {
            //randomly change direction
            match rng.gen_range(0..4) {
                0 => {location.translation.y += 250. * time.delta_seconds();},
                1 => {location.translation.y -= 250. * time.delta_seconds()},
                2 => {location.translation.x += 250. * time.delta_seconds()},
                _ => {location.translation.x -= 250. * time.delta_seconds()},
            }


            //Used to guide sprite in a square pattern based on direction
            match sprite_attributes.direction {
                Direction::Up => location.translation.y += 150. * time.delta_seconds() + MOVEMENT_SPEED,
                Direction::Down => location.translation.y -= 150. * time.delta_seconds() + MOVEMENT_SPEED,
                Direction::Right => location.translation.x += 150. * time.delta_seconds() + MOVEMENT_SPEED,
                Direction::Left => location.translation.x -= 150. * time.delta_seconds() + MOVEMENT_SPEED,
            }
    
            // Check the boundaries and update the direction
            match sprite_attributes.direction {
                Direction::Up if location.translation.y >= 200. => sprite_attributes.direction = Direction::Right,
                Direction::Right if location.translation.x >= 200. => sprite_attributes.direction = Direction::Down,
                Direction::Down if location.translation.y <= -200. => sprite_attributes.direction = Direction::Left,
                Direction::Left if location.translation.x <= -200. => sprite_attributes.direction = Direction::Up,
                _ => {}
            }
        }
    }
}

///Function used for passing user inputs to contrl sprite(s)
fn sprite_control(
    mut commands: Commands,
    mut sprite_info: Query<(Entity, &mut Transform, &mut SpriteAttributes)>, keyboard_input: Res<Input<KeyCode>>, 
    mut windows: Query<&mut Window>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
){
    // get window dimensions
    let (window_width, window_height) = window_dimensions(&mut windows);

    //Start our counter if the space key has been pressed
    // let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
    // let mut projectile_size = PROJECTILE_SIZE;
    // if keyboard_input.pressed(KeyCode::Space) {
    //     if timer.finished() == true {
    //         projectile_size = PROJECTILE_CHARGED_SIZE;
    //     }
    // } else {
    //     timer.reset();
    //     projectile_size = PROJECTILE_SIZE;
    // }


    let mut player_position = Vec3::default();
    let mut player_direction = Direction::Left;

    for (entity, mut location, mut sprite_attributes) in sprite_info.iter_mut() {
        if sprite_attributes.sprite_type == SpriteType::Projectile {
            match sprite_attributes.direction {
                Direction::Right => {
                    location.translation.x += sprite_attributes.movement_speed * time.delta_seconds(); // Move to the right
                    if location.translation.x > window_width {
                        commands.entity(entity).despawn();
                    }
                },
                Direction::Left => {
                    location.translation.x -= sprite_attributes.movement_speed * time.delta_seconds(); // Move to the left
                    if location.translation.x < -window_width {
                        commands.entity(entity).despawn();
                    }
                }
                Direction::Up => location.translation.y += sprite_attributes.movement_speed * time.delta_seconds(),
                Direction::Down => location.translation.y -= sprite_attributes.movement_speed * time.delta_seconds(),
            }
        }
        if sprite_attributes.sprite_type == SpriteType::Player {
            player_position = location.translation;
            player_direction = sprite_attributes.direction;

            if keyboard_input.pressed(KeyCode::Left) {
                if sprite_attributes.direction == Direction::Left{
                    location.rotate_y(3.14159);
                    sprite_attributes.direction = Direction::Right;
                }
                location.translation.x -= sprite_attributes.movement_speed;
                if location.translation.x < -window_width {
                    location.translation.x = window_width;
                }
            }
            if keyboard_input.pressed(KeyCode::Right) {    
                if sprite_attributes.direction == Direction::Right{
                    location.rotate_y(3.14159);
                    sprite_attributes.direction = Direction::Left;
                }

                location.translation.x += sprite_attributes.movement_speed;
                if location.translation.x > window_width {
                    location.translation.x = -window_width;
                }
            }
            if keyboard_input.pressed(KeyCode::Down) {
                location.translation.y -= sprite_attributes.movement_speed;
                if location.translation.y < -window_height{
                    location.translation.y = window_height;
                }
            }
            if keyboard_input.pressed(KeyCode::Up) {
                location.translation.y += sprite_attributes.movement_speed;
                if location.translation.y > window_height {
                    location.translation.y = -window_height;
                }
            }

            //Need to redo this for charge counter. As of now it does not count time. Current logic is unreachable state.
            if keyboard_input.just_released(KeyCode::Space) {
                    fire_projectile(&mut commands, &asset_server, *location, player_direction, PROJECTILE_SIZE);
                } 


            
        }  
        if sprite_attributes.sprite_type == SpriteType::Background {
            // Calculate the scale factors, not sure why its 50.0
            let sprite_original_width = 50.0; // adjust this based on your sprite's original size / 2 for some reason
            let sprite_original_height = 50.0; // adjust this based on your sprite's original size / 2 for some reason
    
            let scale_x = window_width / sprite_original_width;
            let scale_y = window_height / sprite_original_height;
    
            // Set the scale of the Transform component
            location.scale = Vec3::new(scale_x, scale_y, 1.0);
        }
    }


}




// First version of simple function for detecting collisions, under construction.
// This is really messy and I"m not sure I fully understand why its working
fn sprites_collide(
    mut commands: Commands,
    mut sprite_position: Query<(Entity, &mut Transform, &SpriteType, &Sprite)>,
    mut windows: Query<&mut Window>,
) {
    // Here we instantiate positions and sizes. I think this should be a Struct
    let mut player_position: Option<Transform> = None;
    let mut enemy_position: Option<Transform> = None;
    let mut projectile_position: Option<Transform> = None;
    let mut player_size: Option<Vec2> = None;
    let mut enemy_size: Option<Vec2> = None;
    let mut projectile_size: Option<Vec2> = None;

    for (_ , transform, sprite_type, sprite) in &mut sprite_position.iter() {
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
            SpriteType::Projectile => {
                projectile_position = Some(*transform);
                projectile_size = Some(size);
            }
            _ => {}
        }
    }

    if let (Some(player_pos), Some(player_s), Some(enemy_pos), Some(enemy_s)) = (player_position, player_size, enemy_position, enemy_size){
        let collision = collide(player_pos.translation, player_s / 1.5, enemy_pos.translation, enemy_s / 1.5);
        if collision.is_some() {
            for (entity, mut transform, sprite_type, _) in sprite_position.iter_mut(){
                match *sprite_type {
                    SpriteType::Projectile => {
                        commands.entity(entity).despawn();
                    },
                    SpriteType::Player => {
                        let window = windows.single_mut();
                        let window_width = window.width()/2.0;
                        let window_height = window.height()/2.0;
                        transform.translation = Vec3::new(window_width, window_height, transform.translation.z);
                    },
                    SpriteType::Enemy => {
                        let window = windows.single_mut();
                        let window_width = window.width()/2.0;
                        let window_height = window.height()/2.0;
                        transform.translation = Vec3::new(-window_width, -window_height, transform.translation.z);
                    },
                    _ => {}
                }
            }
        }
    }

    if let (Some(projectile_pos), Some(projectile_s), Some(enemy_pos), Some(enemy_s)) = (projectile_position, projectile_size, enemy_position, enemy_size){
        let collision = collide(projectile_pos.translation, projectile_s, enemy_pos.translation, enemy_s);
        if collision.is_some() {
            for (entity, mut transform, sprite_type, _) in sprite_position.iter_mut(){
                if *sprite_type == SpriteType::Projectile {
                    commands.entity(entity).despawn();
                }
                if *sprite_type == SpriteType::Enemy {
                    let window = windows.single_mut();
                    let window_width = window.width()/2.0;
                    let window_height = window.height()/2.0;
                    transform.translation = Vec3::new(-window_width+100.0, -window_height+100.0, transform.translation.z);
                }
            }
        }
    }
}



//This function gets our window info (x,y dimensions)
fn window_dimensions(windows: &mut Query<&mut Window>) -> (f32,f32) {
    let window = windows.single_mut();
    (window.width()/2.0, window.height()/2.0)
}

//function for firing the projectile
fn fire_projectile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    player_position: Transform,
    player_direction: Direction,
    projectile_size: Vec2,
) {
    let mut x_adjustment = 0.0;
    let mut projectile_direction = Direction::Up;
    if player_direction == Direction::Left {
        x_adjustment = player_position.translation.x + 75.0;
        projectile_direction = Direction::Right;
    } else {
        x_adjustment = player_position.translation.x - 75.0;
        projectile_direction = Direction::Left;
    }
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(projectile_size),
                ..Default::default()
            },
            texture: asset_server.load("objects/rusty-fireball.png"),
            
            transform: Transform::from_xyz(x_adjustment, player_position.translation.y + 20.0, 5.0),
            ..Default::default()
        })
        .insert(projectile_direction)
        .insert(SpriteType::Projectile);
}
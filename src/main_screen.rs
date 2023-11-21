use bevy::{prelude::*, winit::WinitSettings};

// Button Taken from the bevy example
pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub fn button_system(
    mut interaction_query: Query<
        (
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
        &Children,
        ),
        (Changed<Interaction>, With<Button>),
        >,
    mut text_query: Query<&mut Text>,
    mut commands: Commands,
    mut query: Query<Entity, With<Button>>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
           Interaction::Hovered => {
                text.sections[0].value = "Start the Adventure".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            },
            Interaction::Pressed => {
                delete_button(&mut commands, &mut query);
            }
           _ => {
                text.sections[0].value = "START".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn setup_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(130.0),
                        border: UiRect::all(Val::Px(5.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            //font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
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

fn delete_button(commands: &mut Commands, query: &mut Query<Entity, With<Button>>) {
    for entity in query.iter() {
        commands.entity(entity).remove::<Button>();
    }
}
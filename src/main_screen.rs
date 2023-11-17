use bevy::{prelude::*, winit::WinitSettings};



// Taken from the bevy example
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
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
           Interaction::Hovered => {
                text.sections[0].value = "Start the Adventure".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
           _ => {
                text.sections[0].value = "START".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
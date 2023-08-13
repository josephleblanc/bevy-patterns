#![allow(dead_code, unreachable_code, unused_variables, unused_imports)]
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

#[derive(Event)]
pub struct ClickButtonEvent {
    big_button: Option<BigButtonState>,
    little_button: Option<LittleButtonState>,
    button_entity: Entity,
    /* ... more buttons */
}

pub const BIG_BUTTON_CLICKED: Color = Color::BLUE;
pub const LITTLE_BUTTON_CLICKED: Color = Color::RED;

fn change_button_color(
    mut query: Query<(&mut BackgroundColor, Entity), With<Button>>,
    mut event_reader: EventReader<ClickButtonEvent>,
) {
    if let Some(event) = event_reader.iter().last() {
        if let Ok((mut bg_color, _)) = query.get_mut(event.button_entity) {
            if let Some(big_button_state) = event.big_button {
                *bg_color = BackgroundColor(BIG_BUTTON_CLICKED);
            }
            if let Some(little_button_state) = event.little_button {
                *bg_color = BackgroundColor(LITTLE_BUTTON_CLICKED);
            }
        }
    }
}

#[derive(Component, Copy, Clone)]
pub struct LittleButtonState(pub bool);
#[derive(Component, Copy, Clone)]
pub struct BigButtonState(pub bool);

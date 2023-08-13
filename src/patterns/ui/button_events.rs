use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct PreSelectButton {
    entity: Entity,
}

impl PreSelectButton {
    pub fn send_event(
        mut preselect_event_wtr: EventWriter<PreSelectButton>,
        query: Query<(Entity, &Button, &Interaction), Changed<Interaction>>,
    ) {
        for (entity, _button, interaction) in query.iter() {
            if *interaction == Interaction::Pressed {
                preselect_event_wtr.send(Self { entity });
                info!("sending event");
            }
        }
    }
    pub fn read_events(
        mut preselect_event_rdr: EventReader<PreSelectButton>,
        mut button_event_rdr: EventReader<MouseButtonInput>,
        query: Query<(Entity, &Button, &Interaction), Changed<Interaction>>,
    ) {
        if button_event_rdr
            .iter()
            .filter(|button_event| button_event.button == MouseButton::Right)
            .next()
            .is_some()
        {
            info!("looking for new events");
            for button_event in preselect_event_rdr.iter() {
                info!("button event read: {:#?}", button_event);
            }
        }
    }
}

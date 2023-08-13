#![allow(dead_code, unreachable_code, unused_variables, unused_imports)]
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

mod patterns;
use patterns::ui::button_events::PreSelectButton;
use patterns::ui::childbuilder_methods;
use patterns::ui::closure_checkbox;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Test TextBundle".into(),
                    // mode: bevy::window::WindowMode::BorderlessFullscreen,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        ))
        .add_systems(
            Startup,
            (
                spawn_camera,
                childbuilder_methods::spawn_layout.after(spawn_camera),
            ),
        )
        .add_event::<PreSelectButton>()
        .add_systems(
            Update,
            (
                PreSelectButton::send_event,
                PreSelectButton::read_events.after(PreSelectButton::send_event),
            ),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

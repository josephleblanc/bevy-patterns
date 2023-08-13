#![allow(dead_code)]
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

pub fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("OpenSans-Italic.ttf");

    let text = "Button 1";
    let text1 = "Button 2";

    let button_color_normal = Color::rgb(0.2, 0.2, 0.2);

    // Top-level grid (app frame)
    let frame_id = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            ..default()
        })
        .id();
    let menu_column = commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                column_gap: Val::Percent(3.),
                width: Val::Percent(20.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Start,
                ..default()
            },
            background_color: Color::rgb(0.3, 0.3, 0.3).into(),
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn_nested_button_bundle(button_color_normal)
                .with_children(|builder| {
                    builder.spawn_nested_text_bundle(text, font.clone());
                });
            builder
                .spawn_nested_button_bundle(button_color_normal)
                .with_children(|builder| {
                    builder.spawn_nested_text_bundle(text1, font.clone());
                });
        });
}

pub trait IntoChildBuilder<'w, 's, 'a, 'b>: Sized {
    fn to_childbuilder(self) -> &'a mut ChildBuilder<'w, 's, 'b>;
}

impl<'w, 's, 'a, 'b> IntoChildBuilder<'w, 's, 'a, 'b> for &'a mut ChildBuilder<'w, 's, 'b> {
    fn to_childbuilder(self) -> &'a mut ChildBuilder<'w, 's, 'b> {
        self
    }
}
pub trait SpawnTextBundle<'w, 's, 'a, 'b>
where
    Self: IntoChildBuilder<'w, 's, 'a, 'b>,
    'w: 'a + 'b,
    's: 'a + 'b,
    'b: 'a,
{
    fn spawn_nested_text_bundle(
        self,
        text: &str,
        font: Handle<Font>,
    ) -> EntityCommands<'w, 's, 'a> {
        let builder: &mut ChildBuilder<'w, 's, 'b> = self.to_childbuilder();
        builder.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font,
                font_size: 48.0,
                color: Color::WHITE,
            },
        ))
    }
}
impl<'w, 's, 'a, 'b> SpawnTextBundle<'w, 's, 'a, 'b> for &'a mut ChildBuilder<'w, 's, 'b> {}

pub trait SpawnButtonBundle<'w, 's, 'a, 'b>
where
    Self: IntoChildBuilder<'w, 's, 'a, 'b>,
    'w: 'a + 'b,
    's: 'a + 'b,
    'b: 'a,
{
    fn spawn_nested_button_bundle(self, button_color: Color) -> EntityCommands<'w, 's, 'a> {
        let builder: &'a mut ChildBuilder<'w, 's, 'b> = self.to_childbuilder();
        builder.spawn(ButtonBundle {
            background_color: button_color.into(),
            ..default()
        })
    }
}
impl<'w, 's, 'a, 'b> SpawnButtonBundle<'w, 's, 'a, 'b> for &'a mut ChildBuilder<'w, 's, 'b> {}

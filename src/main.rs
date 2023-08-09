use bevy::prelude::*;
use plugins::{player::player::PlayerPlugin, world_generation::world_plugin::WorldPlugin};



mod plugins;

fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pokemon Multiplayer".into(),
                resolution: (854.0, 480.0).into(),
                resizable: true,
                ..default()
                }),
                ..default()
        })
        .build(),
    )
        .add_plugins((
            PlayerPlugin,
            WorldPlugin,
        )) // prevents blurry sprites
        .run();
}


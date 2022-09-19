mod plugin;
mod settings;
mod map;
use plugin::{texture_loader, input, camera};

use bevy::{
    prelude::{
        App,
    },
    window::WindowDescriptor,
    DefaultPlugins, log::LogSettings,
};
use bevy::prelude::ClearColor;

fn main() {
    App::new()
        .insert_resource(LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy-test=debug".into(),
            level: bevy::log::Level::DEBUG,
        })
        .insert_resource(ClearColor(settings::CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            width: settings::RESOLUTION.x,
            height: settings::RESOLUTION.y,
            title: "Bevy Test".to_string(),
            resizable: false,
            cursor_visible: true,
            cursor_locked: false,
            ..Default::default()
        })
        .add_plugin(texture_loader::TextureLoaderPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(map::MapTestPlugin)
        .add_state(settings::GameState::Game)
        .add_plugins(DefaultPlugins)
        .run();
}

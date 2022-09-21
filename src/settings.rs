use bevy::prelude::{Color, Plugin, Vec2};

pub const CLEAR_COLOR: Color = Color::rgb(0.75, 0.75, 0.75);
pub const RESOLUTION: Vec2 = Vec2::new(960.0 * 16.0 / 9.0, 960.0);
pub const RESOLUTION_HALF: Vec2 = Vec2::new(RESOLUTION.x / 2.0, RESOLUTION.y / 2.0);

pub const CAMERA_SPEED: f32 = 1.0;

pub const DEFAULT_CHUNK_SIZE: u32 = 16;
pub const DEFAULT_MAP_SIZE: u32 = 16;

//pub const DEFAULT_TILE_SIZE: f32 = 16.0;

#[derive(Debug, Clone, PartialEq, Hash, Copy, Eq)]
pub enum GameState {
    //StartMenu,
    Game,
    //PauseMenu
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(bevy::DefaultPlugins)
            .insert_resource(bevy::prelude::ClearColor(CLEAR_COLOR))
            .insert_resource(bevy::log::LogSettings {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy-test=debug".into(),
                level: bevy::log::Level::DEBUG,
            })
            .insert_resource(bevy::window::WindowDescriptor {
                width: RESOLUTION.x,
                height: RESOLUTION.y,
                title: "Bevy Test".to_string(),
                resizable: false,
                cursor_visible: true,
                cursor_locked: false,
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..Default::default()
            });
    }
}

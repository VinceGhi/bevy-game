use bevy::prelude::{Color, Vec2};

pub const CLEAR_COLOR: Color = Color::rgb(0.75, 0.75, 0.75);
pub const RESOLUTION: Vec2 = Vec2::new(960.0 * 16.0 / 9.0, 960.0);
pub const RESOLUTION_HALF: Vec2 = Vec2::new(RESOLUTION.x / 2.0, RESOLUTION.y / 2.0);

pub const CAMERA_SPEED: f32 = 256.0;

//pub const DEFAULT_TILE_SIZE: f32 = 16.0;

#[derive(Debug, Clone, PartialEq, Hash, Copy, Eq)]
pub enum GameState {
    //StartMenu,
    Game,
    //PauseMenu
}

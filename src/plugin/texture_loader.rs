use bevy::{prelude::{Plugin, App, StartupStage, Handle, Commands, AssetServer, Res, Vec2, ResMut, Assets}, sprite::TextureAtlas};

use crate::DEBUG_LOGGER;

pub const TILE_SIZE: f32 = 16.0;

/*
    TEXTURE STUFF
*/
pub struct TextureLoader;

impl Plugin for TextureLoader {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_terrain);
    }
}

// ASCII
pub struct AsciiTileset(pub Handle<TextureAtlas>, pub Vec2);

pub enum AsciiTilsetId {
    Zero = 48, One = 49, Two = 50, Three = 51, Four = 52, Five = 53, Six = 54, Seven = 55, Eight = 56, Nine = 57, Ten = 58,
}


fn load_ascii(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let image = assets.load("ascii.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(64.0), 16, 16);
    let atlas_handle = texture_atlases.add(atlas);

    DEBUG_LOGGER.log("Loading ascii.png".to_string());
    commands.insert_resource(AsciiTileset(atlas_handle, Vec2::splat(64.0)));
}

// TERRAIN
pub struct TerrainTileset(pub Handle<TextureAtlas>, pub Vec2);

pub enum TerrainTilsetId {
    DarkGreen = 0,
    Green = 1,
    LightGreen = 3,
    DarkBlue = 10,
    Blue = 11,
    LightBlue = 12
}

fn load_terrain(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let image = assets.load("terrain.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(16.0), 10, 10);
    let atlas_handle = texture_atlases.add(atlas);

    DEBUG_LOGGER.log("Loading terrain.png".to_string());
    commands.insert_resource(TerrainTileset(atlas_handle, Vec2::splat(16.0)));
}
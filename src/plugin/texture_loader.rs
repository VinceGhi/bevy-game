use bevy::{
    prelude::{
        App, AssetServer, Assets, Commands, Handle, Plugin, Res, ResMut, StartupStage, Vec2,
    },
    render::texture::ImageSettings,
    sprite::TextureAtlas,
};

/*
    TEXTURE STUFF
*/
pub struct TextureLoaderPlugin;

impl Plugin for TextureLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageSettings::default_nearest())
            .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_terrain);
    }
}

// ASCII
pub struct AsciiTileset(pub Handle<TextureAtlas>, pub Vec2);

pub enum AsciiTilsetId {
    Zero = 48,
    //One = 49,
    //Two = 50,
    //Three = 51,
    //Four = 52,
    //Five = 53,
    //Six = 54,
    //Seven = 55,
    //Eight = 56,
    //Nine = 57,
    //Ten = 58,
}

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("ascii.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(64.0), 16, 16);
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(AsciiTileset(atlas_handle, Vec2::splat(64.0)));
}

// TERRAIN
pub struct TerrainTileset(pub Handle<TextureAtlas>, pub Vec2);

//pub enum TerrainTilsetId {
//    DarkGreen = 0,
//    Green = 1,
//    LightGreen = 3,
//    DarkBlue = 10,
//    Blue = 11,
//    LightBlue = 12,
//}

fn load_terrain(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let image = assets.load("terrain.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(16.0), 10, 10);
    let atlas_handle = texture_atlases.add(atlas);
    commands.insert_resource(TerrainTileset(atlas_handle, Vec2::splat(16.0)));
}

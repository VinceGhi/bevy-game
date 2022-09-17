mod logger;

use bevy::{
    prelude::{
        App, Color, ClearColor, Commands, Camera2dBundle, AssetServer, ResMut, Assets, Res, Vec2, Handle, StartupStage, Transform, Name
    },
    DefaultPlugins, window::WindowDescriptor, sprite::{TextureAtlas, TextureAtlasSprite, SpriteSheetBundle}, render::camera::ScalingMode
};
use logger::{Logger, LogLevel};

pub const CLEAR_COLOR: Color = Color::rgb(0.75, 0.75, 0.75);
pub const RESOLUTION_SCALE: f32 = 16.0 / 9.0;
pub const RESOLUTION: f32 = 720.0;
pub const DEBUG_LOGGER: Logger = Logger::new(LogLevel::Debug, true);

fn main() {
    DEBUG_LOGGER.log("Creating and running App".to_string());
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .insert_resource(WindowDescriptor {
            width: RESOLUTION * RESOLUTION_SCALE,
            height: RESOLUTION,
            title: "Bevy Test".to_string(),
            resizable: false,
            cursor_visible: true,
            cursor_locked: false,
            ..Default::default()
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii_tileset)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 1.0;
    camera.projection.bottom = -1.0;
    camera.projection.right = 1.0 * RESOLUTION;
    camera.projection.left = -1.0 * RESOLUTION;
    camera.projection.scaling_mode = ScalingMode::None;
    camera.transform = Transform::from_xyz(0.0, 0.0, 1000.0);

    let entity_camera = commands.spawn_bundle(camera).id();
    DEBUG_LOGGER.log(format!("Spawned camera [id:{}]", entity_camera.id()));
}

fn spawn_player(mut commands: Commands, ascii_tilset: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.75, 0.0, 0.0);
    sprite.custom_size = Some(Vec2::splat(1.0));

    let sprite_sheet_bundle = SpriteSheetBundle {
        sprite,
        texture_atlas: ascii_tilset.0.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    };

    let entity_player = commands.spawn_bundle(sprite_sheet_bundle).insert(Name::new("Player")).id();
    DEBUG_LOGGER.log(format!("Spawned player [id:{}]", entity_player.id()));
}

fn load_ascii_tileset(mut commands: Commands, assets: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let image = assets.load("ascii_tileset.png");
    let atlas = TextureAtlas::from_grid(image, Vec2::splat(64.0), 16, 16);
    let atlas_handle = texture_atlases.add(atlas);

    DEBUG_LOGGER.log("Loading ascii_lowres.png".to_string());
    commands.insert_resource(AsciiSheet(atlas_handle));
}

struct AsciiSheet(Handle<TextureAtlas>);
mod logger;
mod plugin;

use bevy::{
    prelude::{
        App, Color, ClearColor, Commands, Camera2dBundle, OrthographicProjection, EventReader, Component, Query, With, KeyCode, Transform, Name, Res, Vec2
    },
    DefaultPlugins, window::WindowDescriptor, sprite::{ SpriteSheetBundle, TextureAtlasSprite}, render::camera::{ScalingMode}, input::{mouse::MouseWheel, keyboard::{KeyboardInput}}
};
use logger::{Logger, LogLevel};
use plugin::texture_loader;

pub const CLEAR_COLOR: Color = Color::rgb(0.75, 0.75, 0.75);
pub const RESOLUTION_SCALE: f32 = 16.0 / 9.0;
pub const RESOLUTION: f32 = 720.0;
pub const DEBUG_LOGGER: Logger = Logger::new(LogLevel::Debug, cfg!(debug_assertions));

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
        .add_plugin(texture_loader::TextureLoader)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_system(scroll_events)
        .add_system(keyboard_events)
        .add_plugins(DefaultPlugins)
        .run();
}

/*
    SPAWNING STUFF
*/
#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();

    camera.projection.top = 1.0 * RESOLUTION / 2.;
    camera.projection.bottom = -1.0 * RESOLUTION / 2.;
    camera.projection.right = 1.0 * RESOLUTION * RESOLUTION_SCALE / 2.;
    camera.projection.left = -1.0 * RESOLUTION * RESOLUTION_SCALE / 2.;
    camera.projection.scaling_mode = ScalingMode::None;
    camera.transform = Transform::from_xyz(0.0, 0.0, 1000.0);
    camera.projection.scale = 0.5;

    let entity_camera = commands
        .spawn_bundle(camera)
        .insert(MainCamera)
        .insert(Name::new("Camera"))
        .id();
    DEBUG_LOGGER.log(format!("Spawned camera [id:{}]", entity_camera.id()));
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, tilset: Res<texture_loader::TerrainTileset>) {
    let mut sprite = TextureAtlasSprite::new(texture_loader::TerrainTilsetId::Blue as usize);
    sprite.color = Color::rgb(0.75, 0.0, 0.0);
    sprite.custom_size = Some(Vec2::splat(texture_loader::TILE_SIZE));

    let sprite_sheet_bundle = SpriteSheetBundle {
        sprite,
        texture_atlas: tilset.0.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    };

    let entity_player = commands.spawn_bundle(sprite_sheet_bundle)
        .insert(Player)
        .insert(Name::new("Player"))
        .id();
    DEBUG_LOGGER.log(format!("Spawned player [id:{}]", entity_player.id()));
}

/*
    INPUT HANDLING STUFF
*/
fn scroll_events(mut scroll_evr: EventReader<MouseWheel>, mut player_query: Query<&mut OrthographicProjection, With<MainCamera>>) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        let mut projection = player_query.single_mut();
        match ev.unit {
            MouseScrollUnit::Line => {
                let zoom_value = ev.y * 0.2;
                DEBUG_LOGGER.log(format!("Zooming with {}", zoom_value));
                projection.scale += zoom_value;
            }
            MouseScrollUnit::Pixel => {/* do nothing atm*/}
        }
    }
}

fn keyboard_events(mut keyboard_events: EventReader<KeyboardInput>, mut transform_query: Query<&mut Transform, With<Player>>) {
    for event in keyboard_events.iter() {
        let mut transform = transform_query.single_mut();

        match event.state {
            bevy::input::ButtonState::Pressed => {
                match event.key_code {
                    Some(key) => {
                        if key == KeyCode::Right {
                            DEBUG_LOGGER.log("Pressed right arrow key".to_string());
                            transform.translation.x += texture_loader::TILE_SIZE;
                        } else if key == KeyCode::Down {
                            DEBUG_LOGGER.log("Pressed down arrow key".to_string());
                            transform.translation.y -= texture_loader::TILE_SIZE;
                        } else if key == KeyCode::Left {
                            DEBUG_LOGGER.log("Pressed left arrow key".to_string());
                            transform.translation.x -= texture_loader::TILE_SIZE;
                        } else if key == KeyCode::Up {
                            DEBUG_LOGGER.log("Pressed up arrow key".to_string());
                            transform.translation.y += texture_loader::TILE_SIZE;
                        }
                    },
                    None => {/* DO NOTHING ATM*/},
                }
            },
            bevy::input::ButtonState::Released => {/* DO NOTHING ATM*/},
        }
        
    }
}
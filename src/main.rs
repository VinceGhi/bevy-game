mod map;
mod plugin;
mod settings;

use plugin::{camera, input, texture_loader};

use bevy::prelude::ClearColor;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::{App, AssetServer, Color, Commands, Component, Query, Res, TextBundle, With},
    text::{Text, TextSection, TextStyle},
    ui::{PositionType, Style, UiRect, Val},
    DefaultPlugins,
};

fn main() {
    App::new()
        // RESOURCES
        .insert_resource(ClearColor(settings::CLEAR_COLOR))
        .insert_resource(bevy::log::LogSettings {
            filter: "info,wgpu_core=warn,wgpu_hal=warn,bevy-test=debug".into(),
            level: bevy::log::Level::DEBUG,
        })
        .insert_resource(bevy::window::WindowDescriptor {
            width: settings::RESOLUTION.x,
            height: settings::RESOLUTION.y,
            title: "Bevy Test".to_string(),
            resizable: false,
            cursor_visible: true,
            cursor_locked: false,
            present_mode: bevy::window::PresentMode::AutoNoVsync,
            ..Default::default()
        })
        // STATE
        .add_state(settings::GameState::Game)
        // PLUGINS
        .add_plugin(texture_loader::TextureLoaderPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(camera::CameraPlugin)
        //.add_plugin(player::PlayerPlugin)
        .add_plugin(map::MapTestPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // STARTUP SYSTEMS
        .add_startup_system(fps_text_setup_system)
        // SYSTEMS
        .add_system(fps_text_update_system)
        .run();
}

fn fps_text_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "\nAverage FPS: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.0, 1.0, 0.0),
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.0, 1.0, 1.0),
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
        .insert(StatsText);
}

#[derive(Component)]
struct StatsText;

fn fps_text_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<StatsText>>,
) {
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            text.sections[1].value = format!("{average:.2}");
        }
    };
}

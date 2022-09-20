mod plugin;
mod settings;
mod map;

use plugin::{texture_loader, input, camera};

use bevy::{
    prelude::{
        App, Commands, Component, TextBundle, Res, AssetServer, Color, With, Query,
    },
    window::{WindowDescriptor, PresentMode},
    DefaultPlugins, log::LogSettings, text::{TextSection, TextStyle, Text}, ui::{Style, PositionType, UiRect, Val}, utils::default, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
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
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .add_plugin(texture_loader::TextureLoaderPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(camera::CameraPlugin)
        //.add_plugin(player::PlayerPlugin)
        .add_plugin(map::MapTestPlugin)
        .add_state(settings::GameState::Game)
        .add_plugins(DefaultPlugins)
        .add_system(fps_stats)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.spawn_bundle(
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

fn fps_stats(
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
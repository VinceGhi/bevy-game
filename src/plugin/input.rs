use super::camera::{self, CameraCrossedChunkBorderEvent, TileMapCameraPosition};
use crate::settings;
use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::{
        EventReader, EventWriter, Input, KeyCode, OrthographicProjection, Plugin, Query, Res,
        SystemSet, Transform, With,
    },
    time::Time,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_update(settings::GameState::Game)
                .with_system(key_input)
                .with_system(scroll_events),
        );
    }
}
fn scroll_events(
    mut scroll_events: EventReader<MouseWheel>,
    mut player_query: Query<
        (
            &mut OrthographicProjection,
            &Transform,
            &mut TileMapCameraPosition,
        ),
        With<camera::MainCamera>,
    >,
    mut event_writer: EventWriter<CameraCrossedChunkBorderEvent>,
) {
    for event in scroll_events.iter() {
        let mut camera = player_query.single_mut();
        let camera_pos = TileMapCameraPosition::new(camera.1.clone(), camera.0.clone(), true);
        camera.2.pos = camera_pos.pos;
        camera.2.view = camera_pos.view;
        if event.unit == MouseScrollUnit::Line {
            camera.0.scale += (camera.0.scale * 1.1 - camera.0.scale) * -event.y;
            event_writer.send(CameraCrossedChunkBorderEvent(
                camera.1.translation,
                camera.0.scale,
                camera_pos,
            ));
            continue;
        }
    }
}

const MOVE_CAMERA_KEYS: [KeyCode; 8] = [
    KeyCode::W,
    KeyCode::A,
    KeyCode::S,
    KeyCode::D,
    KeyCode::Up,
    KeyCode::Left,
    KeyCode::Down,
    KeyCode::Right,
];
fn key_input(
    input: Res<Input<KeyCode>>,
    timer: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &OrthographicProjection,
            &mut camera::TileMapCameraPosition,
        ),
        With<camera::MainCamera>,
    >,
    mut event_writer: EventWriter<CameraCrossedChunkBorderEvent>,
) {
    let mut camera = query.single_mut();
    let mut camera_moved: bool = false;
    let camera_speed = settings::CAMERA_SPEED * timer.delta_seconds() * camera.1.scale * 500.0;
    for pressed in input
        .get_pressed()
        .filter(|key| MOVE_CAMERA_KEYS.contains(key))
    {
        let mut camera_movement = camera.0.translation.clone();

        if pressed == &KeyCode::Right {
            camera_movement.x += camera_speed
        }

        if pressed == &KeyCode::Down {
            camera_movement.y -= camera_speed
        }

        if pressed == &KeyCode::Left {
            camera_movement.x -= camera_speed
        }

        if pressed == &KeyCode::Up {
            camera_movement.y += camera_speed
        }

        camera_moved = true;
        camera.0.translation = camera_movement;
    }

    if camera_moved {
        let new_tilemap_camera_position =
            TileMapCameraPosition::new(camera.0.clone(), camera.1.clone(), false);
        if !new_tilemap_camera_position.eq(camera.2.clone()) {
            camera.2.pos = new_tilemap_camera_position.pos;
            camera.2.view = new_tilemap_camera_position.view;
            event_writer.send(CameraCrossedChunkBorderEvent(
                camera.0.translation,
                camera.1.scale,
                TileMapCameraPosition::new(camera.0.clone(), camera.1.clone(), true),
            ));
        }
    }
}

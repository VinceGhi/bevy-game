use crate::settings;
use bevy::{
    prelude::{
        info, Camera2dBundle, Commands, Component, EventReader, EventWriter, IVec2, Name,
        OrthographicProjection, Plugin, Query, SystemSet, Transform, Vec2, Vec3, Visibility, With,
    },
    render::camera::ScalingMode,
};

use super::map;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CameraCrossedChunkBorderEvent>()
            .add_system_set(
                SystemSet::on_enter(settings::GameState::Game).with_system(spawn_camera),
            )
            .add_system_set(
                SystemSet::on_update(settings::GameState::Game)
                    .with_system(chunk_visibility_check_system),
            );
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(
    mut commands: Commands,
    mut event_writer: EventWriter<CameraCrossedChunkBorderEvent>,
) {
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.top = settings::RESOLUTION_HALF.y;
    camera_bundle.projection.bottom = -settings::RESOLUTION_HALF.y;
    camera_bundle.projection.right = settings::RESOLUTION_HALF.x;
    camera_bundle.projection.left = -settings::RESOLUTION_HALF.x;
    camera_bundle.projection.scaling_mode = ScalingMode::None;
    camera_bundle.projection.scale = 0.5;
    camera_bundle.transform = Transform::from_xyz(0.0, 0.0, 1000.0);

    let camera_position = TileMapCameraPosition::new(
        camera_bundle.transform.clone(),
        camera_bundle.projection.clone(),
        true,
    );

    event_writer.send(CameraCrossedChunkBorderEvent(
        camera_bundle.transform.translation,
        camera_bundle.projection.scale,
        camera_position.clone(),
    ));

    commands
        .spawn_bundle(camera_bundle)
        .insert(MainCamera)
        .insert(camera_position)
        .insert(Name::new("Camera"));
}

pub struct CameraCrossedChunkBorderEvent(pub Vec3, pub f32, pub TileMapCameraPosition);

fn chunk_visibility_check_system(
    mut events: EventReader<CameraCrossedChunkBorderEvent>,
    mut chunk_query: Query<(
        (&mut Visibility, &map::TileMapChunk),
        With<map::TileMapChunk>,
    )>,
) {
    for event in events.iter() {
        let mut vis_chunks = 0;
        chunk_query.for_each_mut(|mut chunk| {
            chunk.0 .0.is_visible = event.2.view.left < chunk.0 .1.pos.x as i32
                && event.2.view.right > chunk.0 .1.pos.x as i32
                && event.2.view.top > chunk.0 .1.pos.y as i32
                && event.2.view.bottom < chunk.0 .1.pos.y as i32;

            if chunk.0 .0.is_visible {
                vis_chunks += 1;
            }
        });

        bevy::prelude::info!(
            "Visible Chunks: {}, Current CameraChunkPos [{}, {}] Bounds: [T:{}, R:{}, B:{}, L:{}]",
            vis_chunks,
            event.2.pos.x,
            event.2.pos.y,
            event.2.view.top,
            event.2.view.right,
            event.2.view.bottom,
            event.2.view.left
        );
    }
}
#[derive(Component, Clone, bevy_inspector_egui::Inspectable)]
pub struct TileMapCameraPosition {
    pub pos: IVec2,
    pub view: Bounds,
}

#[derive(Clone, Copy, bevy_inspector_egui::Inspectable)]
pub struct Bounds {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

impl TileMapCameraPosition {
    pub fn new(
        transform: Transform,
        projection: OrthographicProjection,
        calc_bounds: bool,
    ) -> Self {
        let chunk_size_in_pixel = 16.0 * settings::DEFAULT_CHUNK_SIZE as f32;
        let cam_bound_offset_y =
            (settings::RESOLUTION_HALF.y * projection.scale) / chunk_size_in_pixel;
        let cam_bound_offset_x =
            (settings::RESOLUTION_HALF.x * projection.scale) / chunk_size_in_pixel;

        let cam_pos = IVec2::new(
            (transform.translation.x / chunk_size_in_pixel).floor() as i32,
            (transform.translation.y / chunk_size_in_pixel).floor() as i32,
        );

        let cam_view = if calc_bounds {
            Bounds {
                top: cam_pos.y + cam_bound_offset_y.ceil() as i32 + (projection.scale).ceil() as i32,
                right: cam_pos.x + cam_bound_offset_x.ceil() as i32 + (projection.scale).ceil() as i32,
                bottom: cam_pos.y + (-cam_bound_offset_y).floor() as i32 + (-projection.scale).floor() as i32,
                left: cam_pos.x + (-cam_bound_offset_x).floor() as i32 + (-projection.scale).floor() as i32,
            }
        } else {
            Bounds {
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,
            }
        };

        Self {
            pos: IVec2::new(
                (transform.translation.x / chunk_size_in_pixel).floor() as i32,
                (transform.translation.y / chunk_size_in_pixel).floor() as i32,
            ),
            view: cam_view,
        }
    }

    pub fn eq(&self, other: TileMapCameraPosition) -> bool {
        return self.pos.x == other.pos.x && self.pos.y == other.pos.y;
    }
}

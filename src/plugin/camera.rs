use crate::{map, settings};
use bevy::{
    prelude::{
        info, Camera2dBundle, Commands, Component, EventReader, EventWriter, Name, Plugin, Query,
        SystemSet, Transform, Vec3, Visibility, With, Without,
    },
    render::camera::ScalingMode,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<CameraMovedEvent>()
            .add_system_set(
                SystemSet::on_enter(settings::GameState::Game).with_system(spawn_camera),
            )
            .add_system_set(
                SystemSet::on_update(settings::GameState::Game).with_system(camera_update_event),
            );
    }
}

#[derive(Component)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands, mut event_writer: EventWriter<CameraMovedEvent>) {
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.top = settings::RESOLUTION_HALF.y;
    camera_bundle.projection.bottom = -settings::RESOLUTION_HALF.y;
    camera_bundle.projection.right = settings::RESOLUTION_HALF.x;
    camera_bundle.projection.left = -settings::RESOLUTION_HALF.x;
    camera_bundle.projection.scaling_mode = ScalingMode::None;
    camera_bundle.projection.scale = 0.5;
    camera_bundle.transform = Transform::from_xyz(0.0, 0.0, 1000.0);

    event_writer.send(CameraMovedEvent(
        camera_bundle.transform.translation,
        camera_bundle.projection.scale,
    ));

    commands
        .spawn_bundle(camera_bundle)
        .insert(MainCamera)
        .insert(Name::new("Camera"));
}

pub struct CameraMovedEvent(pub Vec3, pub f32);

fn camera_update_event(
    mut events: EventReader<CameraMovedEvent>,
    mut tilemap_query: Query<(
        (&mut Visibility, &Transform),
        With<map::MapTile>,
        Without<MainCamera>,
    )>,
) {
    for event in events.iter() {
        let camera_view_border_x = (settings::RESOLUTION_HALF.x + 32.0) * event.1;
        let camera_view_border_y = (settings::RESOLUTION_HALF.y + 32.0) * event.1;
        let mut visible_tile_counter = 0;
        tilemap_query.for_each_mut(|mut tile| {
            let dif_x = (event.0.x - tile.0 .1.translation.x).abs();
            let dif_y = (event.0.y - tile.0 .1.translation.y).abs();
            tile.0 .0.is_visible = dif_x < camera_view_border_x && dif_y < camera_view_border_y;
            if tile.0 .0.is_visible {
                visible_tile_counter += 1;
            }
        });

        info!("{} MapTiles are visible", visible_tile_counter);
    }
}

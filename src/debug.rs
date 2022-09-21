use bevy::prelude::*;
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorPlugin};

use crate::{map, plugin::camera};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new());

            let mut registry = app
                .world
                .get_resource_or_insert_with(InspectableRegistry::default);

            registry.register::<map::TileMap>();
            registry.register::<map::TileMapChunk>();
            registry.register::<map::TileMapTile>();
            registry.register::<camera::TileMapCameraPosition>();
        }
    }
}

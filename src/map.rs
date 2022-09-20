use std::ops::Div;

use bevy::{
    prelude::{
        Commands, Component, ComputedVisibility, Name, Plugin, Res, SystemSet, Transform, Vec2,
        Visibility, VisibilityBundle,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use crate::{plugin::texture_loader, settings};
use rand::Rng;

pub struct MapTestPlugin;

impl Plugin for MapTestPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(settings::GameState::Game).with_system(create_map));
    }
}

#[derive(Component)]
pub struct MapTile;

fn create_map(mut commands: Commands, tilset: Res<texture_loader::TerrainTileset>) {
    let map_width_and_height = 200;
    let mut rng = rand::thread_rng();
    for x in 0..map_width_and_height {
        for y in 0..map_width_and_height {
            spawn_map_tile(
                &mut commands,
                &tilset,
                rng.gen::<usize>() % 100,
                Vec2::new(
                    (x - map_width_and_height.div(2)) as f32,
                    (y - map_width_and_height.div(2)) as f32,
                ),
            );
        }
    }
}

fn spawn_map_tile(
    commands: &mut Commands,
    tilset: &Res<texture_loader::TerrainTileset>,
    id: usize,
    position: Vec2,
) {
    let sprite = TextureAtlasSprite::new(id);
    let sprite_sheet_bundle = SpriteSheetBundle {
        sprite,
        texture_atlas: tilset.0.clone(),
        transform: Transform::from_xyz(position.x * 16.0, position.y * 16.0, 0.0),
        ..Default::default()
    };

    let vis_bundle = VisibilityBundle {
        visibility: Visibility::visible(),
        computed: ComputedVisibility::not_visible(),
    };

    commands
        .spawn_bundle(sprite_sheet_bundle)
        .insert_bundle(vis_bundle)
        .insert(MapTile)
        .insert(Name::new("MapTile"));
}

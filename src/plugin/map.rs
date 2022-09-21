use bevy::{
    prelude::{
        BuildChildren, Commands, Component, ComputedVisibility, Entity, Name, Plugin, Res,
        SystemSet, Transform, UVec2, Vec3, Visibility, VisibilityBundle,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
    transform::TransformBundle,
};

use crate::{plugin::texture_loader, settings};

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(settings::GameState::Game).with_system(create_tilemap_system),
        );
    }
}

fn create_tilemap_system(mut commands: Commands, tilset: Res<texture_loader::TerrainTileset>) {
    let tilemap: TileMap = TileMap::new(settings::DEFAULT_MAP_SIZE, settings::DEFAULT_MAP_SIZE);

    let mut chunk_entities: Vec<Entity> = Vec::new();
    let mut debug_chunk_counter = 1;
    for chunk in tilemap.get_chunks() {
        let mut tile_entities: Vec<Entity> = Vec::new();
        for tile in chunk.get_tiles() {
            tile_entities.push(
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(debug_chunk_counter % 100),
                        texture_atlas: tilset.0.clone(),
                        transform: Transform::from_xyz(
                            tile.pos.x as f32 * 16.0,
                            tile.pos.y as f32 * 16.0,
                            0.0,
                        ),
                        computed_visibility: ComputedVisibility::not_visible(),
                        visibility: Visibility::visible(),
                        ..Default::default()
                    })
                    .insert(tile)
                    .insert(Name::from("TileMapTile"))
                    .id(),
            );
        }

        let locale_chunk_position = bevy::prelude::Vec3::new(
            (chunk.pos.x * settings::DEFAULT_CHUNK_SIZE) as f32 * 16.0,
            (chunk.pos.y * settings::DEFAULT_CHUNK_SIZE) as f32 * 16.0,
            0.0,
        );

        chunk_entities.push(
            commands
                .spawn()
                .insert(chunk)
                .insert_bundle(TransformBundle {
                    local: Transform::from_translation(locale_chunk_position),
                    ..Default::default()
                })
                .insert_bundle(VisibilityBundle {
                    visibility: Visibility::visible(),
                    computed: ComputedVisibility::not_visible(),
                })
                .insert(Name::from("TileMapChunk"))
                .push_children(&tile_entities)
                .id(),
        );
        debug_chunk_counter += 1;
    }

    commands
        .spawn()
        .insert(tilemap)
        .insert_bundle(TransformBundle {
            local: Transform::from_translation(Vec3::ZERO),
            ..Default::default()
        })
        .insert_bundle(VisibilityBundle {
            visibility: Visibility::visible(),
            computed: ComputedVisibility::not_visible(),
        })
        .insert(Name::from("TileMap"))
        .push_children(&chunk_entities);
}

//fn get_all_chunks(query: bevy::prelude::Query<&TileMapChunk>) {
//    //query.for_each(|c| bevy::prelude::info!("{}, {}", c.pos.x, c.pos.y));
//}

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct TileMap {
    pub width: u32,
    pub height: u32,
}

impl TileMap {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn get_chunks(&self) -> Vec<TileMapChunk> {
        let mut chunks: Vec<TileMapChunk> = Vec::new();

        for x in 0..self.width {
            for y in 0..self.height {
                chunks.push(TileMapChunk::new(UVec2::new(x, y)));
            }
        }

        return chunks;
    }
}

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct TileMapChunk {
    pub pos: UVec2,
}

impl TileMapChunk {
    fn new(pos: UVec2) -> Self {
        Self { pos }
    }

    fn get_tiles(&self) -> Vec<TileMapTile> {
        let mut tiles: Vec<TileMapTile> = Vec::new();

        for y in 0..settings::DEFAULT_CHUNK_SIZE {
            for x in 0..settings::DEFAULT_CHUNK_SIZE {
                tiles.push(TileMapTile::new(UVec2::new(x as u32, y as u32)));
            }
        }

        return tiles;
    }
}

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct TileMapTile {
    pub pos: UVec2,
}

impl TileMapTile {
    pub fn new(pos_in_chunk: UVec2) -> Self {
        Self { pos: pos_in_chunk }
    }
}

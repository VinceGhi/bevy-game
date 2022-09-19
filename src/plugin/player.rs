use bevy::{
    prelude::{Commands, Component, Name, Plugin, Res, SystemSet, Transform},
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
};

use crate::settings;

use super::texture_loader::{self, AsciiTilsetId};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(settings::GameState::Game).with_system(spawn_player),
        );
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, tilset: Res<texture_loader::AsciiTileset>) {
    let sprite = TextureAtlasSprite::new(AsciiTilsetId::Zero as usize);
    let sprite_sheet_bundle = SpriteSheetBundle {
        sprite,
        texture_atlas: tilset.0.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    };

    commands
        .spawn_bundle(sprite_sheet_bundle)
        .insert(Player)
        .insert(Name::new("Player"));
}

// setup

use bevy::prelude::*;
use ridge_domain::ridge_field::tile::Tile;
pub mod hero_usecase;
pub mod villains_usecase;
pub mod yatsuhashi_usecase;

// スタジオの設置
pub fn install_studio(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// 若干のmerginをとりつつ正三角形を密に配置
pub fn create_tile(mut commands: Commands) {
    commands.spawn(Tile {
        stolen: 5,
        taste: None,
    });
}

// setup

use bevy::prelude::*;
pub mod hero_usecase;
pub mod villains_usecase;
pub mod yatsuhashi_usecase;

// スタジオの設置
pub fn install_studio(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

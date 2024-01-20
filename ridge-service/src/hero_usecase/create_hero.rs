use bevy::prelude::*;
use ridge_domain::{
    ridge_hero::hero::HeroPosition,
    ridge_yatsuhashi::yatsuhashi::{Yatsuhashi, YatsuhashiAddress, YatsuhashiTaste},
};

/// ヒーローポジショニング
pub fn set_hero(mut commands: Commands) {
    commands.insert_resource(HeroPosition { row: 0, col: 6 });
}

use bevy::prelude::*;
use ridge_domain::ridge_hero::hero::HeroPosition;

/// ヒーローポジショニング
pub fn set_hero(mut commands: Commands) {
    commands.insert_resource(HeroPosition {
        row: 0,
        col: 6,
        past_row: 0,
        past_col: 0,
    });
}

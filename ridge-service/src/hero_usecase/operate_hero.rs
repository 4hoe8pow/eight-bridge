use bevy::prelude::*;
use ridge_domain::ridge_hero::hero::HeroPosition;

pub fn operate_hero(mut hero_position: ResMut<HeroPosition>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::W) {
        hero_position.row += 1;
    }
    if keys.just_pressed(KeyCode::A) {
        hero_position.col -= 1;
    }
    if keys.just_pressed(KeyCode::S) {
        hero_position.row -= 1;
    }
    if keys.just_pressed(KeyCode::D) {
        hero_position.col += 1;
    }
}

use bevy::prelude::*;
use ridge_domain::{
    ridge_hero::hero::HeroPosition,
    ridge_yatsuhashi::yatsuhashi::{Yatsuhashi, YatsuhashiAddress, YatsuhashiTaste},
};

pub fn operate_hero(
    mut hero_position: ResMut<HeroPosition>,
    keys: Res<Input<KeyCode>>,
    mut yatsuhashies: Query<(&mut YatsuhashiTaste, &mut YatsuhashiAddress), With<Yatsuhashi>>,
) {
    // 現在のHeroPosition の味を消去
    yatsuhashies
        .iter_mut()
        .filter(|(_, address)| address.row == hero_position.row && address.col == hero_position.col)
        .for_each(|(mut taste, _)| *taste = YatsuhashiTaste::default());

    if keys.any_just_pressed([KeyCode::W, KeyCode::Up]) {
        hero_position.row += 1;
        if hero_position.row > 6 {
            hero_position.col -= 1;
        } else if hero_position.row < 6 {
            hero_position.col += 1;
        }
    }
    if keys.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        hero_position.col -= 1;
    }
    if keys.any_just_pressed([KeyCode::S, KeyCode::Down]) {
        hero_position.row -= 1;
        if hero_position.row > 5 {
            hero_position.col += 1;
        } else if hero_position.row < 5 {
            hero_position.col -= 1;
        }
    }
    if keys.any_just_pressed([KeyCode::D, KeyCode::Right]) {
        hero_position.col += 1;
    }

    // Heroposition === YatsuhashiAddress にマッチする三角形をゴマ味に変更
    yatsuhashies
        .iter_mut()
        .filter(|(_, address)| address.row == hero_position.row && address.col == hero_position.col)
        .for_each(|(mut taste, _)| *taste = YatsuhashiTaste::Sesami);
}

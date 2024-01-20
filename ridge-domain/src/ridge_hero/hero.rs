use bevy::prelude::*;

/// プレーヤー八つ橋
#[derive(Component, Default, Debug)]
pub struct Hero;

/// 操作基準
#[derive(Resource)]
pub struct HeroPosition {
    pub row: i8,
    pub col: i8,
}

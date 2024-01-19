use bevy::prelude::*;

use super::villain_animation::VillainAnimation;

/// 外野にいるただの動物
#[derive(Component, Default, Debug)]
pub struct Villains;

/// 敵関連のコンボーネントバンドル
#[derive(Bundle, Default)]
pub struct VillainsBundle {
    pub villains: Villains,
    pub sprite: SpriteSheetBundle,
    pub animation: VillainAnimation,
}

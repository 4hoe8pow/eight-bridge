use bevy::prelude::*;

use crate::ridge_yatsuhashi::yatsuhashi::YatsuhashiTaste;

#[derive(Component)]
// タイル
pub struct Tile {
    /// 6角形の状態から奪われた八つ橋の数
    /// 現在のタイルがあと何枚で６角形になるのかを示す
    pub stolen: u8,
    /// Some ならタイルはその八つ橋色に光る
    /// None なら透明
    pub taste: Option<YatsuhashiTaste>,
}

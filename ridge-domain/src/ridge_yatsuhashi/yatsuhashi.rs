use bevy::prelude::*;
use rand::Rng;

/// 八つ橋
#[derive(Component, Default, Debug)]
pub struct Yatsuhashi;

/// 残りいくつで六角形になるかカウンター
#[derive(Component, Default, Debug)]
pub struct YatsuhashiStolen(pub u8);

/// 八つ橋の味ないし色
#[derive(Component, Default, Debug, Clone)]
pub enum YatsuhashiTaste {
    /// 透明
    #[default]
    Tasteless,
    /// 黄
    Cinnamon,
    /// 緑
    Matcha,
    /// 青
    Ramune,
    /// 赤
    Strawberry,
}

impl YatsuhashiTaste {
    /// ランダムな列挙型メンバーを生成する
    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..=3) {
            0 => YatsuhashiTaste::Cinnamon,
            1 => YatsuhashiTaste::Matcha,
            2 => YatsuhashiTaste::Ramune,
            _ => YatsuhashiTaste::Strawberry,
        }
    }
}

#[derive(Bundle, Default)]
// タイル
pub struct YatsuhashiBundle {
    /// 八つ橋エンティティ
    pub yatsuhashi: Yatsuhashi,
    /// 6角形の状態から奪われた八つ橋の数
    /// 現在のタイルがあと何枚で６角形になるのかを示す
    pub stolen: YatsuhashiStolen,
    /// Some ならタイルはその八つ橋色に光る
    /// None なら透明
    pub taste: YatsuhashiTaste,
}

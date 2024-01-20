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
    /// ヒーロー
    Sesami,
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
        match rand::thread_rng().gen_range(2..=5) {
            2 => YatsuhashiTaste::Cinnamon,
            3 => YatsuhashiTaste::Matcha,
            4 => YatsuhashiTaste::Ramune,
            _ => YatsuhashiTaste::Strawberry,
        }
    }
}

/// 八つ橋インデックス
#[derive(Component, Default, Debug)]
pub struct YatsuhashiIndex {
    pub row: u8,
    pub col: u8,
}

#[derive(Bundle, Default)]
// タイル
pub struct YatsuhashiBundle {
    /// 八つ橋エンティティ
    pub yatsuhashi: Yatsuhashi,
    /// 6角形の状態から奪われた八つ橋の数
    /// 現在のタイルがあと何枚で６角形になるのかを示す
    pub stolen: YatsuhashiStolen,
    /// 八つ橋の味
    pub taste: YatsuhashiTaste,
    /// 所在地
    pub address: YatsuhashiIndex,
}

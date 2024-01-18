use bevy::prelude::*;
use rand::Rng;

/// 八つ橋
#[derive(Component, Default, Debug)]
pub struct Yatsuhashi;

/// 八つ橋の味
#[derive(Component, Default, Debug, Clone)]
pub enum YatsuhashiTaste {
    #[default]
    Cinnamon,
    Matcha,
    Ramune,
    Strawberry,
}

impl YatsuhashiTaste {
    pub fn random() -> Self {
        // ランダムな列挙型メンバーを生成する
        match rand::thread_rng().gen_range(0..=3) {
            0 => YatsuhashiTaste::Cinnamon,
            1 => YatsuhashiTaste::Matcha,
            2 => YatsuhashiTaste::Ramune,
            _ => YatsuhashiTaste::Strawberry,
        }
    }
}

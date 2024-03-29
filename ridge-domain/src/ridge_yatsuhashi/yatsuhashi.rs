use bevy::prelude::*;
use rand::Rng;

/// 八つ橋
#[derive(Component, Default, Debug)]
pub struct Yatsuhashi;

/// 残りいくつで六角形になるかカウンター
#[derive(Component, Default, Debug)]
pub struct YatsuhashiStolen(pub u8);

/// 八つ橋の味ないし色
#[derive(Component, Default, Debug, Clone, PartialEq)]
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
#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct YatsuhashiAddress {
    pub row: i8,
    pub col: i8,
}

impl YatsuhashiAddress {
    pub fn in_field(&self) -> bool {
        match self.row {
            0 | 11 if (0..=12).contains(&self.col) => true,
            1 | 10 if (0..=14).contains(&self.col) => true,
            2 | 9 if (0..=16).contains(&self.col) => true,
            3 | 8 if (0..=18).contains(&self.col) => true,
            4 | 7 if (0..=20).contains(&self.col) => true,
            5 | 6 if (0..=22).contains(&self.col) => true,
            _ => false,
        }
    }

    pub fn reflect(&self, direction: YatsuhashiDirection) -> YatsuhashiDirection {
        let mut next_direction = YatsuhashiDirection::default();
        if self.col < 0 {
            if self.row > 5 && direction == YatsuhashiDirection::NineOclock {
                next_direction = YatsuhashiDirection::FourOclock;
            } else if self.row > 5 && direction == YatsuhashiDirection::TenOclock {
                next_direction = YatsuhashiDirection::ThreeOclock;
            } else if self.row <= 5 && direction == YatsuhashiDirection::EightOclock {
                next_direction = YatsuhashiDirection::ThreeOclock;
            } else if self.row <= 5 && direction == YatsuhashiDirection::NineOclock {
                next_direction = YatsuhashiDirection::TwoOclock;
            }
        } else if self.row < 0 {
            if direction == YatsuhashiDirection::FourOclock {
                next_direction = YatsuhashiDirection::TwoOclock;
            } else {
                next_direction = YatsuhashiDirection::TenOclock;
            }
        } else if self.row > 11 {
            if direction == YatsuhashiDirection::TwoOclock {
                next_direction = YatsuhashiDirection::FourOclock;
            } else {
                next_direction = YatsuhashiDirection::EightOclock;
            }
        } else if self.row == 0 && self.col > 11 {
            if direction == YatsuhashiDirection::ThreeOclock {
                next_direction = YatsuhashiDirection::TenOclock;
            } else {
                next_direction = YatsuhashiDirection::NineOclock;
            }
        } else if self.row == 1 && self.col > 13 {
            if direction == YatsuhashiDirection::ThreeOclock {
                next_direction = YatsuhashiDirection::TenOclock;
            } else {
                next_direction = YatsuhashiDirection::NineOclock;
            }
        } else if self.row == 2 && self.col > 15 {
            if direction == YatsuhashiDirection::ThreeOclock {
                next_direction = YatsuhashiDirection::TenOclock;
            } else {
                next_direction = YatsuhashiDirection::NineOclock;
            }
        } else if self.row == 3 && self.col > 17 {
            if direction == YatsuhashiDirection::ThreeOclock {
                next_direction = YatsuhashiDirection::TenOclock;
            } else {
                next_direction = YatsuhashiDirection::NineOclock;
            }
        } else if self.row == 4 && self.col > 19 {
            if direction == YatsuhashiDirection::ThreeOclock {
                next_direction = YatsuhashiDirection::TenOclock;
            } else {
                next_direction = YatsuhashiDirection::NineOclock;
            }
        } else if self.row == 5 && self.col > 21 {
            if direction == YatsuhashiDirection::ThreeOclock {
                next_direction = YatsuhashiDirection::TenOclock;
            } else {
                next_direction = YatsuhashiDirection::NineOclock;
            }
        } else if self.row == 6 && self.col > 21 {
            if direction == YatsuhashiDirection::ThreeOclock {
                next_direction = YatsuhashiDirection::EightOclock;
            } else {
                next_direction = YatsuhashiDirection::NineOclock;
            }
        } else if self.row == 7 && self.col > 19 {
            if direction == YatsuhashiDirection::ThreeOclock {
                next_direction = YatsuhashiDirection::EightOclock;
            } else {
                next_direction = YatsuhashiDirection::NineOclock;
            }
        } else if self.row == 8 && self.col > 17 {
            if direction == YatsuhashiDirection::ThreeOclock {
                next_direction = YatsuhashiDirection::EightOclock;
            } else {
                next_direction = YatsuhashiDirection::NineOclock;
            }
        } else if self.row == 9 && self.col > 15 {
            if direction == YatsuhashiDirection::ThreeOclock {
                next_direction = YatsuhashiDirection::EightOclock;
            } else {
                next_direction = YatsuhashiDirection::NineOclock;
            }
        } else if self.row == 10 && self.col > 13 {
            if direction == YatsuhashiDirection::ThreeOclock {
                next_direction = YatsuhashiDirection::EightOclock;
            } else {
                next_direction = YatsuhashiDirection::NineOclock;
            }
        } else if self.row == 11 && self.col > 11 {
            if direction == YatsuhashiDirection::ThreeOclock {
                next_direction = YatsuhashiDirection::EightOclock;
            } else {
                next_direction = YatsuhashiDirection::NineOclock;
            }
        }

        next_direction
    }
}

/// 攻撃方向
#[derive(Component, Default, Debug, Clone, PartialEq)]
pub enum YatsuhashiDirection {
    #[default]
    NoMove,
    EightOclock,
    FourOclock,
    TwoOclock,
    TenOclock,
    ThreeOclock,
    NineOclock,
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
    pub address: YatsuhashiAddress,
    /// 進路
    pub dirction: YatsuhashiDirection,
}

use ridge_domain::ridge_yatsuhashi::yatsuhashi::YatsuhashiAddress;

pub const YATSUHASHI_SIZE: f32 = 20.0;
pub const SHIFTS: [i8; 12] = [12, 14, 16, 18, 20, 22, 22, 20, 18, 16, 14, 12];

pub const CINNAMON_SPAWN_POINT: YatsuhashiAddress = YatsuhashiAddress { row: 9, col: 16 };
pub const MATCHA_SPAWN_POINT: YatsuhashiAddress = YatsuhashiAddress { row: 9, col: 0 };
pub const RAMUNE_SPAWN_POINT: YatsuhashiAddress = YatsuhashiAddress { row: 2, col: 0 };
pub const STRAWBERRY_SPAWN_POINT: YatsuhashiAddress = YatsuhashiAddress { row: 2, col: 16 };

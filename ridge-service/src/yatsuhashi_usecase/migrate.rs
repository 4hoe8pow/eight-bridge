use bevy::prelude::*;
use ridge_domain::ridge_yatsuhashi::yatsuhashi::{
    Yatsuhashi, YatsuhashiAddress, YatsuhashiDirection, YatsuhashiTaste,
};

/// 移動準備
#[derive(Event)]
pub struct ReloadEvent {
    pub current: YatsuhashiAddress,
    pub next: YatsuhashiAddress,
    pub migrate_taste: YatsuhashiTaste,
    pub migrate_direction: YatsuhashiDirection,
}

// 結合
#[derive(Event)]
pub struct BondEvent {
    pub child: YatsuhashiAddress,
}

// 次の八つ橋が無味なら次の八つ橋に今の八つ橋の味を移し、自身を無味にする
// 次の八つ橋がヒーローかつ自身と異なる味なら結合する
pub fn push_yatsuhashi(
    yatsuhashies: Query<
        (
            &mut YatsuhashiDirection,
            &mut YatsuhashiAddress,
            &YatsuhashiTaste,
        ),
        With<Yatsuhashi>,
    >,
    mut event: EventWriter<ReloadEvent>,
) {
    // a. 次の八つ橋の場所をしらべる
    for (direction, current_address, taste) in yatsuhashies.iter() {
        let next_address: Option<YatsuhashiAddress> = match *direction {
            YatsuhashiDirection::EightOclock => {
                let (drow, dcol) = if current_address.row < 6 {
                    (-1, -2)
                } else {
                    match current_address.row == 6 {
                        true => (-1, -1),
                        false => (-1, 0),
                    }
                };
                Some(YatsuhashiAddress {
                    row: current_address.row + drow,
                    col: current_address.col + dcol,
                })
            }
            YatsuhashiDirection::FourOclock => {
                let (drow, dcol) = if current_address.row < 6 {
                    (-1, 0)
                } else {
                    match current_address.row == 6 {
                        true => (-1, 1),
                        false => (-1, 2),
                    }
                };
                Some(YatsuhashiAddress {
                    row: current_address.row + drow,
                    col: current_address.col + dcol,
                })
            }
            YatsuhashiDirection::TwoOclock => {
                let (drow, dcol) = if current_address.row > 5 {
                    (1, 0)
                } else {
                    match current_address.row == 5 {
                        true => (1, 1),
                        false => (1, 2),
                    }
                };
                Some(YatsuhashiAddress {
                    row: current_address.row + drow,
                    col: current_address.col + dcol,
                })
            }
            YatsuhashiDirection::TenOclock => {
                let (drow, dcol) = if current_address.row > 5 {
                    (1, -2)
                } else {
                    match current_address.row == 5 {
                        true => (1, -1),
                        false => (1, 0),
                    }
                };
                Some(YatsuhashiAddress {
                    row: current_address.row + drow,
                    col: current_address.col + dcol,
                })
            }
            YatsuhashiDirection::ThreeOclock => Some(YatsuhashiAddress {
                row: current_address.row,
                col: current_address.col + 1,
            }),
            YatsuhashiDirection::NineOclock => Some(YatsuhashiAddress {
                row: current_address.row,
                col: current_address.col - 1,
            }),
            //実在する番地を返すと動いてしまう
            YatsuhashiDirection::NoMove => None,
        };

        if next_address.is_some() {
            if next_address.clone().unwrap().in_field() {
                event.send(ReloadEvent {
                    current: current_address.clone(),
                    next: next_address.clone().unwrap(),
                    migrate_taste: taste.clone(),
                    migrate_direction: direction.clone(),
                })
            } else {
                event.send(ReloadEvent {
                    current: current_address.clone(),
                    next: current_address.clone(),
                    migrate_taste: taste.clone(),
                    migrate_direction: next_address.clone().unwrap().reflect(direction.clone()),
                })
            }
        }
    }
}

pub fn pop_yatsuhashi(
    mut reload_events: EventReader<ReloadEvent>,
    mut yatsuhashies: Query<
        (
            &mut YatsuhashiTaste,
            &mut YatsuhashiAddress,
            &mut YatsuhashiDirection,
        ),
        With<Yatsuhashi>,
    >,
    mut bond_event: EventWriter<BondEvent>,
) {
    // 次の八つ橋の味を調べる
    for event in reload_events.read() {
        let (current, next, migrate_taste, migrate_direction) = (
            &event.current,
            &event.next,
            &event.migrate_taste,
            &event.migrate_direction,
        );

        yatsuhashies
            .iter_mut()
            .map(|(taste, address, direction)| (taste, address.clone(), direction))
            .filter(|(_, address, _)| *address == *next || *address == *current)
            .for_each(|(mut taste, address, mut direction)| {
                // 反射
                if address == *next && next == current && *taste != YatsuhashiTaste::default() {
                    *direction = migrate_direction.clone();
                }
                // 結合
                else if address == *next && *taste == YatsuhashiTaste::Sesami {
                    bond_event.send(BondEvent {
                        child: current.clone(),
                    })
                }
                // 進行
                else if address == *next && *taste == YatsuhashiTaste::Tasteless {
                    *taste = migrate_taste.clone();
                    *direction = migrate_direction.clone();
                }
                // 残像消去
                else if address == *current && *taste == migrate_taste.clone() {
                    *taste = YatsuhashiTaste::Tasteless;
                    *direction = YatsuhashiDirection::NoMove;
                }
            });
    }
}

use bevy::prelude::*;
use ridge_domain::ridge_yatsuhashi::yatsuhashi::{
    Yatsuhashi, YatsuhashiAddress, YatsuhashiDirection, YatsuhashiTaste,
};

/// 移動準備
#[derive(Event)]
pub struct YatsuhashiQueueEvent {
    pub current: YatsuhashiAddress,
    pub next: YatsuhashiAddress,
    pub migrate_taste: YatsuhashiTaste,
    pub migrate_direction: YatsuhashiDirection,
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
    mut event: EventWriter<YatsuhashiQueueEvent>,
) {
    // a. 次の八つ橋の場所をしらべる
    for (direction, current_address, taste) in yatsuhashies.iter() {
        let next_address = match *direction {
            YatsuhashiDirection::EightOclock => {
                let (drow, dcol) = if current_address.row < 6 {
                    (-1, -2)
                } else if current_address.row == 6 {
                    (-1, -1)
                } else {
                    (-1, 0)
                };
                YatsuhashiAddress {
                    row: current_address.row + drow,
                    col: current_address.col + dcol,
                }
            }
            YatsuhashiDirection::FourOclock => {
                let (drow, dcol) = if current_address.row < 6 {
                    (-1, 2)
                } else if current_address.row == 6 {
                    (-1, 1)
                } else {
                    (-1, 0)
                };
                YatsuhashiAddress {
                    row: current_address.row + drow,
                    col: current_address.col + dcol,
                }
            }
            YatsuhashiDirection::TwoOclock => {
                let (drow, dcol) = if current_address.row > 5 {
                    (1, 2)
                } else if current_address.row == 5 {
                    (1, 1)
                } else {
                    (1, 0)
                };
                YatsuhashiAddress {
                    row: current_address.row + drow,
                    col: current_address.col + dcol,
                }
            }
            YatsuhashiDirection::TenOclock => {
                let (drow, dcol) = if current_address.row > 5 {
                    (1, -2)
                } else if current_address.row == 5 {
                    (1, -1)
                } else {
                    (1, 0)
                };
                YatsuhashiAddress {
                    row: current_address.row + drow,
                    col: current_address.col + dcol,
                }
            }
            YatsuhashiDirection::ThreeOclock => YatsuhashiAddress {
                row: current_address.row,
                col: current_address.col + 1,
            },
            YatsuhashiDirection::NineOclock => YatsuhashiAddress {
                row: current_address.row,
                col: current_address.col - 1,
            },
            _ => YatsuhashiAddress::default(),
        };

        if next_address.in_field() {
            eprintln!("IN-FIELD:{:?}", next_address);
            event.send(YatsuhashiQueueEvent {
                current: current_address.clone(),
                next: next_address.clone(),
                migrate_taste: taste.clone(),
                migrate_direction: direction.clone(),
            })
        } else {
            eprintln!("OUT-FIELD:{:?}", next_address);
            event.send(YatsuhashiQueueEvent {
                current: current_address.clone(),
                next: current_address.clone(),
                migrate_taste: taste.clone(),
                migrate_direction: next_address.clone().reflect(direction.clone()),
            })
        }
    }
}

pub fn pop_yatsuhashi(
    mut events: EventReader<YatsuhashiQueueEvent>,
    mut yatsuhashies: Query<
        (
            &mut YatsuhashiTaste,
            &mut YatsuhashiAddress,
            &mut YatsuhashiDirection,
        ),
        With<Yatsuhashi>,
    >,
) {
    // 次の八つ橋の味を調べる
    for event in events.read() {
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
                if address == *next {
                    *taste = migrate_taste.clone();
                    *direction = migrate_direction.clone()
                } else if address == *current
                    && *taste == migrate_taste.clone()
                    && *taste != YatsuhashiTaste::Sesami
                {
                    *taste = YatsuhashiTaste::Tasteless;
                    *direction = YatsuhashiDirection::NoMove;
                }
            });
    }
}

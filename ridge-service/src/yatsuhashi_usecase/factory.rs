use crate::{
    const_parameter::{
        CINNAMON_SPAWN_POINT, MATCHA_SPAWN_POINT, RAMUNE_SPAWN_POINT, SHIFTS,
        STRAWBERRY_SPAWN_POINT, YATSUHASHI_SIZE,
    },
    villains_usecase::fire_yatsuhashi::FireEvent,
};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use ridge_domain::ridge_yatsuhashi::yatsuhashi::{
    Yatsuhashi, YatsuhashiAddress, YatsuhashiBundle, YatsuhashiDirection, YatsuhashiStolen,
    YatsuhashiTaste,
};
/// 若干のmerginをとりつつ正三角形を密に配置
pub fn create_yatsuhashies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (width, height) = (
        window_query.single().width(),
        window_query.single().height(),
    );
    let mut is_reverse = false;

    SHIFTS.iter().enumerate().for_each(|(index, &shift)| {
        (0..=shift).for_each(|pivot| {
            // 反転の判定
            if (index < 6 && is_even(pivot)) || (index >= 6 && !is_even(pivot)) {
                is_reverse = true;
            } else {
                is_reverse = false;
            };

            // 反転の定義
            let reverse_half = if is_reverse {
                Transform::from_rotation(Quat::from_rotation_z((180.0_f32).to_radians()))
            } else {
                Transform::from_rotation(Quat::from_rotation_z((0.0_f32).to_radians()))
            };

            // 八つ橋の設置
            commands
                .spawn(YatsuhashiBundle {
                    stolen: YatsuhashiStolen(5),
                    taste: YatsuhashiTaste::default(),
                    address: YatsuhashiAddress {
                        row: index as i8,
                        col: pivot,
                    },
                    ..default()
                })
                .insert(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::RegularPolygon::new(YATSUHASHI_SIZE, 3).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::WHITE)),
                    transform: Transform::from_translation(provice_hexagon(
                        index as i8,
                        pivot,
                        width,
                        height,
                        is_reverse,
                    ))
                    .mul_transform(reverse_half),
                    ..default()
                });
        })
    });
}

fn provice_hexagon(row: i8, col: i8, width: f32, height: f32, is_reverse: bool) -> Vec3 {
    let dx = YATSUHASHI_SIZE * 1.5;

    let mut origin = Vec3::new(
        -(width / 2.1) + dx * (col as f32),
        -(height / 4.0) + YATSUHASHI_SIZE * 2.0 * (row as f32),
        0.0,
    );

    match row {
        0 | 11 => origin.x += dx * 5.0,
        1 | 10 => origin.x += dx * 4.0,
        2 | 9 => origin.x += dx * 3.0,
        3 | 8 => origin.x += dx * 2.0,
        4 | 7 => origin.x += dx * 1.0,
        _ => (),
    }

    if is_reverse {
        origin.y += YATSUHASHI_SIZE / 2.0;
    }

    origin
}

fn is_even(num: i8) -> bool {
    num % 2 == 0
}

pub fn fire(
    mut fires: EventReader<FireEvent>,
    mut yatsuhashies: Query<
        (
            &mut YatsuhashiTaste,
            &mut YatsuhashiAddress,
            &mut YatsuhashiDirection,
        ),
        With<Yatsuhashi>,
    >,
) {
    for fires in fires.read() {
        let fire_taste = &fires.taste;

        let (fire_address, fire_direction) = match *fire_taste {
            YatsuhashiTaste::Cinnamon => (CINNAMON_SPAWN_POINT, YatsuhashiDirection::EightOclock),
            YatsuhashiTaste::Matcha => (MATCHA_SPAWN_POINT, YatsuhashiDirection::FourOclock),
            YatsuhashiTaste::Ramune => (RAMUNE_SPAWN_POINT, YatsuhashiDirection::TwoOclock),
            YatsuhashiTaste::Strawberry => (STRAWBERRY_SPAWN_POINT, YatsuhashiDirection::TenOclock),
            _ => (
                YatsuhashiAddress { row: 0, col: 0 },
                YatsuhashiDirection::NoMove,
            ),
        };
        yatsuhashies
            .iter_mut()
            .filter(|(_, address, _)| {
                address.row == fire_address.row && address.col == fire_address.col
            })
            .for_each(|(mut taste, _, mut direction)| {
                *taste = fire_taste.clone();
                *direction = fire_direction.clone();
            });
    }
}

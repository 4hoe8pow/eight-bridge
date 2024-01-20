use crate::const_parameter::{COL_MAX, ROW_MAX, YATSUHASHI_SIZE};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use ridge_domain::ridge_yatsuhashi::yatsuhashi::{
    YatsuhashiBundle, YatsuhashiIndex, YatsuhashiStolen, YatsuhashiTaste,
};
/// 若干のmerginをとりつつ正三角形を密に配置
pub fn create_yatsuhashies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (width, height) = (
        window_query.single().width() / 2.0,
        window_query.single().height() / 2.0,
    );

    (1..=ROW_MAX).for_each(|i| {
        (1..=COL_MAX).for_each(|j| {
            commands
                .spawn(YatsuhashiBundle {
                    stolen: YatsuhashiStolen(5),
                    taste: YatsuhashiTaste::default(),
                    address: YatsuhashiIndex { row: i, col: j },
                    ..default()
                })
                .insert(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::RegularPolygon::new(YATSUHASHI_SIZE, 3).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                    transform: Transform::from_translation(Vec3::new(
                        -width + (YATSUHASHI_SIZE * 2.1) * (i as f32),
                        -height + (YATSUHASHI_SIZE * 2.1) * (j as f32),
                        0.,
                    )),
                    ..default()
                });
        })
    });
}

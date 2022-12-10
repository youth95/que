use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

use crate::camera::SceneCamera;
use crate::marks::{EnemyLabel, EnemyMark, RegionId, RegionRect, ValueText};
use crate::pool::values::Value;
use crate::{AudioAssets, GameStage};

use super::events::{MouseOverEmpty, MouseOverRegionEvent, PlayAudioEvent};
use super::RegionClickEvent;
use super::{pure::RegionMark, Regions};

use crate::marks::{EnemyStatus, EnemyText, HPColor, RegionStatus};
use bevy::math::Vec3;
use bevy::prelude::Transform;

pub struct RegionRenderPlugin;

const SIZE: f32 = 32.;
const GAP: f32 = 4.;

impl Plugin for RegionRenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // .add_system_set(SystemSet::on_enter(GameStage::Main).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameStage::Main)
                    // .with_system(update_cursor_texture)
                    // .with_system(update_cursor_pos)
                    .with_system(current_world_mouse)
                    .with_system(play_audio_system)
                    .with_system(spawn_region_rect)
                    .with_system(mouse_interaction)
                    // .with_system(fill_enemy_text_system)
                    .with_system(fill_value_text_system)
                    .with_system(update_enemy_hp_text_system)
                    .with_system(region_rect_color_system),
            );
    }
}

fn spawn_region_rect(
    mut commands: Commands,
    regions: ResMut<Regions>,
    asset_server: Res<AssetServer>,
    query: Query<
        (
            &RegionId,
            Option<&EnemyMark>,
            Option<&Value>,
            Option<&EnemyLabel>,
        ),
        Added<RegionMark>,
    >,
) {
    for (RegionId(region_id), _, value, label) in query.iter() {
        if let Some(tile) = regions.tiles.get(region_id) {
            let transform = tile.to_transform(SIZE, GAP).unwrap();
            let region_id = RegionId(tile.id);

            // region rect
            commands
                .spawn(SpriteBundle::default())
                .insert(Sprite {
                    color: Color::NONE,
                    ..default()
                })
                .insert(transform)
                .insert(RegionRect)
                .insert(region_id);

            if let Some(label) = label {
                //  enemy hp color
                commands
                    .spawn(SpriteBundle::default())
                    .insert(Transform {
                        translation: Vec3::new(
                            transform.translation.x,
                            transform.translation.y,
                            1.,
                        ),
                        scale: transform.scale,
                        ..Default::default()
                    })
                    .insert(Sprite {
                        color: Color::rgba_u8(255, 0, 0, 255),
                        ..Default::default()
                    })
                    .insert(HPColor)
                    .insert(Visibility { is_visible: false })
                    .insert(region_id);
                // enemy current_hp_text
                let icon = asset_server.get_handle(label.icon.as_str());

                commands
                    .spawn(SpriteBundle::default())
                    .insert(Transform {
                        translation: Vec3::new(
                            transform.translation.x,
                            transform.translation.y,
                            2.,
                        ),
                        // scale: transform.scale,
                        ..Default::default()
                    })
                    .insert(Sprite {
                        color: Color::NONE,
                        custom_size: Some(Vec2::new(transform.scale.x, transform.scale.y)),
                        ..default()
                    })
                    .with_children(|parent| {
                        // text
                        parent
                            .spawn(Text2dBundle {
                                transform: Transform {
                                    translation: Vec3::new(0., -8., 1.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(Visibility { is_visible: false })
                            .insert(EnemyText)
                            .insert(region_id);
                        // icon
                        parent
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::GOLD,
                                    custom_size: Some(Vec2::new(16., 16.)),
                                    ..default()
                                },
                                texture: icon,
                                transform: Transform {
                                    scale: Vec3::new(1.5, 1.5, 1.),
                                    ..Default::default()
                                },
                                ..default()
                            })
                            .insert(Visibility { is_visible: false })
                            .insert(region_id);
                    });
            }

            if value.is_some() {
                commands
                    .spawn(Text2dBundle {
                        visibility: Visibility { is_visible: false },
                        transform: Transform {
                            translation: Vec3::new(
                                transform.translation.x,
                                transform.translation.y,
                                2.,
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(ValueText)
                    .insert(region_id);
            }
        }
    }
}

// fn fill_enemy_text_system(
//     asset_server: Res<AssetServer>,
//     mut query: Query<(&mut Text, &RegionId), Added<EnemyText>>,
//     query_enemy: Query<(&EnemyStatus, &RegionId)>,
// ) {
//     let font = asset_server.load("fonts/hanti.ttf");
//     let text_style = TextStyle {
//         font,
//         font_size: 16.0,
//         color: Color::WHITE,
//     };
//     let text_alignment = TextAlignment {
//         vertical: VerticalAlign::Center,
//         horizontal: HorizontalAlign::Center,
//     };
//     for (mut text, RegionId(id)) in &mut query.iter_mut() {
//         for (enemy, RegionId(region_id)) in &mut query_enemy.iter() {
//             if id == region_id {
//                 text.set(Box::new(Text::with_section(
//                     format!("\n{}/{}", enemy.cur_hp, enemy.max_hp),
//                     text_style.clone(),
//                     text_alignment,
//                 )))
//                 .unwrap();
//             }
//         }
//     }
// }

fn fill_value_text_system(
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Text, &RegionId), Added<ValueText>>,
    query_value: Query<(&RegionId, &Value)>,
) {
    let font = asset_server.load("fonts/hanti.ttf");
    let text_style = TextStyle {
        font,
        font_size: 18.0,
        color: Color::NAVY,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    for (mut text, RegionId(id)) in &mut query.iter_mut() {
        for (RegionId(region_id), value) in query_value.iter() {
            if id == region_id {
                text.set(Box::new(
                    Text::from_section(format!("{}", value.name), text_style.clone())
                        .with_alignment(text_alignment),
                ))
                .unwrap();
            }
        }
    }
}

fn update_enemy_hp_text_system(
    mut query: Query<(&mut Text, &RegionId), With<EnemyText>>,
    query_enemy: Query<(&EnemyStatus, &RegionId), Changed<EnemyStatus>>,
    mut query_color: Query<(&mut Sprite, &RegionId), With<HPColor>>,
) {
    for (enemy, RegionId(region_id)) in &mut query_enemy.iter() {
        for (mut text, RegionId(id)) in &mut query.iter_mut() {
            if id == region_id {
                if text.sections.len() != 0 {
                    text.sections[0].value = format!("\n{}/{}", enemy.cur_hp, enemy.max_hp);
                }
                for (mut sp, RegionId(region_id)) in &mut query_color.iter_mut() {
                    if region_id == id {
                        sp.color.set_a(enemy.cur_hp as f32 / enemy.max_hp as f32);
                    }
                }
            }
        }
    }
}

fn region_rect_color_system(
    region_status_query: Query<
        (&RegionId, &RegionStatus),
        // Or<(Changed<RegionStatus>, Added<RegionStatus>)>,
    >,
    mut region_react_query: Query<(&RegionId, &mut Sprite), With<RegionRect>>,
) {
    for (RegionId(region_id), region_status) in region_status_query.iter() {
        for (RegionId(id), mut sprite) in region_react_query.iter_mut() {
            if region_id == id {
                match region_status {
                    RegionStatus::Mist => {
                        sprite.color = Color::BLACK;
                    }
                    RegionStatus::Found => {
                        sprite.color = Color::GREEN;
                    }
                    RegionStatus::Visited => {
                        sprite.color = Color::GRAY;
                    }
                };
            }
        }
    }
}

fn mouse_interaction(
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    q_camera: Query<&Transform, With<SceneCamera>>,
    q_regions: Query<(&RegionId, &Transform), With<Sprite>>,

    mut trigger_region_event: EventWriter<RegionClickEvent>,
    mut mouse_over_region: EventWriter<MouseOverRegionEvent>,
    mut mouse_over_empty: EventWriter<MouseOverEmpty>,
) {
    let mut ev = None;
    if let Some(wnd) = windows.get_primary() {
        if let Some(pos) = wnd.cursor_position() {
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            let p = pos - size / 2.0;
            if let Ok(camera_transform) = q_camera.get_single() {
                let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
                for (RegionId(id), region) in q_regions.iter() {
                    let dx = (pos_wld.x - region.translation.x).abs();
                    let dy = (pos_wld.y - region.translation.y).abs();
                    if dx <= region.scale.x / 2. && dy <= region.scale.y / 2. {
                        ev = Some(MouseOverRegionEvent(*id))
                    }
                }
            }
        }
    }

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(MouseOverRegionEvent(id)) = ev {
            trigger_region_event.send(RegionClickEvent(id));
        }
    }

    if let Some(MouseOverRegionEvent(id)) = ev {
        mouse_over_region.send(MouseOverRegionEvent(id));
    } else {
        mouse_over_empty.send(MouseOverEmpty);
    }
}

#[derive(Resource, Default)]
pub struct WorldMouse(pub Vec3);

fn current_world_mouse(
    mut windows: ResMut<Windows>,
    q_camera: Query<&Transform, With<SceneCamera>>,
    mut commands: Commands,
) {
    if let Some(wnd) = windows.get_primary_mut() {
        // if wnd.cursor_visible() {
        //     wnd.set_cursor_visibility(false);
        // }
        if let Some(pos) = wnd.cursor_position() {
            let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            let p = pos - size / 2.0;
            if let Ok(camera_transform) = q_camera.get_single() {
                let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
                commands.insert_resource(WorldMouse(Vec3::new(pos_wld.x, pos_wld.y, pos_wld.z)));
            }
        }
    }
}

#[derive(Component)]
struct Cursor;

fn play_audio_system(
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut play_audio_event: EventReader<PlayAudioEvent>,
) {
    for PlayAudioEvent(path) in play_audio_event.iter() {
        let sound = match path {
            super::events::AudioSound::Click => audio_assets.click.clone(),
            super::events::AudioSound::Dao5 => audio_assets.dao5.clone(),
        };
        audio.play(sound);
    }
}

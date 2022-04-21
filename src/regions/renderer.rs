use bevy::prelude::*;

use crate::marks::{EnemyMark, RegionId, RegionRect};

use super::{pure::RegionMark, Regions};

use crate::marks::{EnemyStatus, EnemyText, HPColor, RegionStatus};
use bevy::math::Vec3;
use bevy::prelude::Transform;

pub use super::pure::{
    ChangeEnemyHpEvent, ChangeRegionStatusEvent, RegionPurePlugin, TriggerRegionEvent,
};

pub struct RegionRenderPlugin;

const SIZE: f32 = 32.;
const GAP: f32 = 4.;

impl Plugin for RegionRenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(spawn_region_rect)
            .add_system(mouse_interaction)
            .add_system(update_enemy_hp_text_system)
            .add_system(region_rect_color_system);
    }
}

pub fn spawn_region_rect(
    mut commands: Commands,
    regions: ResMut<Regions>,
    query: Query<(&RegionId, Option<&EnemyMark>), Added<RegionMark>>,
) {
    for (RegionId(region_id), enemy_mark) in query.iter() {
        if let Some(tile) = regions.tiles.get(region_id) {
            let transform = tile.to_transform(SIZE, GAP).unwrap();
            let region_id = RegionId(tile.id);

            // region rect
            commands
                .spawn()
                .insert_bundle(SpriteBundle::default())
                .insert(transform)
                .insert(RegionRect)
                .insert(region_id);

            if enemy_mark.is_some() {
                //  enemy hp color
                commands
                    .spawn()
                    .insert_bundle(SpriteBundle::default())
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
                commands
                    .spawn_bundle(Text2dBundle {
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
                    .insert(EnemyStatus {
                        name: "怪".to_string(),
                        max_hp: 20,
                        cur_hp: 20,
                    })
                    .insert(EnemyText)
                    .insert(region_id);
            }
        }
    }
}

pub fn update_enemy_hp_text_system(
    asset_server: Res<AssetServer>,
    mut query: Query<
        (&mut Text, &mut EnemyStatus, &RegionId),
        (Changed<EnemyStatus>, With<EnemyText>),
    >,
    mut query_color: Query<(&mut Sprite, &RegionId), With<HPColor>>,
) {
    let font = asset_server.load("fonts/hanti.ttf");
    let text_style = TextStyle {
        font,
        font_size: 18.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    for (mut text, enemy, RegionId(id)) in &mut query.iter_mut() {
        text.set(Box::new(Text::with_section(
            format!("{}\n{}/{}", enemy.name, enemy.cur_hp, enemy.max_hp),
            text_style.clone(),
            text_alignment,
        )))
        .unwrap();
        for (mut sp, RegionId(region_id)) in &mut query_color.iter_mut() {
            if region_id == id {
                sp.color.set_a(enemy.cur_hp as f32 / enemy.max_hp as f32);
            }
        }
    }
}

pub fn region_rect_color_system(
    region_status_query: Query<(&RegionId, &RegionStatus)>,
    mut region_react_query: Query<(&mut Sprite, &RegionId), With<RegionRect>>,
) {
    for (RegionId(region_id), region_status) in region_status_query.iter() {
        for (mut sprite, RegionId(id)) in region_react_query.iter_mut() {
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
                }
            }
        }
    }
}

pub fn mouse_interaction(
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut trigger_region_event: EventWriter<TriggerRegionEvent>,
    q_camera: Query<&Transform, With<Camera>>,
    q_regions: Query<(&RegionId, &Transform), With<Sprite>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        if let Some(wnd) = windows.get_primary() {
            if let Some(pos) = wnd.cursor_position() {
                let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
                let p = pos - size / 2.0;
                if let Ok(camera_transform) = q_camera.get_single() {
                    // apply the camera transform
                    let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
                    for (RegionId(id), region) in q_regions.iter() {
                        let dx = (pos_wld.x - region.translation.x).abs();
                        let dy = (pos_wld.y - region.translation.y).abs();
                        if dx <= region.scale.x / 2. && dy <= region.scale.y / 2. {
                            trigger_region_event.send(TriggerRegionEvent(*id));
                        }
                    }
                }
            }
        }
    }
}

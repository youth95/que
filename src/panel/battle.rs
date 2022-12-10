use bevy::{prelude::*, sprite::Anchor};

use crate::{
    assets::UIImageAssets,
    marks::{EnemyLabel, EnemyStatus},
    panel::BattlePanelVisibly,
    pool::values,
    regions::{CurrentOverRegion, RegionEntityMap, RegionMark},
    GameStage, PlayerStatusType,
};

use super::BattlePanel;
pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStage::Main).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameStage::Main).with_system(update_battle_panel_enemy_label),
            );
    }
}

#[derive(Component)]
enum BattlePanelPart {
    EnemyIcon,
    EnemyHP,
    EnemyDEF,
    EnemyATK,
}

fn setup(
    mut commands: Commands,
    ui_image_assets: Res<UIImageAssets>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/hanti.ttf");

    let with_color = |color: Color| TextStyle {
        font: font.clone_weak(),
        font_size: 16.0,
        color,
    };
    commands
        .spawn(SpriteBundle {
            texture: ui_image_assets.battle_panel.clone_weak(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(208.0, 160.0)),
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        })
        .insert(BattlePanel)
        .insert(BattlePanelVisibly)
        .with_children(|parent| {
            // player
            parent
                .spawn(SpriteBundle {
                    texture: ui_image_assets.icon_hp.clone_weak(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(16.0, -106.0, 1.0)),
                    ..default()
                })
                .insert(BattlePanelVisibly);

            parent
                .spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "HP".to_string(),
                            style: with_color(Color::GREEN),
                        }],
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(42.0, -105.0, 1.0)),
                    ..default()
                })
                .insert(PlayerStatusType::HP)
                .insert(BattlePanelVisibly);

            parent
                .spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "ATK".to_string(),
                            style: with_color(Color::RED),
                        }],
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(42.0, -130.0, 1.0)),
                    ..default()
                })
                .insert(PlayerStatusType::ATK)
                .insert(BattlePanelVisibly);

            parent
                .spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "DEF".to_string(),
                            style: with_color(Color::BLUE),
                        }],
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(122.0, -130.0, 1.0)),
                    ..default()
                })
                .insert(PlayerStatusType::DEF)
                .insert(BattlePanelVisibly);

            // enemy
            parent
                .spawn(SpriteBundle {
                    texture: ui_image_assets.icon_hp.clone_weak(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(16.0, 16.0)),
                        anchor: Anchor::TopLeft,
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(16.0, -16.0, 1.0)),
                    ..default()
                })
                .insert(BattlePanelPart::EnemyIcon)
                .insert(BattlePanelVisibly);

            parent
                .spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "HP".to_string(),
                            style: with_color(Color::GREEN),
                        }],
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(42.0, -16.0, 1.0)),
                    ..default()
                })
                .insert(BattlePanelPart::EnemyHP)
                .insert(BattlePanelVisibly);

            parent
                .spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "ATK".to_string(),
                            style: with_color(Color::RED),
                        }],
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(42.0, -40.0, 1.0)),
                    ..default()
                })
                .insert(BattlePanelPart::EnemyATK)
                .insert(BattlePanelVisibly);

            parent
                .spawn(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "DEF".to_string(),
                            style: with_color(Color::BLUE),
                        }],
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(122.0, -40.0, 1.0)),
                    ..default()
                })
                .insert(BattlePanelPart::EnemyDEF)
                .insert(BattlePanelVisibly);
        });
}

fn update_battle_panel_enemy_label(
    region_mark_query: Query<
        (&EnemyLabel, &EnemyStatus),
        (With<RegionMark>, Without<values::Value>),
    >,
    mut panel_parts_images_query: Query<&mut Handle<Image>, With<BattlePanelPart>>,
    mut panel_parts_text_query: Query<(&mut Text, &BattlePanelPart)>,
    region_entity_map: Res<RegionEntityMap>,
    current_over_region: Res<CurrentOverRegion>,
    asset_server: Res<AssetServer>,
) {
    match current_over_region.as_ref() {
        CurrentOverRegion::None => (),
        CurrentOverRegion::Region(id) => {
            if let Some(entity) = region_entity_map.0.get(id) {
                if let Ok(label) = region_mark_query.get_component::<EnemyLabel>(*entity) {
                    let icon: Handle<Image> = asset_server.get_handle(label.icon.as_str());
                    panel_parts_images_query
                        .get_single_mut()
                        .unwrap()
                        .set(Box::new(icon.clone()))
                        .unwrap();
                }
                if let Ok(status) = region_mark_query.get_component::<EnemyStatus>(*entity) {
                    for (mut text, part) in panel_parts_text_query.iter_mut() {
                        text.sections[0].value = match part {
                            BattlePanelPart::EnemyIcon => "".to_string(),
                            BattlePanelPart::EnemyHP => {
                                format!("{}/{}", status.cur_hp, status.max_hp,)
                            }
                            BattlePanelPart::EnemyDEF => format!("{}", status.def),
                            BattlePanelPart::EnemyATK => format!("{}", status.atk),
                        };
                    }
                }
            }
        }
    };
}

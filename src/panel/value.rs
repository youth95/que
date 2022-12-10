use bevy::{prelude::*, sprite::Anchor};

use crate::{
    assets::UIImageAssets,
    pool::values,
    regions::{CurrentOverRegion, RegionEntityMap, RegionMark},
    GameStage,
};

use super::{ValuePanel, ValuePanelVisibly};
pub struct ValuePanelPlugin;

impl Plugin for ValuePanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStage::Main).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameStage::Main).with_system(update_value_content),
            );
    }
}

#[derive(Component)]
struct ValuePanelText;

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
            texture: ui_image_assets.text_panel.clone_weak(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(208.0, 160.0)),
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        })
        .insert(ValuePanel)
        .insert(ValuePanelVisibly)
        .with_children(|parent| {
            parent
                .spawn(Text2dBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Name".to_string(),
                                style: with_color(Color::WHITE),
                            },
                            TextSection {
                                value: "\nIntro".to_string(),
                                style: with_color(Color::WHITE),
                            },
                            TextSection {
                                value: "\nValues".to_string(),
                                style: with_color(Color::WHITE),
                            },
                        ],
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(12.0, -10.0, 1.0)),
                    ..default()
                })
                .insert(ValuePanelText)
                .insert(ValuePanelVisibly);
        });
}

fn update_value_content(
    region_mark_query: Query<&values::Value, With<RegionMark>>,
    mut panel_query: Query<&mut Text, With<ValuePanelText>>,
    region_entity_map: Res<RegionEntityMap>,
    current_over_region: Res<CurrentOverRegion>,
) {
    match current_over_region.as_ref() {
        CurrentOverRegion::None => (),
        CurrentOverRegion::Region(id) => {
            if let Some(entity) = region_entity_map.0.get(id) {
                if let Ok(value) = region_mark_query.get_component::<values::Value>(*entity) {
                    for mut panel_text in panel_query.iter_mut() {
                        panel_text.sections[0].value = value.name.clone();
                        panel_text.sections[1].value = format!("\n{}\n", value.intro);
                        panel_text.sections[2].value = value
                            .values
                            .iter()
                            .map(|v| match v {
                                values::KeyValue::PlayerCurrentHp(v) => {
                                    format!("\n生命值 {}", v.to_str())
                                }
                                values::KeyValue::PlayerMaxHp(v) => {
                                    format!("\n最大生命值 {}", v.to_str())
                                }
                                values::KeyValue::PlayerAtk(v) => {
                                    format!("\n攻击力 {}", v.to_str())
                                }
                                values::KeyValue::PlayerDef(v) => {
                                    format!("\n防御力 {}", v.to_str())
                                }
                                values::KeyValue::PlayerGold(v) => format!("\n魂 {}", v.to_str()),
                            })
                            .collect::<Vec<_>>()
                            .join("\n");
                    }
                }
            }
        }
    };
}

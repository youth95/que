use bevy::prelude::*;

use crate::{
    assets::MonsterImageAssets,
    marks::{EnemyLabel, EnemyStatus, RegionStatus},
    pool::values,
    regions::{
        events::{MouseOverEmpty, MouseOverRegionEvent},
        CurrentOverRegion, RegionEntityMap, RegionMark,
    },
    GameStage,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<PlayerStatus>()
            .add_system_set(SystemSet::on_enter(GameStage::Main).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameStage::Main)
                    .with_system(update_intro_panel_visible)
                    .with_system(update_current_over_region_empty)
                    .with_system(update_current_over_region)
                    .with_system(update_intro_panel_with_value)
                    .with_system(update_intro_panel_with_enemy)
                    .with_system(update_player_status)
                    .with_system(to_game_over),
            );
    }
}

#[derive(Debug)]
pub struct PlayerStatus {
    pub atk: i64,
    pub def: i64,
    pub cur_hp: i64,
    pub max_hp: i64,
    pub gold: i64,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        Self {
            atk: 10,
            def: 1,
            cur_hp: 100,
            max_hp: 100,
            gold: 10,
        }
    }
}

#[derive(Component)]
struct PlayerStatusHub;

#[derive(Component)]
struct IntroPanel;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(PlayerStatus::default());
    // player status hub
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            color: Color::rgba_u8(0, 0, 0, 254).into(),
            ..default()
        })
        .with_children(|parent| {
            // Rich text with multiple sections
            let hp_style = TextStyle {
                font: asset_server.load("fonts/hanti.ttf"),
                font_size: 18.0,
                color: Color::GREEN,
            };
            let atk_style = TextStyle {
                font: asset_server.load("fonts/hanti.ttf"),
                font_size: 18.0,
                color: Color::RED,
            };
            let def_style = TextStyle {
                font: asset_server.load("fonts/hanti.ttf"),
                font_size: 18.0,
                color: Color::BLUE,
            };
            let gold_style = TextStyle {
                font: asset_server.load("fonts/hanti.ttf"),
                font_size: 18.0,
                color: Color::GOLD,
            };
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "血: ".to_string(),
                                style: hp_style.clone(),
                            },
                            TextSection {
                                value: "".to_string(),
                                style: hp_style.clone(),
                            },
                            TextSection {
                                value: "".to_string(),
                                style: hp_style.clone(),
                            },
                            TextSection {
                                value: "\n攻: ".to_string(),
                                style: atk_style.clone(),
                            },
                            TextSection {
                                value: "".to_string(),
                                style: atk_style.clone(),
                            },
                            TextSection {
                                value: "\n防: ".to_string(),
                                style: def_style.clone(),
                            },
                            TextSection {
                                value: "".to_string(),
                                style: def_style.clone(),
                            },
                            TextSection {
                                value: "\n魂: ".to_string(),
                                style: gold_style.clone(),
                            },
                            TextSection {
                                value: "".to_string(),
                                style: gold_style.clone(),
                            },
                        ],
                        ..Default::default()
                    },

                    ..Default::default()
                })
                .insert(PlayerStatusHub);
        });

    // intro panel
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(30.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                position_type: PositionType::Absolute,
                padding: Rect::all(Val::Px(10.0)),
                position: Rect {
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            color: Color::rgba_u8(0, 0, 0, 224).into(),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(IntroPanel);
}

fn update_player_status(
    player_status: Res<PlayerStatus>,
    mut query_text: Query<&mut Text, With<PlayerStatusHub>>,
) {
    for mut text in query_text.iter_mut() {
        text.sections[1].value = format!("{:.2}", player_status.cur_hp);
        text.sections[2].value = format!("/{:.2}", player_status.max_hp);

        text.sections[4].value = format!("{}", player_status.atk);
        text.sections[6].value = format!("{}", player_status.def);
        text.sections[8].value = format!("{}", player_status.gold);
    }
}

fn update_current_over_region(
    mut mouse_over_region: EventReader<MouseOverRegionEvent>,
    region_query: Query<&RegionStatus, With<RegionMark>>,
    region_entity_map: Res<RegionEntityMap>,
    current_over_region: Res<CurrentOverRegion>,
    mut commands: Commands,
) {
    for MouseOverRegionEvent(id) in mouse_over_region.iter() {
        if let Some(entity) = region_entity_map.0.get(&id) {
            if let Ok(region_status) = region_query.get_component::<RegionStatus>(*entity) {
                match region_status {
                    RegionStatus::Found => match current_over_region.as_ref() {
                        CurrentOverRegion::None => {
                            commands.insert_resource(CurrentOverRegion::Region(*id));
                        }
                        CurrentOverRegion::Region(cur_id) => {
                            if cur_id != id {
                                commands.insert_resource(CurrentOverRegion::Region(*id));
                            }
                        }
                    },
                    _ => match current_over_region.as_ref() {
                        CurrentOverRegion::Region(_) => {
                            commands.insert_resource(CurrentOverRegion::None);
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

fn update_intro_panel_visible(
    current_over_region: Res<CurrentOverRegion>,
    mut panel_query: Query<&mut Visibility, With<IntroPanel>>,
) {
    if current_over_region.is_changed() {
        match current_over_region.as_ref() {
            CurrentOverRegion::None => panel_query.get_single_mut().unwrap().is_visible = false,
            CurrentOverRegion::Region(_) => panel_query.get_single_mut().unwrap().is_visible = true,
        };
    }
}

fn update_current_over_region_empty(
    current_over_region: Res<CurrentOverRegion>,

    mut mouse_over_empty: EventReader<MouseOverEmpty>,
    mut commands: Commands,
) {
    for _ in mouse_over_empty.iter() {
        match current_over_region.as_ref() {
            CurrentOverRegion::Region(_) => {
                commands.insert_resource(CurrentOverRegion::None);
            }
            _ => {}
        }
    }
}

fn update_intro_panel_with_enemy(
    mut panel_query: Query<Entity, With<IntroPanel>>,
    region_mark_query: Query<
        (&EnemyLabel, &EnemyStatus),
        (With<RegionMark>, Without<values::Value>),
    >,
    current_over_region: Res<CurrentOverRegion>,
    region_entity_map: Res<RegionEntityMap>,
    asset_server: ResMut<AssetServer>,
    monster_image_assets: Res<MonsterImageAssets>,
    mut commands: Commands,
) {
    let to_monster_image = |path: &String| -> Handle<Image> {
        if path == "textures/monsters/m0.png" {
            return monster_image_assets.m0.clone();
        }
        if path == "textures/monsters/m1.png" {
            return monster_image_assets.m1.clone();
        }
        if path == "textures/monsters/m2.png" {
            return monster_image_assets.m2.clone();
        }
        if path == "textures/monsters/m3.png" {
            return monster_image_assets.m3.clone();
        }

        return monster_image_assets.m0.clone();
    };
    if current_over_region.is_changed() {
        match current_over_region.into_inner() {
            CurrentOverRegion::None => {
                if let Ok(entity) = panel_query.get_single_mut() {
                    commands.entity(entity).despawn_descendants();
                }
            }
            CurrentOverRegion::Region(region_id) => {
                if let Ok(entity) = panel_query.get_single_mut() {
                    if let Some(region_entity) = region_entity_map.0.get(&region_id) {
                        if let Ok(enemy_label) =
                            region_mark_query.get_component::<EnemyLabel>(*region_entity)
                        {
                            if let Ok(enemy_status) =
                                region_mark_query.get_component::<EnemyStatus>(*region_entity)
                            {
                                commands.entity(entity).despawn_descendants();
                                commands.entity(entity).with_children(|parent| {
                                    parent.spawn_bundle(NodeBundle {
                                        style: Style {
                                            size: Size::new(
                                                Val::Percent(100.0),
                                                Val::Percent(100.0),
                                            ),
                                            ..default()
                                        },
                                        color: Color::NONE.into(),
                                        ..default()
                                    });
                                    let name_style = TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 18.0,
                                        color: Color::WHITE,
                                    };
                                    let atk_style = TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 24.0,
                                        color: Color::RED,
                                    };
                                    let def_style = TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 24.0,
                                        color: Color::BLUE,
                                    };
                                    parent.spawn_bundle(TextBundle {
                                        style: Style {
                                            margin: Rect {
                                                top: Val::Px(20.),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        },
                                        text: Text {
                                            sections: vec![
                                                // TextSection {
                                                //     value: enemy_label.name.clone(),
                                                //     style: name_style.clone(),
                                                // },
                                                TextSection {
                                                    value: format!("\n{}", enemy_label.intro),
                                                    style: name_style.clone(),
                                                },
                                                TextSection {
                                                    value: "\n\n攻: ".to_string(),
                                                    style: atk_style.clone(),
                                                },
                                                TextSection {
                                                    value: format!("{}", enemy_status.atk),
                                                    style: atk_style.clone(),
                                                },
                                                TextSection {
                                                    value: "    防: ".to_string(),
                                                    style: def_style.clone(),
                                                },
                                                TextSection {
                                                    value: format!("{}", enemy_status.def),
                                                    style: def_style.clone(),
                                                },
                                            ],
                                            ..Default::default()
                                        },

                                        ..Default::default()
                                    });
                                    parent.spawn_bundle(ImageBundle {
                                        style: Style {
                                            align_self: AlignSelf::Stretch,
                                            max_size: Size::new(
                                                Val::Percent(100.0),
                                                Val::Px(200.0),
                                            ),
                                            ..Default::default()
                                        },
                                        image: to_monster_image(&enemy_label.image_label).into(),

                                        ..Default::default()
                                    });
                                });
                            }
                        }
                    }
                }
            }
        };
    }
}

fn update_intro_panel_with_value(
    mut panel_query: Query<Entity, With<IntroPanel>>,
    region_mark_query: Query<&values::Value, With<RegionMark>>,
    current_over_region: Res<CurrentOverRegion>,
    region_entity_map: Res<RegionEntityMap>,
    asset_server: ResMut<AssetServer>,
    monster_image_assets: Res<MonsterImageAssets>,
    mut commands: Commands,
) {
    let to_monster_image = |path: &String| -> Handle<Image> {
        if path == "textures/monsters/m0.png" {
            return monster_image_assets.m0.clone();
        }
        if path == "textures/monsters/m1.png" {
            return monster_image_assets.m1.clone();
        }
        if path == "textures/monsters/m2.png" {
            return monster_image_assets.m2.clone();
        }
        if path == "textures/monsters/m3.png" {
            return monster_image_assets.m3.clone();
        }
        return monster_image_assets.empty.clone();
    };
    if current_over_region.is_changed() {
        match current_over_region.into_inner() {
            CurrentOverRegion::None => {
                if let Ok(entity) = panel_query.get_single_mut() {
                    commands.entity(entity).despawn_descendants();
                }
            }
            CurrentOverRegion::Region(region_id) => {
                if let Ok(entity) = panel_query.get_single_mut() {
                    if let Some(region_entity) = region_entity_map.0.get(&region_id) {
                        if let Ok(value) =
                            region_mark_query.get_component::<values::Value>(*region_entity)
                        {
                            commands.entity(entity).despawn_descendants();
                            commands.entity(entity).with_children(|parent| {
                                parent.spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                        ..default()
                                    },
                                    color: Color::NONE.into(),
                                    ..default()
                                });
                                let name_style = TextStyle {
                                    font: asset_server.load("fonts/hanti.ttf"),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                };
                                let mut sections = value
                                    .values
                                    .iter()
                                    .map(|v| match v {
                                        values::KeyValue::PlayerCurrentHp(val) => TextSection {
                                            value: format!("\n 获得 生命值 {}", val.to_str()),
                                            style: name_style.clone(),
                                        },
                                        values::KeyValue::PlayerMaxHp(val) => TextSection {
                                            value: format!("\n 获得 最大生命值 {}", val.to_str()),
                                            style: name_style.clone(),
                                        },
                                        values::KeyValue::PlayerAtk(val) => TextSection {
                                            value: format!("\n 获得 攻击力 {}", val.to_str()),
                                            style: name_style.clone(),
                                        },
                                        values::KeyValue::PlayerDef(val) => TextSection {
                                            value: format!("\n 获得 防御力 {}", val.to_str()),
                                            style: name_style.clone(),
                                        },
                                        values::KeyValue::PlayerGold(val) => TextSection {
                                            value: format!("\n 获得 魂 {}", val.to_str()),
                                            style: name_style.clone(),
                                        },
                                    })
                                    .collect::<Vec<_>>();
                                sections.insert(
                                    0,
                                    TextSection {
                                        value: format!("\n{}\n\n", value.intro),
                                        style: name_style.clone(),
                                    },
                                );

                                parent.spawn_bundle(TextBundle {
                                    style: Style {
                                        margin: Rect {
                                            top: Val::Px(20.),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    text: Text {
                                        sections,
                                        ..Default::default()
                                    },

                                    ..Default::default()
                                });
                                parent.spawn_bundle(ImageBundle {
                                    style: Style {
                                        align_self: AlignSelf::Stretch,
                                        max_size: Size::new(Val::Percent(100.0), Val::Px(200.0)),
                                        ..Default::default()
                                    },
                                    image: to_monster_image(&value.image_label).into(),

                                    ..Default::default()
                                });
                            });
                        }
                    }
                }
            }
        };
    }
}

fn to_game_over(player_status: Res<PlayerStatus>, mut game_stage: ResMut<State<GameStage>>) {
    if player_status.is_changed() {
        if player_status.cur_hp <= 0 {
            game_stage.set(GameStage::GameOver).unwrap();
        }
    }
}

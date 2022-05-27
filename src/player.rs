use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBuilder, NinePatchBundle, NinePatchData};

use crate::{
    assets::UIImageAssets,
    marks::RegionStatus,
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
                    // .with_system(update_intro_panel_visible)
                    .with_system(update_current_over_region_empty)
                    .with_system(update_current_over_region)
                    // .with_system(update_intro_panel_with_value)
                    // .with_system(update_intro_panel_with_enemy)
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

#[derive(Component)]
pub enum PlayerStatusType {
    ATK,
    DEF,
    HP,
    GOLD,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_image: Res<UIImageAssets>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());
    commands.insert_resource(PlayerStatus::default());
    // player status hub
    // TODO 加底图
    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(10, 10, 20, 20));

    let status_hub_entity = commands
        .spawn_bundle(NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                justify_content: JustifyContent::SpaceBetween,
                padding: Rect::all(Val::Px(5.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(16.0), Val::Px(16.0)),
                            margin: Rect {
                                right: Val::Px(4.0),
                                ..default()
                            },
                            ..default()
                        },
                        image: UiImage(ui_image.icon_hp.clone_weak()),
                        ..default()
                    });
                    // hp
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(16.0)),
                                min_size: Size::new(Val::Px(80.0), Val::Px(16.0)),
                                ..default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 16.0,
                                        color: Color::GREEN.into(),
                                        ..default()
                                    },
                                    value: "100/100".to_string(),
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(PlayerStatusType::HP);
                });

            parent
                .spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(16.0), Val::Px(16.0)),
                            margin: Rect {
                                right: Val::Px(4.0),
                                ..default()
                            },
                            ..default()
                        },
                        image: UiImage(ui_image.icon_atk.clone_weak()),
                        ..default()
                    });
                    // atk
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(16.0)),
                                min_size: Size::new(Val::Px(34.0), Val::Px(16.0)),
                                ..default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 16.0,
                                        color: Color::RED.into(),
                                        ..default()
                                    },
                                    value: "80".to_string(),
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(PlayerStatusType::ATK);

                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(16.0), Val::Px(16.0)),
                            margin: Rect {
                                right: Val::Px(4.0),
                                ..default()
                            },
                            ..default()
                        },
                        image: UiImage(ui_image.icon_def.clone_weak()),
                        ..default()
                    });
                    // def
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(16.0)),
                                min_size: Size::new(Val::Px(34.0), Val::Px(16.0)),
                                ..default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 16.0,
                                        color: Color::BLUE.into(),
                                        ..default()
                                    },
                                    value: "80".to_string(),
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(PlayerStatusType::DEF);
                });

            parent
                .spawn_bundle(NodeBundle {
                    color: Color::NONE.into(),
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(16.0), Val::Px(16.0)),
                            margin: Rect {
                                right: Val::Px(4.0),
                                ..default()
                            },
                            ..default()
                        },
                        image: UiImage(ui_image.icon_gold.clone_weak()),
                        ..default()
                    });
                    // gold
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                size: Size::new(Val::Auto, Val::Px(16.0)),
                                min_size: Size::new(Val::Px(34.0), Val::Px(16.0)),
                                ..default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    style: TextStyle {
                                        font: asset_server.load("fonts/hanti.ttf"),
                                        font_size: 16.0,
                                        color: Color::GOLD.into(),
                                        ..default()
                                    },
                                    value: "80".to_string(),
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(PlayerStatusType::GOLD);
                });
        })
        .id();

    commands.spawn_bundle(NinePatchBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Px(32.0)),
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            ..default()
        },
        nine_patch_data: NinePatchData::with_single_content(
            ui_image.status_hub_panel.clone_weak(),
            nine_patch_handle,
            status_hub_entity,
        ),
        ..Default::default()
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
    mut query_text: Query<(&mut Text, &PlayerStatusType)>,
) {
    for (mut text, status) in query_text.iter_mut() {
        text.sections[0].value = match status {
            PlayerStatusType::ATK => format!("{}", player_status.atk),
            PlayerStatusType::DEF => format!("{}", player_status.def),
            PlayerStatusType::HP => format!("{}/{}", player_status.cur_hp, player_status.max_hp),
            PlayerStatusType::GOLD => format!("{}", player_status.gold),
        };
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

fn to_game_over(player_status: Res<PlayerStatus>, mut game_stage: ResMut<State<GameStage>>) {
    if player_status.is_changed() {
        if player_status.cur_hp <= 0 {
            game_stage.set(GameStage::GameOver).unwrap();
        }
    }
}

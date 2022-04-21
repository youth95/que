use bevy::prelude::*;

use crate::{
    marks::{RegionId, RegionStatus},
    regions::{
        events::{MouseOverEmpty, MouseOverRegionEvent},
        RegionMark,
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(setup)
            // .add_system(text_update_system)
            .add_system(intro_panel_visible)
            .add_system(debug);
    }
}

#[derive(Component, Debug)]
pub struct Player {
    pub atk: i64,
    pub def: i64,
    pub cur_hp: i64,
    pub max_hp: i64,
    pub gold: i64,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            atk: 10,
            def: 10,
            cur_hp: 100,
            max_hp: 100,
            gold: 10,
        }
    }
}

#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct IntroPanel;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn().insert(Player::default());

    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());

    // player status hub
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Percent(100.0)),
                // justify_content: JustifyContent::SpaceBetween,
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
                .insert(FpsText);
        });

    // intro panel
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            color: Color::rgba_u8(0, 0, 0, 254).into(),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(IntroPanel);
}

// fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
//     for mut text in query.iter_mut() {
//         if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
//             if let Some(average) = fps.average() {
//                 // Update the value of the second section
//                 text.sections[1].value = format!("{:.2}", average);
//             }
//         }
//     }
// }

fn debug(query: Query<&Player, Changed<Player>>, mut query_text: Query<&mut Text, With<FpsText>>) {
    for player in query.iter() {
        for mut text in query_text.iter_mut() {
            text.sections[1].value = format!("{:.2}", player.cur_hp);
            text.sections[2].value = format!("/{:.2}", player.max_hp);

            text.sections[4].value = format!("{}", player.atk);
            text.sections[6].value = format!("{}", player.def);
            text.sections[8].value = format!("{}", player.gold);
        }
    }
}

fn intro_panel_visible(
    mut mouse_over_region: EventReader<MouseOverRegionEvent>,
    mut mouse_over_empty: EventReader<MouseOverEmpty>,
    region_query: Query<(&RegionStatus, &RegionId), With<RegionMark>>,
    mut query: Query<&mut Visibility, With<IntroPanel>>,
) {
    for MouseOverRegionEvent(id) in mouse_over_region.iter() {
        region_query
            .iter()
            .for_each(|(status, RegionId(region_id))| {
                if region_id == id {
                    match status {
                        RegionStatus::Found => {
                            query.iter_mut().for_each(|mut visibility| {
                                visibility.is_visible = true;
                            });
                        }
                        _ => {
                            query.iter_mut().for_each(|mut visibility| {
                                visibility.is_visible = false;
                            });
                        }
                    };
                }
            });
    }
    for _ in mouse_over_empty.iter() {
        query.iter_mut().for_each(|mut visibility| {
            visibility.is_visible = false;
        });
    }
}

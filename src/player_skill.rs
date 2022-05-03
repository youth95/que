use std::collections::LinkedList;

use bevy::prelude::*;

use crate::{
    assets::{FontAssets, UIImageAssets},
    marks::EnemyStatus,
    player::PlayerStatus,
    regions::{
        events::{AtkMonsterWithPlayerSkill, AudioSound, PlayAudioEvent},
        ChangeEnemyHpEvent, RegionEntityMap,
    },
    GameStage,
};

pub struct PlayerSkillPlugin;

impl Plugin for PlayerSkillPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSkills>().add_system_set(
            SystemSet::on_update(GameStage::Main)
                .with_system(update_player_skill)
                .with_system(update_player_skills_order),
        );
    }
}

#[derive(Component)]
pub struct SkillPanelPart;

pub struct PlayerSkills {
    skills: LinkedList<Skill>,
}

#[derive(Clone)]
struct Skill {
    name: String,
    description: String,
    icon: String,
    event: SkillEvent,
    cd: u32,       // 冷却时间
    cd_timer: u32, // 剩余冷却时间
}

#[derive(Clone)]
enum SkillEvent {
    CommonAttack,
    DoubleAttack,
}

impl Default for PlayerSkills {
    fn default() -> Self {
        let mut player_skills = Self {
            skills: Default::default(),
        };
        player_skills.skills.push_back(Skill {
            name: "普通攻击".to_string(),
            description: "朴实无华的一次攻击, 造成等同于攻击力的伤害".to_string(),
            icon: "textures/ui/skill_icons/skill_icon1.png".to_string(),
            event: SkillEvent::CommonAttack,
            cd: 0,
            cd_timer: 0,
        });
        player_skills.skills.push_back(Skill {
            name: "会心攻击".to_string(),
            description: "认真的一次攻击, 造成等同于攻击力2倍的伤害".to_string(),
            icon: "textures/ui/skill_icons/skill_icon2.png".to_string(),
            event: SkillEvent::DoubleAttack,
            cd: 1,
            cd_timer: 1,
        });
        player_skills
    }
}

pub fn update_player_skill(
    mut commands: Commands,
    self_query: Query<Entity, With<SkillPanelPart>>,
    player_skills: Res<PlayerSkills>,
    ui_assets: Res<UIImageAssets>,
    font_assets: Res<FontAssets>,
    asset_server: Res<AssetServer>,
) {
    if player_skills.is_changed() {
        for entity in self_query.iter() {
            commands.entity(entity).despawn();
        }
        let skill_count = player_skills.skills.len();

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Px(48.)),
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Val::Px(10.),
                        left: Val::Px(0.),
                        ..default()
                    },
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(1.5, 1.5, 1.)),
                color: Color::NONE.into(),
                ..default()
            })
            .insert(SkillPanelPart)
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(34.0 * skill_count as f32), Val::Px(44.)),
                            justify_content: JustifyContent::SpaceBetween,
                            align_self: AlignSelf::Center,
                            margin: Rect::all(Val::Auto),
                            ..default()
                        },
                        color: Color::NONE.into(),
                        ..default()
                    })
                    .insert(SkillPanelPart)
                    .with_children(|parent| {
                        for (i, skill) in player_skills.skills.iter().enumerate() {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(32.), Val::Px(44.)),
                                        flex_direction: FlexDirection::ColumnReverse,
                                        position_type: PositionType::Relative,
                                        ..default()
                                    },
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .insert(SkillPanelPart)
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(ImageBundle {
                                            style: Style {
                                                size: Size::new(Val::Px(32.), Val::Px(32.)),
                                                ..default()
                                            },
                                            image: UiImage(if i == 0 {
                                                ui_assets.skill_block_active.clone_weak()
                                            } else {
                                                ui_assets.skill_block_normal.clone_weak()
                                            }),
                                            ..default()
                                        })
                                        .insert(SkillPanelPart)
                                        .with_children(|parent| {
                                            let icon = asset_server.get_handle(skill.icon.as_str());
                                            parent
                                                .spawn_bundle(ImageBundle {
                                                    style: Style {
                                                        size: Size::new(Val::Px(24.), Val::Px(24.)),
                                                        // position_type: PositionType::Absolute,
                                                        // position: Rect {
                                                        //     top: Val::Px(0.),
                                                        //     left: Val::Px(0.),
                                                        //     ..default()
                                                        // },
                                                        margin: Rect::all(Val::Auto),
                                                        ..default()
                                                    },
                                                    image: icon.into(),
                                                    ..default()
                                                })
                                                .insert(SkillPanelPart);
                                        });

                                    parent
                                        .spawn_bundle(TextBundle {
                                            style: Style {
                                                size: Size::new(Val::Auto, Val::Px(16.)),
                                                align_self: AlignSelf::Center,
                                                ..default()
                                            },
                                            text: Text {
                                                sections: vec![TextSection {
                                                    value: if skill.cd_timer > 0 {
                                                        format!("{}", skill.cd_timer)
                                                    } else {
                                                        "".to_string()
                                                    },
                                                    style: TextStyle {
                                                        font: font_assets.bold.clone_weak(),
                                                        font_size: 16.,
                                                        ..default()
                                                    },
                                                }],
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .insert(SkillPanelPart);
                                });
                        }
                    });
            });
    }
}

pub fn update_player_skills_order(
    mut atk_monster_with_player_skill: EventReader<AtkMonsterWithPlayerSkill>,
    mut player_skills: ResMut<PlayerSkills>,

    mut player_status: ResMut<PlayerStatus>,
    query: Query<&EnemyStatus>,
    // mut trigger_region_event: EventReader<RegionClickEvent>,
    region_entity_map: Res<RegionEntityMap>,
    mut change_enemy_hp_event: EventWriter<ChangeEnemyHpEvent>,
    mut play_audio_event: EventWriter<PlayAudioEvent>,
) {
    for AtkMonsterWithPlayerSkill(id) in atk_monster_with_player_skill.iter() {
        let mut triggered = false;
        loop {
            if let Some(mut skill) = player_skills.skills.pop_front() {
                let skill = &mut skill;
                if skill.cd_timer == 0 {
                    if triggered == false {
                        if let Some(en) = region_entity_map.0.get(id) {
                            if let Ok(enemy) = query.get_component::<EnemyStatus>(*en) {
                                match skill.event {
                                    SkillEvent::CommonAttack => {
                                        change_enemy_hp_event.send(ChangeEnemyHpEvent(
                                            *id,
                                            -(player_status.atk - enemy.def),
                                        ));
                                    }
                                    SkillEvent::DoubleAttack => {
                                        change_enemy_hp_event.send(ChangeEnemyHpEvent(
                                            *id,
                                            -(player_status.atk * 2 - enemy.def),
                                        ));
                                    }
                                };
                                player_status.cur_hp -= (enemy.atk - player_status.def).max(0);
                            }
                        }
                        play_audio_event.send(PlayAudioEvent(AudioSound::Dao5));
                        skill.cd_timer = skill.clone().cd;
                        player_skills.skills.push_back(skill.clone());
                        triggered = true
                    } else {
                        player_skills.skills.push_front(skill.clone());
                        break;
                    }
                } else {
                    skill.cd_timer -= 1;
                    player_skills.skills.push_back(skill.clone());
                }
            }
        }
    }
}

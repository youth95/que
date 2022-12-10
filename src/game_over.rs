use bevy::prelude::*;

use crate::GameStage;
pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStage::GameOver).with_system(cleanup))
            .add_system_set(SystemSet::on_update(GameStage::GameOver).with_system(again_system));
    }
}

fn cleanup(asset_server: Res<AssetServer>, query: Query<Entity>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    commands.spawn(Camera2dBundle::default());

    let style = TextStyle {
        font: asset_server.load("fonts/hanti.ttf"),
        font_size: 120.0,
        color: Color::RED,
    };

    let btn_style = TextStyle {
        font: asset_server.load("fonts/hanti.ttf"),
        font_size: 60.0,
        color: Color::RED,
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(20.)),
                        ..Default::default()
                    },
                    background_color: Color::GREEN.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        style: Style {
                            ..Default::default()
                        },
                        text: Text {
                            sections: vec![TextSection {
                                value: "再来亿次".to_string(),
                                style: btn_style.clone(),
                                ..Default::default()
                            }],
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            parent.spawn(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "Game Over".to_string(),
                        style: style.clone(),
                        ..Default::default()
                    }],
                    alignment: TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

fn again_system(
    query: Query<Entity>,
    mut interaction_query: Query<&Interaction, (With<Interaction>, With<Button>)>,
    mut game_stage: ResMut<State<GameStage>>,
    mut commands: Commands,
) {
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                for entity in query.iter() {
                    commands.entity(entity).despawn();
                }
                game_stage.set(GameStage::Main).unwrap();
            }
            _ => (),
        };
    }
}

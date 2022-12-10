use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::GameStage;
pub struct CameraPlugin;

#[derive(Component)]
pub struct SceneCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameStage::Main).with_system(setup));
        app.add_system(motion);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(SceneCamera);
}

fn motion(
    mut camera_query: Query<&mut Transform, With<SceneCamera>>,
    mut motion_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
) {
    if buttons.pressed(MouseButton::Right) {
        for ev in motion_evr.iter() {
            for mut camera in camera_query.iter_mut() {
                camera.translation.x -= ev.delta.x;
                camera.translation.y += ev.delta.y;
            }
        }
    }
}

use bevy::{input::mouse::MouseMotion, prelude::*};
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup).add_system(motion);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn motion(
    mut camera_query: Query<&mut Transform, With<Camera>>,
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

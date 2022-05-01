mod battle;
mod value;

use bevy::prelude::*;

use crate::{
    marks::RegionStatus,
    regions::{CurrentOverRegion, RegionEntityMap, RegionMark, WorldMouse},
    GameStage,
};

use self::{battle::BattlePlugin, value::ValuePanelPlugin};
pub struct PanelPlugin;

impl Plugin for PanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BattlePlugin)
            .add_plugin(ValuePanelPlugin)
            .add_system_set(
                SystemSet::on_update(GameStage::Main)
                    .with_system(update_panel_pos::<BattlePanel>)
                    .with_system(update_region_panel_visibly::<BattlePanelVisibly, HasBattlePanel>)
                    .with_system(update_panel_pos::<ValuePanel>)
                    .with_system(update_region_panel_visibly::<ValuePanelVisibly, HasValuePanel>),
            );
    }
}

fn update_region_panel_visibly<P: Component, C: Component>(
    current_over_region: Res<CurrentOverRegion>,
    mut panel_query: Query<&mut Visibility, With<P>>,
    region_mark_status_query: Query<&RegionStatus, (With<RegionMark>, With<C>)>,
    region_entity_map: Res<RegionEntityMap>,
) {
    if current_over_region.is_changed() {
        let mut set = |v: bool| {
            for mut visibility in panel_query.iter_mut() {
                visibility.is_visible = v;
            }
        };
        match current_over_region.as_ref() {
            CurrentOverRegion::None => set(false),
            CurrentOverRegion::Region(id) => {
                info!("{}", id);
                if let Some(entity) = region_entity_map.0.get(id) {
                    if let Ok(status) =
                        region_mark_status_query.get_component::<RegionStatus>(*entity)
                    {
                        match status {
                            RegionStatus::Found => set(true),
                            _ => set(false),
                        }
                        return;
                    }
                }
                set(false);
            }
        };
    }
}

pub fn update_panel_pos<P: Component>(
    mut query: Query<&mut Transform, With<P>>,
    world_mouse: Res<WorldMouse>,
) {
    if world_mouse.is_changed() {
        for mut transform in query.iter_mut() {
            let mut pos = world_mouse.0.clone();
            pos.z = 99.;
            pos.x += 20.;
            transform.translation = pos;
            transform.scale = Vec3::new(1.5, 1.5, 1.0);
        }
    }
}

#[derive(Component)]
pub struct BattlePanel;

#[derive(Component)]
pub struct HasBattlePanel;

#[derive(Component)]
pub struct BattlePanelVisibly;

#[derive(Component)]
pub struct ValuePanel;

#[derive(Component)]
pub struct HasValuePanel;

#[derive(Component)]
pub struct ValuePanelVisibly;

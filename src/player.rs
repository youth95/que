use bevy::prelude::Plugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Player>();
    }
}

pub struct Player {
    pub atk: i64,
    pub def: i64,
    pub cur_hp: i64,
    pub max_hp: i64,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            atk: 10,
            def: 10,
            cur_hp: 100,
            max_hp: 100,
        }
    }
}

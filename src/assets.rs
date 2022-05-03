use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "sounds/click.wav")]
    pub click: Handle<AudioSource>,
    #[asset(path = "sounds/dao5.mp3")]
    pub dao5: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/hanti.ttf")]
    pub hanti: Handle<Font>,

    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub bold: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct MonsterImageAssets {
    #[asset(path = "textures/monsters/m0.png")]
    pub m0: Handle<Image>,
    #[asset(path = "textures/monsters/m1.png")]
    pub m1: Handle<Image>,
    #[asset(path = "textures/monsters/m2.png")]
    pub m2: Handle<Image>,
    #[asset(path = "textures/monsters/m3.png")]
    pub m3: Handle<Image>,

    #[asset(path = "textures/values/empty.png")]
    pub empty: Handle<Image>,

    #[asset(path = "textures/monster_icons", folder(typed))]
    pub icons: Vec<Handle<Image>>,
}

#[derive(AssetCollection)]
pub struct UIImageAssets {
    #[asset(path = "textures/ui/icons/icon_ATK.png")]
    pub icon_atk: Handle<Image>,
    #[asset(path = "textures/ui/icons/icon_DEF.png")]
    pub icon_def: Handle<Image>,
    #[asset(path = "textures/ui/icons/icon_Gold.png")]
    pub icon_gold: Handle<Image>,
    #[asset(path = "textures/ui/icons/icon_HP.png")]
    pub icon_hp: Handle<Image>,

    #[asset(path = "textures/ui/components/status_hub_panel.png")]
    pub status_hub_panel: Handle<Image>,

    #[asset(path = "textures/ui/components/battle_panel.png")]
    pub battle_panel: Handle<Image>,

    #[asset(path = "textures/ui/components/text_panel.png")]
    pub text_panel: Handle<Image>,

    #[asset(path = "textures/ui/icons/cursor1.png")]
    pub icon_cursor1: Handle<Image>,

    #[asset(path = "textures/ui/icons/cursor2.png")]
    pub icon_cursor2: Handle<Image>,

    #[asset(path = "textures/ui/components/skill_block_normal.png")]
    pub skill_block_normal: Handle<Image>,

    #[asset(path = "textures/ui/components/skill_block_active.png")]
    pub skill_block_active: Handle<Image>,

    #[asset(path = "textures/ui/skill_icons", folder(typed))]
    pub _skill_icons: Vec<Handle<Image>>,
}

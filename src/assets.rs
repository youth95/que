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
pub struct MonsterImageAssets {
    #[asset(path = "textures/monsters/m0.png")]
    pub m0: Handle<Image>,
    #[asset(path = "textures/monsters/m1.png")]
    pub m1: Handle<Image>,
    #[asset(path = "textures/monsters/m2.png")]
    pub m2: Handle<Image>,
    #[asset(path = "textures/monsters/m3.png")]
    pub m3: Handle<Image>,
}

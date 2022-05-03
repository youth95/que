use crate::marks::RegionStatus;
// 点击某个区域，触发事件
pub struct RegionClickEvent(pub u64);

// 鼠标悬停在某个区域上，触发事件
pub struct MouseOverRegionEvent(pub u64);

// 鼠标悬停在空白区域上
pub struct MouseOverEmpty;

// 音效播放事件
pub struct PlayAudioEvent(pub AudioSound);

pub enum AudioSound {
    Click,
    Dao5,
}

// 敌人血量变化事件
pub struct ChangeEnemyHpEvent(pub u64, pub i64);

// 区域状态改变事件
#[derive(Debug)]
pub struct ChangeRegionStatusEvent(pub u64, pub RegionStatus);


pub struct AtkMonsterWithPlayerSkill(pub u64);

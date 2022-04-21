use crate::marks::RegionStatus;
// 点击某个区域，触发事件
pub struct RegionClickEvent(pub u64);

// 音效播放事件
pub struct PlayAudioEvent(pub String);

// 敌人血量变化事件
pub struct ChangeEnemyHpEvent(pub u64, pub i64);

// 区域状态改变事件
#[derive(Debug)]
pub struct ChangeRegionStatusEvent(pub u64, pub RegionStatus);

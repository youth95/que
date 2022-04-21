use crate::marks::RegionStatus;

pub struct TriggerRegionEvent(pub u64);

pub struct ChangeEnemyHpEvent(pub u64, pub i64);

#[derive(Debug)]
pub struct ChangeRegionStatusEvent(pub u64, pub RegionStatus);

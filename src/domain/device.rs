// Domain entity

#[derive(Debug, Clone)]
pub struct Device {
    pub hw: String,
    pub sn: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub unix_epoch: u64,
    pub custom: String,
}

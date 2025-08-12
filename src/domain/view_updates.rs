use crate::domain::Device;

pub trait ViewUpdates {
    fn display_devices(&self, devices: &[Device]);
}

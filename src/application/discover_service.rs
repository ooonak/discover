use std::slice;

use crate::domain::{Device, DeviceListener, ViewUpdates};

pub struct DiscoverService<'a> {
    ui: &'a dyn ViewUpdates,
}

impl<'a> DiscoverService<'a> {
    pub fn new(ui: &'a dyn ViewUpdates) -> Self {
        Self { ui }
    }
}

impl<'a> DeviceListener for DiscoverService<'a> {
    fn on_device_discovered(&self, device: Device) {
        
        println!("DiscoverService (application), new device from driving port (DeviceListener), forwarding to driven port (Viewupdates): {:?}", device);
        
        self.ui.display_devices(slice::from_ref(&device));
    }
}

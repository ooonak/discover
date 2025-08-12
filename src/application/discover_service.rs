use std::slice;

use crate::domain::{Device, DeviceListener, ViewUpdates};

pub struct DiscoverService<'a, T: ViewUpdates + 'a> {
    ui: &'a T,
}

impl<'a, T: ViewUpdates + 'a> DiscoverService<'a, T> {
    pub fn new(ui: &'a T) -> Self {
        Self { ui }
    }
}

impl<'a, T: ViewUpdates + 'a> DeviceListener for DiscoverService<'a, T> {
    fn on_device_discovered(&self, device: Device) {
        
        println!("DiscoverService (application), new device from driving port (DeviceListener), forwarding to driven port (Viewupdates): {:?}", device);
        
        self.ui.display_devices(slice::from_ref(&device));
    }
}

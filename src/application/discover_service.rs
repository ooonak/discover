use std::slice;
use log::info;

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
        
        info!("From driving port (DeviceListener), forwarding to driven port (Viewupdates)");
        
        self.ui.display_devices(slice::from_ref(&device));
    }
}

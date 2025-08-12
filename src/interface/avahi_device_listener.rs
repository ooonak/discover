use log::info;

use crate::domain::{Device, DeviceListener};

// An adapter, a specific implementation for a driving port.
pub struct AvahiDeviceListener<'a, T: DeviceListener + 'a> {
    listener: &'a T,
}

impl<'a, T: DeviceListener + 'a> AvahiDeviceListener<'a, T> {
    pub fn new(device_listener: &'a T) -> Self {
        Self { listener: device_listener }
    }

    pub fn listen(&self) {
        // When Avahi message comes in:
        let device = Device { hw: String::from("deadbeef"), sn: String::from("12345678") };
        info!("New device event {:?}", device);
        self.listener.on_device_discovered(device);
    }
}
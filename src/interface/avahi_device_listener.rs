use crate::domain::{Device, DeviceListener};

// An adapter, a specific implementation for a driving port.
pub struct AvahiDeviceListener<'a> {
    handler: &'a dyn DeviceListener,
}

impl<'a> AvahiDeviceListener<'a> {
    pub fn new(handler: &'a dyn DeviceListener) -> Self {
        Self { handler }
    }

    pub fn listen(&self) {
        // When Avahi message comes in:
        let device = Device { hw: String::from("deadbeef"), sn: String::from("12345678") };
        self.handler.on_device_discovered(device);
    }
}
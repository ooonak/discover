use crate::domain::Device;

/* The interface defining an incomming, driving port, towards the domain */
pub trait DeviceListener {
    fn on_device_discovered(&self, device: Device);
}

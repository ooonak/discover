use zbus::Result;

mod domain;
mod application;
mod interface;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let ui = interface::SimpleConsoleUi{};
    let discover_service = application::DiscoverService::new(&ui);
    let avahi_device_listener = interface::AvahiDeviceListener::new(&discover_service);
    
    avahi_device_listener.listen().await?;

    Ok(())
}

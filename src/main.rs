use zbus::Result;

mod application;
mod domain;
mod interface;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let ui = interface::SimpleConsoleUi {};
    let discover_service = application::DiscoverService::new(&ui);
    let avahi_device_listener = interface::AvahiDeviceListener::new(&discover_service);

    log::debug!("Before");
    avahi_device_listener.listen().await?;
    log::debug!("After");
    Ok(())
}

use log::info;
use zbus::{Connection, Proxy};
use std::result::Result;
use futures::stream::StreamExt;
use crate::{domain::DeviceListener};

// Just inline the code under its own module
include!("server2.rs");

// An adapter, a specific implementation for a driving port.
pub struct AvahiDeviceListener<'a, T: DeviceListener + 'a> {
    listener: &'a T,
}

impl<'a, T: DeviceListener + 'a> AvahiDeviceListener<'a, T> {
    pub fn new(device_listener: &'a T) -> Self {
        Self { listener: device_listener }
    }

    pub async fn listen(&self) -> Result<(), zbus::Error> {
        let connection = Connection::system().await?;

        // Connect to Avahi server
        //let server_proxy = Server2Proxy::new(&connection).await?;
        
        let service_type = "_discover._tcp";

        // Ask the Avahi daemon to start browsing for services of a given type, interface, and domain.
        // -1 as interface, protocol = 0 = AVAHI_PROTO_UNSPEC
        // Type = "e.g. _http._tcp", domain = "", flags = 0
        let path = server_proxy
                .service_browser_prepare(-1, -1, service_type, "local", 0)
                .await?;
    
        // Create a proxy for that new object path with interface org.freedesktop.Avahi.ServiceBrowser.
        // The ServiceBrowser proxy represents a remote D-Bus object.
        // This object emits signals like: ItemNew, ItemRemove
        let browser_proxy = Proxy::new(
            &connection,
            "org.freedesktop.Avahi",
            path.as_str(),
            "org.freedesktop.Avahi.ServiceBrowser",
        )
        .await?;

    // Subscribe to ItemNew signals
    let item_new_signals = browser_proxy.receive_signal("ItemNew").await?;
    let item_remove_signals = browser_proxy.receive_signal("ItemRemove").await?;

    // Merge the two streams into one
    let mut combined = futures::stream::select(item_new_signals, item_remove_signals);

    while let Some(signal) = combined.next().await {
        let msg = signal;
        match msg.header().member().map(|m| m.as_str()) {
            // We typically get two events, one for IPv4 and IPv6.
            Some("ItemNew") => {
                let (interface, protocol, name, stype, domain, flags): (
                    i32,
                    i32,
                    String,
                    String,
                    String,
                    u32,
                ) = msg.body().deserialize()?;

                log::info!(
                    "New service: {:?}",
                    (
                        interface,
                        protocol,
                        name.clone(),
                        stype,
                        domain.clone(),
                        flags
                    )
                );

                // Resolve it
                // We call ResolveService manually with the fields we got from ItemNew, link is by data (name, type, domain).
                let resolver_proxy = Proxy::new(
                    &connection,
                    "org.freedesktop.Avahi",
                    "/",
                    "org.freedesktop.Avahi.Server",
                )
                .await?;

                // Call ResolveService asynchronously
                let resolved: (
                    i32,          // interface
                    i32,          // protocol
                    String,       // name
                    String,       // type
                    String,       // domain
                    String,       // host
                    i32,          // address protocol
                    String,       // address (e.g., "192.168.1.123" or "fe80::...")
                    u16,          // port
                    Vec<Vec<u8>>, // TXT records
                    u32,          // flags
                ) = resolver_proxy
                    .call(
                        "ResolveService",
                        &(
                            interface,
                            protocol,
                            name,
                            service_type,
                            domain,
                            -1i32, // protocol to resolve
                            0u32,  // flags
                        ),
                    )
                    .await?;

                let (
                    if_idx,
                    proto,
                    name,
                    stype,
                    domain,
                    host_name,
                    addr_proto,
                    address,
                    port,
                    txt_records,
                    flags,
                ) = resolved;

                println!(
                    "Resolved: if_idx: {if_idx}, proto: {proto}, name: {name}, stype: {stype}, domain: {domain}, host_name: {host_name}, addr_proto: {addr_proto}, address: {address}, port: {port}, flags: {flags}"
                );

                for txt in txt_records {
                    if let Ok(s) = String::from_utf8(txt.clone()) {
                        log::info!("  TXT: {s}");
                    }
                }
            }
            Some("ItemRemove") => {
                let (interface, protocol, name, stype, domain, flags): (
                    i32,
                    i32,
                    String,
                    String,
                    String,
                    u32,
                ) = msg.body().deserialize()?;

                log::info!(
                    "Removed service: {:?}",
                    (interface, protocol, name, stype, domain, flags)
                );
            }
            _ => {
                log::info!("Some other event");
            }
        }
    }

        Ok(())
    }
}
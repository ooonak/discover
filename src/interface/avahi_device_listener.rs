use crate::domain::DeviceListener;
use futures::stream::StreamExt;
use std::result::Result;
use zbus::Connection;

// Inline the code generated from introspection under its own module
include!("server.rs");
include!("service_browser.rs");
include!("service_resolver.rs");

const AVAHI_PROTO_UNSPEC: i32 = -1;
const AVAHI_PROTO_INET: i32 = 0;
const AVAHI_PROTO_INET6: i32 = 1;
const AVAHI_IF_UNSPEC: i32 = -1;
const AVAHI_IF_LOCAL: i32 = 0;

// An adapter, a specific implementation for a driving port.
pub struct AvahiDeviceListener<'a, T: DeviceListener + 'a> {
    listener: &'a T,
}

impl<'a, T: DeviceListener + 'a> AvahiDeviceListener<'a, T> {
    pub fn new(device_listener: &'a T) -> Self {
        Self {
            listener: device_listener,
        }
    }

    pub async fn listen(&self) -> Result<(), zbus::Error> {
        let connection = Connection::system().await?;

        // Connect to Avahi server
        let server = ServerProxy::builder(&connection)
            .destination("org.freedesktop.Avahi")?
            .path("/")?
            .build()
            .await?;

        // ServiceBrowserNew asks the Avahi daemon to start browsing for services of a given type, interface, and domain.
        // -1 as interface, protocol = 0 = AVAHI_PROTO_UNSPEC
        // Type = "e.g. _http._tcp", domain = "", flags = 0
        let browser_path = server
            .service_browser_new(
                AVAHI_IF_UNSPEC,
                AVAHI_PROTO_INET,
                "_discover._tcp",
                "",
                0,
            )
            .await?;

        // ServiceBrowserNew asks the Avahi daemon to start browsing for services of a given type, interface, and domain.
        let proxy = ServiceBrowserProxy::builder(&connection)
            .destination("org.freedesktop.Avahi")?
            .path(browser_path.clone())?
            .build()
            .await?;

        handle_new_services(&connection, &proxy).await?;
        handle_removed_services(&proxy).await?;

        // Keep running
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }

        proxy.free().await?;
        Ok(())
    }
}

async fn handle_new_services(
    connection: &Connection,
    browser: &ServiceBrowserProxy<'_>,
) -> zbus::Result<()> {
    let mut stream = browser.receive_item_new().await?;

    let conn = connection.clone();
    tokio::spawn(async move {
        while let Some(signal) = stream.next().await {
            if let Ok(args) = signal.args() {
                log::info!(
                    "New service: ({}, {}, {}, {}, {}, {})",
                    args.interface,
                    args.protocol,
                    args.name,
                    args.type_,
                    args.domain,
                    args.name
                );

                // Resolve service
                let server = ServerProxy::new(&conn).await.unwrap();
                if let Ok(resolved) = server
                    .resolve_service(
                        args.interface,
                        args.protocol,
                        &args.name,
                        &args.type_,
                        &args.domain,
                        -1, // aprotocol
                        0,  // flags
                    )
                    .await
                {
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

                    log::info!(
                        "Resolved: if_idx={if_idx}, proto={proto}, name={name}, stype={stype}, \
                         domain={domain}, host_name={host_name}, addr_proto={addr_proto}, \
                         address={address}, port={port}, flags={flags}"
                    );

                    for txt in txt_records {
                        if let Ok(s) = String::from_utf8(txt) {
                            log::info!("  TXT: {s}");
                        }
                    }
                }
            } else {
                log::warn!(
                    "Failed to parse args in signal: member={:?}, interface={:?}, signature={:?}",
                    signal.message().header().member(),
                    signal.message().header().interface(),
                    signal.message().header().signature()
                );
            }
        }
    });

    Ok(())
}

async fn handle_removed_services(browser: &ServiceBrowserProxy<'_>) -> zbus::Result<()> {
    let mut stream = browser.receive_item_remove().await?;

    tokio::spawn(async move {
        while let Some(signal) = stream.next().await {
            if let Ok(args) = signal.args() {
                log::info!(
                    "Removed service: ({}, {}, {}, {}, {}, {})",
                    args.interface,
                    args.protocol,
                    args.name,
                    args.type_,
                    args.domain,
                    args.name
                );
            } else {
                log::warn!(
                    "Failed to parse args in signal: member={:?}, interface={:?}, signature={:?}",
                    signal.message().header().member(),
                    signal.message().header().interface(),
                    signal.message().header().signature()
                );
            }
        }
    });

    Ok(())
}

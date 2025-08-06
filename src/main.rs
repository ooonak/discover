use zbus::{Connection, MessageStream, Proxy, Result};
use zvariant::OwnedObjectPath;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::system().await?;

    // Create a proxy for the Avahi server
    let proxy = Proxy::new(
        &connection,
        "org.freedesktop.Avahi",
        "/",
        "org.freedesktop.Avahi.Server",
    )
    .await?;

    let service_type = "_googlecast._tcp";

    // ServiceBrowserNew asks the Avahi daemon to start browsing for services of a given type, interface, and domain.
    // -1 as interface, protocol = 0 = AVAHI_PROTO_UNSPEC
    // Type = "e.g. _http._tcp", domain = "", flags = 0
    let browser_path: OwnedObjectPath = proxy
        .call(
            "ServiceBrowserNew",
            &( -1i32, -1i32, service_type, "".to_string(), 0u32 ),
        )
        .await?;

    // Create a proxy for that new object path with interface org.freedesktop.Avahi.ServiceBrowser.
    // The ServiceBrowser proxy represents a remote D-Bus object
    // This object emits signals like: ItemNew, ItemRemove
    let browser_proxy = Proxy::new(
        &connection,
        "org.freedesktop.Avahi",
        browser_path.as_str(),
        "org.freedesktop.Avahi.ServiceBrowser",
    )
    .await?;

    // Subscribe to ItemNew signals
    let mut item_new_signals = browser_proxy.receive_signal("ItemNew").await?;

    println!("Listening for Avahi service announcements...");

    while let Some(signal) = item_new_signals.next().await {
        let msg = signal;
        let (interface, protocol, name, stype, domain, flags): (i32, i32, String, String, String, u32) =
            msg.body().deserialize()?;

        println!("New service: {:?}", (interface, protocol, name, stype, domain, flags));
    }

    Ok(())
}

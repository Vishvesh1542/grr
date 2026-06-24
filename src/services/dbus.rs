use async_channel::Sender;
use zbus::connection::Builder;
use zbus::interface;

use crate::BarEvent;

struct DBusDaemon {
    sender: Sender<BarEvent>,
}
#[interface(name = "com.vishvesh.grr")]
impl DBusDaemon {
    async fn toggle(&self) {
        println!("Recieved DBus request: Toggle");
        let _ = self.sender.send(BarEvent::ToggleLauncher()).await;
    }
}

pub async fn start_listening(sender: Sender<BarEvent>) -> zbus::Result<()> {
    let daemon = DBusDaemon { sender };

    let _connection = Builder::session()?
        .name("com.vishvesh.grr")?
        .serve_at("/com/vishvesh/grr", daemon)?
        .build()
        .await?;

    // Keep the connection alive
    std::future::pending::<()>().await;

    Ok(())
}

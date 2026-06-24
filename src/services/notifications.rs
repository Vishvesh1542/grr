use async_channel::Sender;
use zbus::connection::Builder;
use zbus::{interface, zvariant};

use crate::BarEvent;
use crate::widgets::notification::NotificationInfo;

struct NotificationDaemon {
    sender: Sender<BarEvent>,
    counter: u32,
}

#[interface(name = "org.freedesktop.Notifications")]
impl NotificationDaemon {
    async fn get_capabilities(&self) -> Vec<String> {
        vec!["body".to_string()]
    }

    async fn get_server_information(&self) -> (String, String, String, String) {
        (
            "grr-notification-daemon".to_string(),
            "grr".to_string(),
            "0.1".to_string(),
            "1.2".to_string(),
        )
    }

    async fn notify(
        &mut self,
        app_name: String,
        replaces_id: u32, // Don't know enough
        app_icon: String,
        summary: String, // Can contain HTML
        body: String,
        _actions: Vec<String>, // Don't know enough
        _hints: std::collections::HashMap<String, zvariant::OwnedValue>, // Don't know enough
        expire_timeout: i32,
    ) -> u32 {
        let _ = self
            .sender
            .send(BarEvent::Notification(NotificationInfo {
                app_name,
                app_icon,
                summary,
                body,
                timeout: expire_timeout,
            }))
            .await;

        let id = if replaces_id != 0 {
            replaces_id
        } else {
            self.counter += 1;
            self.counter
        };

        id
    }

    async fn close_notification(&self, _id: u32) {
        // no-op
    }
}
pub async fn start_listening(sender: Sender<BarEvent>) -> zbus::Result<()> {
    let daemon = NotificationDaemon { sender, counter: 0 };

    let _connection = Builder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", daemon)?
        .build()
        .await?;

    // Keep the connection alive
    std::future::pending::<()>().await;

    Ok(())
}

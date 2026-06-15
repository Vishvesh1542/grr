use crate::BarEvent;
use crate::widgets::notification::NotificationInfo;
use async_channel::Sender;
use dbus::blocking::Connection;
use dbus::channel::MatchingReceiver;
use dbus_crossroads::{Crossroads, IfaceBuilder};
use std::thread;

pub fn start_listening(sender: Sender<BarEvent>) {
    thread::spawn(|| {
        let connection = Connection::new_session().expect("Error connecting to DBus");
        connection
            .request_name("org.freedesktop.Notifications", false, true, false)
            .expect("Error registering Notifications");

        let mut crossroads = Crossroads::new();

        let iface_token = crossroads.register(
            "org.freedesktop.Notifications",
            |b: &mut IfaceBuilder<()>| {
                // Required by spec: advertise supported capabilities
                b.method("GetCapabilities", (), ("capabilities",), |_, _, _: ()| {
                    Ok((vec!["body".to_string()],))
                });

                // Required by spec: server info
                b.method(
                    "GetServerInformation",
                    (),
                    ("name", "vendor", "version", "spec_version"),
                    |_, _, _: ()| {
                        Ok((
                            "grr-notification-daemon".to_string(),
                            "grr".to_string(),
                            "0.1".to_string(),
                            "1.2".to_string(),
                        ))
                    },
                );

                // We need to keep track of the notif ids
                let mut counter = 1;
                b.method(
                    "Notify",
                    (
                        "app_name",
                        "replaces_id",
                        "app_icon",
                        "summary",
                        "body",
                        "actions",
                        "hints",
                        "expire_timeout",
                    ),
                    ("id",),
                    move |_,
                          _,
                          (
                        app_name,
                        replaces_id, // Don't know enough
                        app_icon,    // Don't know enough
                        summary,     // Can contain common HTML tags
                        body,
                        _actions,       // Don't know enough
                        _hints,         // Don't know
                        expire_timeout, // -1 default, 0 never, millis otherwise,
                    ): (
                        String,
                        u32,
                        String,
                        String,
                        String,
                        Vec<String>,
                        dbus::arg::PropMap,
                        i32,
                    )| {
                        let _ = sender.send_blocking(BarEvent::Notification(NotificationInfo {
                            app_name: app_name,
                            app_icon: app_icon,
                            summary: summary,
                            body: body,
                            timeout: expire_timeout,
                        }));
                        let id = if replaces_id > 0 {
                            replaces_id
                        } else {
                            counter + 1
                        };

                        if replaces_id > 0 {
                            counter += 1;
                        };

                        Ok((id,))
                    },
                );

                // CloseNotification: spec requires this; Also think it's needed for actions to work
                b.method("CloseNotification", ("id",), (), |_, _, (_id,): (u32,)| {
                    Ok(())
                });
            },
        );

        crossroads.insert("/org/freedesktop/Notifications", &[iface_token], ());

        connection.start_receive(
            dbus::message::MatchRule::new_method_call(),
            Box::new(move |msg, conn| {
                crossroads.handle_message(msg, conn).unwrap();
                true
            }),
        );

        loop {
            connection
                .process(std::time::Duration::from_millis(200))
                .expect("Error: Notifications");
        }
    });
}

use crate::BarEvent;
use async_channel::Sender;
use dbus::blocking::Connection;
use dbus::channel::MatchingReceiver;
use dbus_crossroads::{Crossroads, IfaceBuilder};
use std::thread;

pub fn start_listening(sender: Sender<BarEvent>) {
    thread::spawn(|| {
        let connection = Connection::new_session().expect("Error connecting to DBus");
        connection
            .request_name("com.grr.grr", false, true, false)
            .expect("Error registering DBus");

        let mut crossroads = Crossroads::new();

        let iface_token = crossroads.register("com.grr.grr", |b: &mut IfaceBuilder<()>| {
            b.method("toggle", (), (), move |_, _, (): ()| {
                println!("Recieved DBus event!!");
                let _ = sender.send_blocking(BarEvent::ToggleLauncher());
                Ok(())
            });
        });

        crossroads.insert("/com/grr/grr", &[iface_token], ());

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
                .expect("Error: DBus");
        }
    });
}

use adw;
use adw::prelude::*;
use async_channel::{Receiver, Sender};
use gtk4::gdk::{Display, Monitor};
use gtk4::glib;
use std::cell::RefCell;
use std::rc::Rc;

mod bar;
mod services;
mod widgets;
use crate::services::dbus;
use crate::services::notifications;
use crate::widgets::launcher;
use crate::widgets::notification::{NotificationInfo, NotificationServer};
use crate::{
    bar::Bar,
    services::{config, niri},
};

pub enum BarEvent {
    WorkspaceChanged(i32, i32, String), // (active, total, monitor)
    OverviewToggled(bool),              // Current overview state
    Notification(NotificationInfo),
    ToggleLauncher(),
}

fn update_monitors(app: &adw::Application, bars: &mut Vec<Bar>) {
    if let Some(display) = Display::default() {
        let monitors = display.monitors();
        let mut active_monitors = Vec::new();
        // Check if all monitors have a bar. ( Monitor was added )
        for i in 0..monitors.n_items() {
            if let Some(monitor) = monitors.item(i).and_downcast::<Monitor>() {
                active_monitors.push(monitor.clone());
                if !bars.iter().any(|bar| bar.monitor == monitor) {
                    let bar = Bar::init(app, &monitor);
                    bars.push(bar);
                }
            }
        }

        // Check if all bars have a valid monitor ( Monitor was removed )
        bars.retain(|bar| {
            let is_alive = active_monitors.contains(&bar.monitor);
            if !is_alive {
                bar.destroy();
            }
            is_alive
        });
    };
}

fn load_css() {
    let provider = gtk4::CssProvider::new();
    provider.load_from_data(include_str!("style/bar.css"));

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn main() {
    unsafe {
        std::env::set_var("GSK_RENDERER", "cairo");
    }

    config::init_config();

    let app = adw::Application::builder()
        .application_id("com.vishvesh.grr")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(|app| {
        let (s, r): (Sender<BarEvent>, Receiver<BarEvent>) = async_channel::unbounded();

        let s2 = s.clone();
        let s3 = s.clone();
        niri::start_listening(s);
        notifications::start_listening(s2);
        dbus::start_listening(s3);

        let n_s = NotificationServer::init();

        let launcher = launcher::Launcher::init();

        let bars: Rc<RefCell<Vec<Bar>>> = Rc::new(RefCell::new(Vec::new()));
        update_monitors(app, &mut bars.borrow_mut());

        // On monitors changed
        if let Some(d) = Display::default() {
            let monitors = d.monitors();

            let app_clone = app.clone();
            let bars_clone = bars.clone();
            monitors.connect_items_changed(move |_, _, _, _| {
                update_monitors(&app_clone, &mut bars_clone.borrow_mut());
            });
        }

        // Recieve all events
        glib::spawn_future_local(async move {
            let bars_clone = bars.clone();
            while let Ok(event) = r.recv().await {
                match event {
                    BarEvent::WorkspaceChanged(active, total, output) => {
                        let mut bars_borrow = bars_clone.borrow_mut();
                        for bar in bars_borrow.iter_mut() {
                            if let Some(bar_connector) = bar.monitor.connector() {
                                if bar_connector == output {
                                    bar.workspace_changed(active, total)
                                }
                            }
                        }
                    }
                    BarEvent::OverviewToggled(is_open) => {
                        let mut bars_borrow = bars_clone.borrow_mut();
                        for bar in bars_borrow.iter_mut() {
                            bar.set_overview_state(is_open);
                        }
                    }
                    BarEvent::Notification(notification) => {
                        n_s.new_notif(notification);
                    }
                    BarEvent::ToggleLauncher() => {
                        launcher.toggle();
                    }
                }
            }
        });
    });

    app.run();
}

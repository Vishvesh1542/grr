use adw::prelude::AdwWindowExt;
use adw::{Window, glib};
use gtk4::prelude::{BoxExt, GtkWindowExt};
use gtk4::{Box, Label};
use gtk4_layer_shell::{self, Edge, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::services::config;

#[derive(Clone)]
pub struct NotificationInfo {
    pub app_name: String,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub timeout: i32,
}

struct NotificationWidget {
    notification_info: NotificationInfo,
    window: Window,
    create_time: Instant,
    timeout_duration: i32,
}

pub struct NotificationServer {
    notification_history: Rc<RefCell<Vec<NotificationInfo>>>,
    current_notification: Rc<RefCell<Option<NotificationWidget>>>,
}
impl NotificationServer {
    pub fn init() -> Self {
        Self {
            notification_history: Rc::new(RefCell::new(Vec::new())),
            current_notification: Rc::new(RefCell::new(None)),
        }
    }

    pub fn new_notif(&self, notif: NotificationInfo) {
        if let Some(prev_notif) = self.current_notification.borrow().as_ref() {
            prev_notif.window.destroy();
        }

        let window = Window::builder().build();
        window.init_layer_shell();
        window.set_anchor(Edge::Top, true);

        let layout = Box::new(gtk4::Orientation::Vertical, config::get_config().spacing);

        let title = Label::new(Some(&notif.app_name));
        layout.append(&title);

        let description = Label::new(Some(&notif.body));
        layout.append(&description);

        window.set_content(Some(&layout));
        window.present();

        let mut timeout_duration = 2000; // Milliseconds

        if notif.timeout == 0 {
            timeout_duration = -1 // Never
        } else if notif.timeout > 0 {
            timeout_duration = notif.timeout
        };

        let w = window.clone();

        *self.current_notification.borrow_mut() = Some(NotificationWidget {
            notification_info: notif.clone(),
            window: window,
            create_time: Instant::now(),
            timeout_duration,
        });

        if timeout_duration != -1 {
            let current_notification = self.current_notification.clone();
            glib::timeout_add_local_once(
                Duration::from_millis(timeout_duration as u64),
                move || {
                    w.destroy();
                    *current_notification.borrow_mut() = None
                },
            );
        }
        self.notification_history.borrow_mut().push(notif);
    }
}

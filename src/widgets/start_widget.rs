use gtk4::{Box, Button, Orientation, prelude::*};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use crate::services::{config, niri};

pub struct StartWidget {
    pub widget: Box,
    buttons_container: Box,
    buttons: Rc<RefCell<Vec<Button>>>,
}
impl StartWidget {
    pub fn init() -> Self {
        let widget = Box::new(Orientation::Horizontal, 0);
        widget.add_css_class("start-widget");

        let buttons_container = Box::builder()
            .orientation(Orientation::Horizontal)
            .margin_start(config::get_config().margin)
            .valign(gtk4::Align::Center)
            .css_classes(["start-widget-container"])
            .build();
        let buttons: Rc<RefCell<Vec<Button>>> = Rc::new(RefCell::new(Vec::new()));

        let motion = gtk4::EventControllerMotion::new();

        let buttons_container_clone = buttons_container.clone();
        motion.connect_enter(move |_, _, _| {
            buttons_container_clone.add_css_class("start-widget-container-hovered");
        });
        let buttons_container_clone = buttons_container.clone();
        motion.connect_leave(move |_| {
            buttons_container_clone.remove_css_class("start-widget-container-hovered");
        });

        widget.add_controller(motion);

        widget.append(&buttons_container);

        Self {
            widget,
            buttons_container,
            buttons,
        }
    }

    pub fn workspace_changed(&self, active_workspace: i32, total_workspaces: i32) {
        let dif = (total_workspaces - self.buttons.borrow().len() as i32).abs();

        if self.buttons.borrow().len() as i32 > total_workspaces {
            for _ in 0..dif {
                let b = self.buttons.borrow().last().cloned();
                if let Some(b) = b {
                    b.remove_css_class("start-widget-button-visible");
                    let b_c = self.buttons_container.clone();
                    let b = b.clone();
                    gtk4::glib::timeout_add_local_once(Duration::from_millis(250), move || {
                        b_c.remove(&b);
                    });
                    self.buttons.borrow_mut().pop();
                }
            }
        };
        if total_workspaces > self.buttons.borrow().len() as i32 {
            for _ in 0..dif {
                let b = Button::builder()
                    .css_classes(["start-widget-button"])
                    .vexpand(false)
                    .hexpand(false)
                    .valign(gtk4::Align::Center)
                    .halign(gtk4::Align::Center)
                    .margin_end(2)
                    .build();

                let pos = (self.buttons.borrow().len() + 1) as i32;
                b.connect_clicked(move |_| niri::switch_workspace(pos));

                self.buttons_container.append(&b);

                let b2 = b.clone();
                self.buttons.borrow_mut().push(b);
                gtk4::glib::timeout_add_local_once(Duration::from_millis(100), move || {
                    b2.add_css_class("start-widget-button-visible");
                });
            }
        };

        // Active button
        let buttons = self.buttons.borrow();
        for (i, button) in buttons.iter().enumerate() {
            if i == (active_workspace - 1) as usize {
                button.add_css_class("start-widget-button-active");
            } else {
                button.remove_css_class("start-widget-button-active");
            }
        }
    }

    pub fn set_overview_state(&self, is_open: bool) {
        if is_open {
            for button in self.buttons.borrow_mut().iter() {
                button.add_css_class("start-widget-button-overview");
            }
        } else {
            for button in self.buttons.borrow_mut().iter() {
                button.remove_css_class("start-widget-button-overview");
            }
        }
    }
}

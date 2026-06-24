use adw::Window;
use adw::prelude::*;
use gtk4::Orientation;
use gtk4::{Box, Label};
use gtk4_layer_shell;
use gtk4_layer_shell::LayerShell;

use crate::services::search_provider;

pub struct Launcher {
    window: Window,
    layout: Box,
}

impl Launcher {
    pub fn init() -> Self {
        let window = Window::new();
        window.init_layer_shell();
        window.set_layer(gtk4_layer_shell::Layer::Overlay);
        let layout = Box::builder().orientation(Orientation::Vertical).build();

        window.set_content(Some(&layout));
        Self { window, layout }
    }

    pub fn toggle(&self) {
        if self.window.is_visible() {
            self.window.set_visible(false);
        } else {
            self.window.set_visible(true);
            while let Some(w) = self.layout.last_child() {
                self.layout.remove(&w);
            }
            let apps = search_provider::get_app_list("zen");
            println!("{:?}", apps);
            for app in apps {
                self.layout
                    .append(&Label::new(Some(&app.name().to_string())));
            }
        }
    }
}

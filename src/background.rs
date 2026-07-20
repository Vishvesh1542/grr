use adw::{Application, ApplicationWindow, prelude::*};
use gtk4::{
    ContentFit, Picture,
    gdk::{self, Monitor},
    gio::File,
};
use gtk4_layer_shell::{self, Edge, Layer, LayerShell};

use crate::services::config;

pub struct Background {
    pub window: ApplicationWindow,
    pub monitor: Monitor,
    pub picture: Picture,
}
impl Background {
    pub fn init(app: &Application, monitor: &gdk::Monitor) -> Self {
        let window = ApplicationWindow::builder().application(app).build();
        window.init_layer_shell();
        window.set_exclusive_zone(-1);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Bottom, true);
        window.set_layer(Layer::Background);
        window.set_monitor(Some(monitor));

        let picture = Picture::for_filename(config::get_config().background.path.as_str());
        picture.set_content_fit(ContentFit::Fill);

        window.set_content(Some(&picture));

        window.present();

        Self {
            window,
            monitor: monitor.clone(),
            picture,
        }
    }

    pub fn destroy(&self) {
        self.window.destroy();
    }

    pub fn switch_background(&self, new_bg_file: &File) {
        self.picture.set_file(Some(new_bg_file));

        if let Some(path) = new_bg_file.path() {
            let string = path.to_str().unwrap_or("null").to_string();
            config::get_config_mut().background.path = string;
            config::save();
        };
    }
}

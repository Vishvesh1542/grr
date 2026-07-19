use adw::ApplicationWindow;
use adw::gdk::Monitor;
use adw::prelude::*;
use adw::{Application, prelude::AdwApplicationWindowExt};
use gtk4::gio::File;
use gtk4::{ContentFit, Picture, gdk};
use gtk4_layer_shell::{self, Edge, Layer, LayerShell};

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

        let picture = Picture::for_filename("/home/vishvesh/Pictures/Wallpapers/wallpaper-2.png");
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
    }
}

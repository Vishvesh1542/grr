use crate::services::search_provider;
use adw::{gio::AppInfo, prelude::*};
use gtk4::{Box, Button, Label, Orientation, ScrolledWindow, SearchEntry, Window, gdk::Key, glib};
use gtk4_layer_shell::{self, Edge, LayerShell};
use std::time::Duration;

fn launch(app: &AppInfo) {
    let context = gtk4::gdk::Display::default().map(|d| d.app_launch_context());
    if let Err(e) = app.launch(&[], context.as_ref()) {
        println!("Failed to launch {}: {}", app.display_name(), e);
    }
}

#[derive(Clone)]
pub struct Launcher {
    window: Window,
    layout: Box,
    search_box: SearchEntry,
    scroll_layout: Box,
    footer_results: Label,
    footer_time: Label,
}

impl Launcher {
    pub fn init() -> Self {
        let window = Window::new();
        window.add_css_class("launcher-window");
        window.init_layer_shell();
        window.set_layer(gtk4_layer_shell::Layer::Overlay);
        window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);

        window.set_anchor(Edge::Bottom, true);
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);

        // Fills entire screen
        let outer = Box::builder()
            .orientation(Orientation::Vertical)
            .halign(gtk4::Align::Center)
            .valign(gtk4::Align::Center)
            .css_classes(["launcher-outer-box"])
            .build();

        let layout = Box::builder()
            .orientation(Orientation::Vertical)
            .css_classes(["launcher-box"])
            .build();
        layout.set_size_request(500, 600);

        let search_box = SearchEntry::builder()
            .placeholder_text("Search for apps")
            .css_classes(["launcher-search-box", "title-2"])
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(7)
            .margin_end(7)
            .height_request(50)
            .search_delay(10)
            .build();
        let s_b = search_box.clone();

        let scrollable = ScrolledWindow::new();
        scrollable.add_css_class("launcher-scrolled-window");
        scrollable.set_vexpand(true);

        layout.append(&search_box);
        layout.append(&scrollable);
        outer.append(&layout);

        let scroll_layout = Box::new(Orientation::Vertical, 0);
        scrollable.set_child(Some(&scroll_layout));
        window.set_child(Some(&outer));

        let footer = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(10)
            .css_classes(["launcher-footer"])
            .build();
        let footer_results = Label::builder()
            .halign(gtk4::Align::Start)
            .margin_start(10)
            .margin_top(10)
            .margin_bottom(10)
            .build();
        let footer_time = Label::builder()
            .halign(gtk4::Align::End)
            .hexpand(true)
            .margin_top(10)
            .margin_end(10)
            .margin_bottom(10)
            .build();

        footer.append(&footer_results);
        footer.append(&footer_time);
        layout.append(&footer);

        let s = Self {
            window,
            layout,
            search_box,
            scroll_layout,
            footer_results,
            footer_time,
        };

        let s2 = s.clone();
        let s3 = s.clone();
        let s4 = s.clone();
        let s5 = s.clone();

        s_b.connect_search_changed(move |entry| {
            s2.update_search(&entry.text());
        });

        s_b.connect_activate(move |entry| {
            let apps = search_provider::get_app_list(&entry.text());
            if let Some(app) = apps.first() {
                launch(app);
                s3.toggle();
            };
        });

        let key_controller = gtk4::EventControllerKey::new();

        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == Key::Escape {
                s4.toggle();
                return glib::Propagation::Stop;
            }

            // Focus search box on any alphanumeric or punctuation keypress
            if let Some(c) = key.to_unicode().filter(|c| !c.is_control()) {
                s4.search_box.grab_focus();
                let mut text = s4.search_box.text().to_string();
                text.push(c);
                s4.search_box.set_text(&text);
                // Move cursor to end
                s4.search_box.set_position(-1);
            }

            glib::Propagation::Proceed
        });

        let escape_controller = gtk4::EventControllerKey::new();
        escape_controller.connect_key_pressed(move |_, key, _, _| {
            if key == Key::Escape {
                s5.toggle();
                return glib::Propagation::Stop;
            }

            glib::Propagation::Proceed
        });

        s.search_box.add_controller(escape_controller);
        s.window.add_controller(key_controller);

        s
    }

    fn reset_layout(&self) {
        while let Some(w) = self.scroll_layout.last_child() {
            self.scroll_layout.remove(&w);
        }
    }

    fn update_search(&self, search_query: &str) {
        let s_time = std::time::Instant::now();
        self.reset_layout();

        let apps = search_provider::get_app_list(search_query);
        let num_of_apps = apps.len();

        for app in apps {
            let button = Button::builder().css_classes(["flat"]).build();
            let layout = Box::new(Orientation::Horizontal, 0);
            button.set_child(Some(&layout));

            let a = app.clone();
            let s = self.clone();
            button.connect_clicked(move |_| {
                launch(&a);
                s.toggle();
            });

            if let Some(icon) = app.icon() {
                let image = gtk4::Image::from_gicon(&icon);
                image.set_icon_size(gtk4::IconSize::Large);
                layout.append(&image);
            } else {
                let image = gtk4::Image::from_icon_name("image-missing-symbolic");
                image.set_icon_size(gtk4::IconSize::Large);
                layout.append(&image);
            }

            let name = Label::builder()
                .margin_start(10)
                .css_classes(["launcher-item-label"])
                .build();
            name.set_text(&app.display_name());
            layout.append(&name);
            self.scroll_layout.append(&button);
        }

        let elapsed = s_time.elapsed();
        self.footer_results
            .set_label(&format!("{} results", num_of_apps));
        self.footer_time
            .set_label(&format!("{:.1}ms", elapsed.as_secs_f64() * 1000.0));
    }

    pub fn toggle(&self) {
        if self.window.is_visible() {
            self.layout.remove_css_class("launcher-box-visible");
            let w = self.window.clone();
            glib::timeout_add_local_once(Duration::from_millis(150), move || {
                w.hide();
            });
        } else {
            self.window.show();
            self.search_box.set_text("");
            self.layout.add_css_class("launcher-box-visible");
        }
    }
}

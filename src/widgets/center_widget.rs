use gtk4::{Box, Label, glib, prelude::*};
use time_format;

use crate::services::config;

pub struct CenterWidget {
    pub widget: Box,
}
impl CenterWidget {
    pub fn init() -> Self {
        let widget = Box::builder().build();

        fn get_current_time_label() -> String {
            let ts = time_format::now().unwrap();
            return time_format::strftime_local(config::get_config().time_format.as_str(), ts)
                .unwrap();
        }

        let time_label = Label::builder()
            .css_classes(["heading"])
            .label(get_current_time_label())
            .build();

        widget.append(&time_label);

        // We don't need 1 second accuracy.
        glib::timeout_add_seconds_local(3, move || {
            time_label.set_label(&get_current_time_label());
            glib::ControlFlow::Continue
        });

        Self { widget }
    }
}

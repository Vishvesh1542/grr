use adw::{Application, ApplicationWindow, prelude::*};
use gtk4::{Box, CenterBox, Orientation, gdk};
use gtk4_layer_shell::{self, Edge, LayerShell};

use crate::services::config::{self};
use crate::widgets::center_widget::CenterWidget;
use crate::widgets::start_widget::StartWidget;

pub struct Bar {
    pub window: ApplicationWindow,
    pub monitor: gdk::Monitor,

    start_widget: StartWidget,
}
impl Bar {
    pub fn init(app: &Application, monitor: &gdk::Monitor) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .height_request(config::get_config().bar.height)
            .build();
        window.init_layer_shell();
        window.auto_exclusive_zone_enable();
        window.set_anchor(Edge::Left, true);
        window.set_anchor(Edge::Right, true);
        window.set_anchor(Edge::Top, true);
        window.set_monitor(Some(monitor));

        let start_widget = StartWidget::init();
        let center_widget = CenterWidget::init();
        let end_widget = Box::new(Orientation::Horizontal, 0);

        let layout = CenterBox::builder().build();
        layout.set_start_widget(Some(&start_widget.widget));
        layout.set_center_widget(Some(&center_widget.widget));
        layout.set_end_widget(Some(&end_widget));

        window.set_content(Some(&layout));
        window.present();

        let monitor = monitor.clone();
        Self {
            window,
            monitor,
            start_widget,
        }
    }

    pub fn destroy(&self) {
        self.window.destroy();
    }

    pub fn workspace_changed(&self, active_workspace: i32, total_workspaces: i32) {
        self.start_widget
            .workspace_changed(active_workspace, total_workspaces)
    }

    pub fn set_overview_state(&self, is_open: bool) {
        self.start_widget.set_overview_state(is_open)
    }
}

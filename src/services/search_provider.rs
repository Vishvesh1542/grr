use gtk4::gio;

pub fn get_app_list(search_query: &str) -> Vec<gio::AppInfo> {
    gio::AppInfo::all()
}

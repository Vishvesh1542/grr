use adw::prelude::*;
use gtk4::gio;

pub fn get_app_list(search_query: &str) -> Vec<gio::AppInfo> {
    let all_apps = gio::AppInfo::all();
    let mut matching_files = Vec::new();
    let search_query = search_query.trim().to_lowercase();

    for app in all_apps {
        if !app.should_show() {
            continue;
        }

        let display_name_matches = app.display_name().to_lowercase().contains(&search_query);

        let desc_matches = app
            .description()
            .map_or(false, |desc| desc.to_lowercase().contains(&search_query));

        let name_matches = app.name().to_lowercase().contains(&search_query);

        let mut executable_matches = false;
        if let Some(executable) = app.commandline() {
            executable_matches = executable
                .to_str()
                .map_or(false, |desc| desc.to_lowercase().contains(&search_query))
        }

        if display_name_matches || desc_matches || name_matches || executable_matches {
            if app.should_show() {
                matching_files.push(app);
            }
        }
    }

    matching_files
}

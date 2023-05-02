#[derive(Debug, Clone)]
pub struct App {
    pub name: String,
    pub short: String,
    pub code: String,
}

pub fn find_app<'a>(apps: &'a [App], name: &str) -> Option<&'a App> {
    apps.iter()
        .find(|app| app.name == name || app.short == name)
}

use std::{collections::HashMap, io::Error, path::PathBuf, str::FromStr};

use super::ini_file::parse_ini_file;

#[derive(Default, Debug, Clone)]
pub enum ApplicationType {
    #[default]
    Application,
    Link,
    Directory,
}

impl FromStr for ApplicationType {
    type Err = ();

    fn from_str(input: &str) -> Result<ApplicationType, Self::Err> {
        match input {
            "Application" => Ok(ApplicationType::Application),
            "Link" => Ok(ApplicationType::Link),
            "Directory" => Ok(ApplicationType::Directory),
            _ => Err(()),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct ApplicationFile {
    pub app_name: String,
    pub app_type: ApplicationType,
    pub app_generic_name: Option<String>,
    pub app_comment: Option<String>,
    pub app_icon: Option<String>,
    pub app_hidden: Option<bool>,
    pub app_only_show_in: Option<Vec<String>>,
    pub app_not_show_in: Option<Vec<String>>,
    pub app_dbus_activatable: Option<bool>,
    pub app_try_exec: Option<PathBuf>,
    pub app_exec: Option<String>,
    pub app_cwd: Option<PathBuf>,
    pub app_terminal: Option<bool>,
    pub app_actions: Option<Vec<ApplicationFileAction>>,
    pub app_mime_types: Option<Vec<String>>,
    pub app_categories: Option<Vec<String>>,
    pub app_implements: Option<Vec<String>>,
    pub app_keywords: Option<Vec<String>>,
    pub app_startup_wm_class: Option<String>,
    pub app_url: Option<String>,
    pub app_prefers_non_default_gpu: Option<bool>,
    pub app_single_main_window: Option<bool>,
    pub file_address: PathBuf
}

impl PartialEq for ApplicationFile {
    fn eq(&self, other: &Self) -> bool {
        self.file_address == other.file_address
    }
}

#[derive(Debug, Clone)]
pub struct ApplicationFileAction {
    name: String,
    icon: PathBuf,
    exec: String,
}

pub fn parse_application_file(filename: PathBuf) -> Result<ApplicationFile, Error> {
    //let file_content = fs::read_to_string(filename).unwrap_or("".to_string());

    let mut file = parse_ini_file(&filename)?;
    let mut app = ApplicationFile::default();

    app.file_address = filename;

    let default_desktop_entry: &mut HashMap<String, String> = &mut Default::default();
    let desktop_entry = file
        .get_mut("Desktop Entry")
        .unwrap_or(default_desktop_entry);

    app.app_type = desktop_entry
        .remove("Type")
        .unwrap_or_default()
        .parse()
        .unwrap_or_default();
    app.app_name = desktop_entry.remove("Name").unwrap_or_default();

    app.app_generic_name = desktop_entry.remove("GenericName");
    app.app_comment = desktop_entry.remove("Comment");
    app.app_exec = desktop_entry.remove("Exec");
    app.app_startup_wm_class = desktop_entry.remove("StartupWMClass");
    app.app_url = desktop_entry.remove("URL");
    app.app_icon = desktop_entry.remove("Icon");

    app.app_hidden = desktop_entry.remove("Hidden").map(|s| s == "true");
    app.app_prefers_non_default_gpu = desktop_entry.remove("PrefersNonDefaultGPU").map(|s| s == "true");
    app.app_single_main_window = desktop_entry.remove("SingleMainWindow").map(|s| s == "true");
    app.app_terminal = desktop_entry.remove("Terminal").map(|s| s == "true");
    app.app_dbus_activatable = desktop_entry.remove("DBusActivatable").map(|s| s == "true");

    app.app_only_show_in = desktop_entry
        .remove("OnlyShowIn")
        .map(|s| s.split(":").map(|c| c.into()).collect());
    app.app_not_show_in = desktop_entry
        .remove("NotShowIn")
        .map(|s| s.split(":").map(|c| c.into()).collect());
    app.app_mime_types = desktop_entry
        .remove("MimeType")
        .map(|s| s.split(":").map(|c| c.into()).collect());
    app.app_categories = desktop_entry
        .remove("Categories")
        .map(|s| s.split(":").map(|c| c.into()).collect());
    app.app_implements = desktop_entry
        .remove("Implements")
        .map(|s| s.split(":").map(|c| c.into()).collect());
    app.app_keywords = desktop_entry
        .remove("Keywords")
        .map(|s| s.split(":").map(|c| c.into()).collect());


    return Ok(app);
}

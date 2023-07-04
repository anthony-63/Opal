use std::{fs::File, path::Path, io::Read};

use serde_derive::Deserialize;

use crate::game::globals::SETTINGS_PATH;

#[derive(Deserialize)]
pub struct OpalSkinSettings {
    selected_skin: String,
}

#[derive(Deserialize)]
pub struct OpalNoteSettings {
    scroll_speed: u32,
}

#[derive(Deserialize)]
pub struct OpalDebugSettings {
    default_map_selected: String,
}

#[derive(Deserialize)]
pub struct OpalSettings {
    skin: OpalSkinSettings,
    note: OpalNoteSettings,
    debug: OpalDebugSettings,
}

pub struct OpalSettingsLoader;

impl OpalSettingsLoader {
    pub fn load_settings() -> OpalSettings {        
        if !Path::new("data/settings.toml").exists() {
            println!("Settings file does not exist. Using default settings.");
        }


        let mut settings_file = File::open(SETTINGS_PATH).expect("Failed to open settings file");
        let mut settings_toml_string = String::new();
        let _ = settings_file.read_to_string(&mut settings_toml_string);

        return toml::from_str(&settings_toml_string).expect("Failed to parse settings file. Missing settings?")
    }

    pub fn default_settings() -> OpalSettings {
        OpalSettings { 
            skin: OpalSkinSettings { 
                selected_skin: "default".to_string(),
            },
            note: OpalNoteSettings { 
                scroll_speed: 18,
            },
            debug: OpalDebugSettings {
                default_map_selected: "dj-taka_quaver".to_string(),
            }
        }
    }
}
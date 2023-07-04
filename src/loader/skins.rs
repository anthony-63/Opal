use std::path::Path;

use kira::{manager::{AudioManager, backend::DefaultBackend, AudioManagerSettings}, sound::static_sound::{StaticSoundData, StaticSoundSettings}};
use nannou::prelude::*;
use serde_derive::Deserialize;

use crate::game::{globals::SKINS_FOLDER_PATH, game::Opal};

#[derive(Deserialize, Clone)]
pub struct OpalSkin {
    pub open_sound: String,
    pub welcome_background_color: u32,
    pub welcome_logo: String,
    pub welcome_wait: u64,
}

#[derive(Clone)]
pub struct OpalSkinData {
    pub skin: OpalSkin,
    path: String,
}

pub struct OpalSkinLoader;

impl OpalSkinLoader {
    pub fn load_skins() -> Vec<OpalSkinData> {
        let mut skin_data: Vec<OpalSkinData> = vec![];

        for skin_root in Path::new(SKINS_FOLDER_PATH).read_dir().unwrap() {
            let skin_root_path = skin_root.unwrap().path();
            
            println!("Skin {:?}:", skin_root_path);

            for skin_child in Path::new(&skin_root_path).read_dir().unwrap() {
                let skin_child_path = skin_child.unwrap().path();
                if skin_child_path.ends_with("skin.toml") {
                    skin_data.push(OpalSkinData{
                        skin: toml::from_str(&std::fs::read_to_string(skin_child_path).unwrap()).unwrap(),
                        path: skin_root_path.to_str().unwrap().to_string(),   
                    })
                }
            }
        }

        return skin_data;
    }

    pub fn empty_skin() -> OpalSkinData {
        return OpalSkinData { 
            skin: OpalSkin { 
                open_sound: String::new(), 
                welcome_background_color: 0, 
                welcome_logo: String::new(),
                welcome_wait: 0,
            }, 
            path: String::new() 
        }
    }

    pub fn play_welcome_sound(effect_audio_manager: &mut AudioManager, skin: OpalSkinData) {
        let audio_data = StaticSoundData::from_file(Path::new(&skin.path).join(skin.skin.open_sound), StaticSoundSettings::default()).expect("Failed to load welcome sound audio");
        effect_audio_manager.play(audio_data.clone()).expect("Failed to play welcome audio sound");
    }


    pub fn get_welcome_background_rgb(skin: OpalSkinData) -> Rgb {
        let c = skin.skin.welcome_background_color;
        let r = (c & 0xff0000) >> 16;
        let g = (c & 0x00ff00) >> 8;
        let b = c & 0x0000ff;
        println!("RGB: {} {} {}", r, g, b);
        return rgb(r as f32, g as f32, b as f32);
    }
}
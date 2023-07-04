use core::time;
use std::thread;

use kira::manager::{AudioManager, AudioManagerSettings, backend::DefaultBackend};

use crate::loader::{settings::{OpalSettings, OpalSettingsLoader}, skins::{OpalSkin, OpalSkinLoader, OpalSkinData}};
use nannou::prelude::*;

pub enum OpalState {
    WELCOME,
    IN_MENU_1,
    IN_MENU_2,
    IN_MAP_LIST,
    IN_SETTINGS,
}

pub struct Opal {
    settings: OpalSettings,
    skins: Vec<OpalSkinData>,
    selected_skin: OpalSkinData,
    effect_sound_manager: AudioManager,
    pub state: OpalState,
}

impl Opal {
    pub fn new() -> Self {
        let skins = OpalSkinLoader::load_skins();
        Self {
            settings: OpalSettingsLoader::load_settings(),
            selected_skin: skins[0].clone(),
            skins,
            effect_sound_manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).expect("Failed to create welcome sound manager"),
            state: OpalState::WELCOME,
        }
    }

    pub fn welcome(&mut self) {
        OpalSkinLoader::play_welcome_sound(&mut self.effect_sound_manager, self.selected_skin.clone());
    }

    pub fn draw_welcome(&self, draw: Draw, frame: Frame) {
        draw.background().color(OpalSkinLoader::get_welcome_background_rgb(self.selected_skin.clone()));
        thread::sleep(time::Duration::from_secs(self.selected_skin.skin.welcome_wait));
    }
    
    pub fn draw_menu_1(&self, draw: Draw, frame: Frame) {
        draw.background().color(RED);
    }
}
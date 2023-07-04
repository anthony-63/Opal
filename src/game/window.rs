use nannou::prelude::*;

use super::game::Opal;
use super::game::OpalState;

pub struct OpalWindow {
    _win: window::Id,
    opal: Opal,
}

impl OpalWindow {
    pub fn run() {
        nannou::app(Self::get_model).update(Self::update).run();
    }

    pub fn get_model(app: &App) -> Self {
        let _win = app.new_window()
            .view(Self::draw)
            .resizable(false).size(1280, 720)
            .title("Opal")
            .build()
            .unwrap();
        
        let mut opal = Opal::new();
        let draw = app.draw();

        opal.welcome();
        
        opal.state = OpalState::IN_MENU_1;

        Self {
            _win,
            opal,
        }
    }

    pub fn draw(app: &App, model: &Self, frame: Frame) {
        let draw = app.draw();

        match model.opal.state {
            OpalState::WELCOME => model.opal.draw_welcome(draw, frame),
            OpalState::IN_MENU_1 => model.opal.draw_menu_1(draw, frame),
            OpalState::IN_MENU_2 => (),
            OpalState::IN_MAP_LIST => (),
            OpalState::IN_SETTINGS => (),
        }
    }

    pub fn update(_app: &App, _model: &mut Self, _update: Update)  {

    }
}


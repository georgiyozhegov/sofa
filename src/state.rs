use raylib::RaylibHandle;

use crate::config::Config;
use crate::input::Action;

pub struct State {
    pub window_width: i32,
    pub window_height: i32,
}

impl State {
    pub fn new(config: &Config) -> Self {
        Self {
            window_width: config.window_initial_width,
            window_height: config.window_initial_height,
        }
    }

    pub fn update_size(&mut self, context: &RaylibHandle) {
        self.window_width = context.get_screen_width();
        self.window_height = context.get_screen_height();
    }

    pub fn update(&mut self, action: &Action) {
    }
}

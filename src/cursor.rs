use crate::{
        config::Config,
        content::Content,
        input::{
                Action,
                Mode,
        },
};
use raylib::text::Font;

#[derive(PartialEq)]
pub enum Shape
{
        Block,
        Stick,
}

pub struct Cursor
{
        pub x: i32,
        pub y: i32,
        pub width: i32,
        pub height: i32,
        pub shape: Shape,
}

impl Cursor
{
        pub fn new(config: &Config) -> Self
        {
                Self {
                        x: 0,
                        y: 0,
                        width: (config.font_size * 0.5 + config.font_spacing) as i32,
                        height: config.font_size as i32,
                        shape: Shape::Block,
                }
        }

        pub fn update(&mut self, content: &Content, action: &Action)
        {
                self.x = content.column as i32 * self.width;
                self.y = content.row as i32 * self.height;
                match action {
                        Action::Switch(Mode::Insert) => self.shape = Shape::Stick,
                        Action::Switch(Mode::Base) => self.shape = Shape::Block,
                        _ => {}
                }
        }
}

use crate::config::Config;
use crate::content::Content;
use crate::cursor::{
        Cursor,
        Shape,
};

use raylib::drawing::{
        RaylibDraw,
        RaylibDrawHandle,
};
use raylib::math::Vector2;
use raylib::text::Font;

pub fn draw_content(canvas: &mut RaylibDrawHandle, content: &Content, font: &Font, config: &Config)
{
        for (index, line) in content.content.iter().enumerate() {
                canvas.draw_text_ex(
                        font,
                        line,
                        Vector2::new(0.0, config.font_size * index as f32),
                        config.font_size,
                        config.font_spacing,
                        config.font_color,
                );
        }
}

pub fn draw_cursor(canvas: &mut RaylibDrawHandle, cursor: &Cursor, config: &Config)
{
        canvas.draw_rectangle(
                cursor.x,
                cursor.y,
                if cursor.shape == Shape::Block {
                        cursor.width
                }
                else {
                        1
                },
                cursor.height,
                config.cursor_color,
        );
}

use crate::config::Config;
use crate::content::Content;
use crate::cursor::{
        Cursor,
        Shape,
};
use crate::input::{
        Input,
        Mode,
        Modifier,
};
use raylib::drawing::{
        RaylibDraw,
        RaylibDrawHandle,
};
use raylib::math::Vector2;
use raylib::text::Font;

pub fn background(canvas: &mut RaylibDrawHandle, config: &Config)
{
        canvas.clear_background(config.background_color);
}

pub fn content(canvas: &mut RaylibDrawHandle, content: &Content, font: &Font, config: &Config)
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

pub fn cursor(canvas: &mut RaylibDrawHandle, cursor: &Cursor, config: &Config)
{
        canvas.draw_rectangle(
                cursor.x - config.cursor_stick_width / 2,
                cursor.y,
                if cursor.shape == Shape::Block {
                        cursor.width
                }
                else {
                        config.cursor_stick_width
                },
                cursor.height,
                config.cursor_color,
        );
}

fn status_line_background(canvas: &mut RaylibDrawHandle, config: &Config)
{
        canvas.draw_rectangle(
                0,
                config.window_height - config.status_line_height,
                config.window_width,
                config.status_line_height,
                config.status_line_color,
        );
}

fn status(input: &Input) -> String
{
        let mut mode = match input.mode {
                Mode::Base => "Base",
                Mode::Insert => "Insert",
        }
        .to_owned();
        let modifier = match input.modifier {
                Some(Modifier::Delete) => " (Delete)",
                Some(Modifier::GoTo) => " (Go to)",
                Some(Modifier::Create) => " (Create)",
                None => "",
                _ => todo!(),
        };
        mode.push_str(modifier);
        mode
}

fn status_line_mode(canvas: &mut RaylibDrawHandle, input: &Input, font: &Font, config: &Config)
{
        canvas.draw_text_ex(
                font,
                &status(input),
                Vector2::new(
                        0.0,
                        (config.window_height - config.status_line_height) as f32,
                ),
                config.font_size,
                config.font_spacing,
                config.status_line_font_color,
        );
}

pub fn status_line(canvas: &mut RaylibDrawHandle, input: &Input, font: &Font, config: &Config)
{
        status_line_background(canvas, config);
        status_line_mode(canvas, input, font, config);
}

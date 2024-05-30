mod config;
mod content;
mod cursor;
mod draw;
mod input;

use std::task::Context;

use config::Config;
use content::Content;
use cursor::Cursor;
use draw::{
        draw_content,
        draw_cursor,
};
use input::{
        Action,
        Input,
};

use raylib::color::Color;
use raylib::drawing::RaylibDraw;

fn main()
{
        let config = Config::new(
                600,
                480,
                "Sofa",
                "/usr/share/fonts/TTF/JetBrainsMonoNLNerdFont-Regular.ttf",
                30.0,
                1.0,
                Color::BLACK,
                Color::WHITE.alpha(0.5),
                Color::PINK,
        );

        // init
        let (mut context, thread) = raylib::init()
                .size(config.window_width, config.window_height)
                .resizable()
                .title(config.window_title)
                .build();

        context.set_target_fps(60);
        context.set_exit_key(None);

        let font = context
                .load_font_ex(&thread, config.font_path, config.font_size as i32, None)
                .unwrap();

        println!("{font:?}");

        let mut input = Input::new();
        let mut content = Content::new();
        let mut cursor = Cursor::new(&config);

        context.get_window_state().set_vsync_hint(true);

        while !context.window_should_close() {
                {
                        let mut canvas = context.begin_drawing(&thread);
                        canvas.clear_background(config.background_color);
                        draw_content(&mut canvas, &content, &font, &config);
                        draw_cursor(&mut canvas, &cursor, &config);
                }

                if let Some(action) = input.action(&mut context) {
                        match action {
                                Action::Quit => break,
                                _ => {}
                        }
                        content.update(&action);
                        cursor.update(&content, &action);
                }
        }
}

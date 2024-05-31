mod config;
mod content;
mod cursor;
mod draw;
mod input;

use config::Config;
use content::Content;
use cursor::Cursor;
use input::{
        Action,
        Input,
};
use raylib::color::Color;

fn main()
{
        let config = Config::new(
                600,
                480,
                "Sofa",
                "/usr/share/fonts/TTF/AgaveNerdFont-Regular.ttf",
                19.0,
                1.0,
                Color::BLACK,
                Color::WHITE.alpha(0.8),
                Color::PINK,
        );

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

        let mut input = Input::new();
        let mut content = Content::new();
        let mut cursor = Cursor::new(&config);

        while !context.window_should_close() {
                {
                        let mut canvas = context.begin_drawing(&thread);
                        draw::background(&mut canvas, &config);
                        draw::content(&mut canvas, &content, &font, &config);
                        draw::cursor(&mut canvas, &cursor, &config);
                }

                if let Some(action) = input.action(&mut context) {
                        if action == Action::Quit {
                                break;
                        }
                        content.update(&action);
                        cursor.update(&content, &action);
                }
        }
}

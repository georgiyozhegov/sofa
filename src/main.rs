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
                600,                                                   // width
                480,                                                   // height
                "Sofa",                                                // title
                "/usr/share/fonts/TTF/IosevkaNerdFontMono-Medium.ttf", // font
                23.0,                                                  // font size
                0.4,                                                   // font scale
                1.0,                                                   // font spacing
                Color::BLACK,                                          // font color
                Color::WHITE.alpha(0.8),                               // cursor color
                Color::PINK,                                           // background color
                23,                                                    // status line height
                Color::WHITE.alpha(0.9),                               // status line color
                4,                                                     // tab size
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
        let mut content = Content::new(&config);
        let mut cursor = Cursor::new(&config);

        while !context.window_should_close() {
                {
                        let mut canvas = context.begin_drawing(&thread);
                        draw::background(&mut canvas, &config);
                        draw::content(&mut canvas, &content, &font, &config);
                        draw::cursor(&mut canvas, &cursor, &config);
                        draw::status_line(&mut canvas, &input, &font, &config);
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

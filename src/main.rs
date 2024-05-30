mod content;
mod input;

use content::Content;
use input::{
        Action,
        Direction,
        Input,
};

use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use raylib::prelude::Vector2;
use raylib::text::RaylibFont;

fn main()
{
        // constants (will replaced by config)
        let width = 600;
        let height = 480;
        let font_path = "/usr/share/fonts/TTF/AgaveNerdFont-Regular.ttf";
        let font_size = 20.0;
        let font_spacing = 1.0;
        let font_color = Color::BLACK;

        // init
        let (mut context, thread) = raylib::init()
                .size(width, height)
                .resizable()
                .title("Sofa")
                .build();
        context.set_target_fps(60);
        context.set_exit_key(None);

        // load font
        let font = context
                .load_font_ex(&thread, font_path, font_size as i32, None)
                .unwrap();

        // input handler
        let mut input = Input::new();

        // content handler
        let mut content = Content::new();

        // mainloop
        while !context.window_should_close() {
                {
                        let mut canvas = context.begin_drawing(&thread);
                        canvas.clear_background(Color::PLUM);
                        canvas.draw_text_ex(
                                &font,
                                &content.to_string(),
                                Vector2::new(0.0, 0.0),
                                font_size,
                                font_spacing,
                                font_color,
                        );
                        canvas.draw_rectangle(
                                content.column as i32 * (font_size / 2.0 + font_spacing) as i32,
                                content.row as i32 * font_size as i32,
                                (font_size / 2.0 + font_spacing) as i32,
                                font_size as i32,
                                Color::WHEAT,
                        );
                }

                if let Some(action) = input.action(&mut context) {
                        println!("{action:?}");
                        match action {
                                Action::Quit => break,
                                Action::Insert(c) => content.insert(c),
                                Action::Delete => content.delete(),
                                Action::NewLine => content.new_line(),
                                Action::Move(Direction::Left) => content.move_left(),
                                Action::Move(Direction::Down) => content.move_down(),
                                Action::Move(Direction::Up) => content.move_up(),
                                Action::Move(Direction::Right) => content.move_right(),
                        }
                }
        }
}

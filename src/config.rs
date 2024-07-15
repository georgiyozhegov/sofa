use raylib::color::Color;

#[derive(Debug)]
pub struct Config
{
        pub window_width: i32,
        pub window_height: i32,
        pub window_title: &'static str,
        pub font_path: &'static str,
        pub font_size: f32,
        pub font_scale: f32,
        pub font_spacing: f32,
        pub font_color: Color,
        pub cursor_color: Color,
        pub background_color: Color,
        pub status_line_height: i32,
        pub status_line_color: Color,
}

impl Config
{
        pub fn new(
                window_width: i32,
                window_height: i32,
                window_title: &'static str,
                font_path: &'static str,
                font_size: f32,
                font_scale: f32,
                font_spacing: f32,
                font_color: Color,
                cursor_color: Color,
                background_color: Color,
                status_line_height: i32,
                status_line_color: Color,
        ) -> Self
        {
                Self {
                        window_width,
                        window_height,
                        window_title,
                        font_path,
                        font_size,
                        font_scale,
                        font_spacing,
                        font_color,
                        cursor_color,
                        background_color,
                        status_line_height,
                        status_line_color,
                }
        }
}

use crate::input::{
        Action,
        Direction,
        Item,
        Location,
};
use crate::config::Config;

pub struct Content
{
        pub content: Vec<String>,
        pub row: usize,
        pub column: usize,
        tab_size: usize,
}

impl Content
{
        pub fn new(config: &Config) -> Self
        {
                Self {
                        content: vec![String::new()],
                        row: 0,
                        column: 0,
                        tab_size: config.tab_size,
                }
        }

        pub fn insert(&mut self, c: char)
        {
                self.content[self.row].insert(self.column, c);
                self.column += 1;
        }

        pub fn delete_char(&mut self)
        {
                if self.column > 0 {
                        self.content[self.row].remove(self.column - 1);
                        self.column -= 1;
                }
                else if self.row > 0 {
                        let line = self.content[self.row].clone();
                        let index = self.column;
                        self.content[self.row] = line[..index].to_string();
                        self.row -= 1;
                        self.column = self.content[self.row].len();
                        self.content[self.row].push_str(&line[index..]);
                }
        }

        pub fn delete_line(&mut self)
        {
                if self.row == 0 {
                        self.content[self.row].clear();
                        self.column = 0;
                }
                else {
                        self.content.remove(self.row);
                        self.row -= 1;
                        self.column = self.content[self.row].len();
                }
        }

        pub fn new_line(&mut self)
        {
                let line = self.content[self.row].clone();
                self.content
                        .insert(self.row + 1, line[self.column..].to_string());
                self.content[self.row] = line[..self.column].to_string();
                self.row += 1;
                self.column = 0;
        }

        pub fn tab(&mut self) 
        {
                for _ in 0..self.tab_size {
                    self.insert(' ');
                }
        }

        pub fn move_left(&mut self)
        {
                if self.column > 0 {
                        self.column -= 1;
                }
        }

        pub fn move_down(&mut self)
        {
                if self.row + 1 < self.content.len() {
                        self.row += 1;
                        self.column = self.content[self.row].len().min(self.column);
                }
        }

        pub fn move_up(&mut self)
        {
                if self.row > 0 {
                        self.row -= 1;
                        self.column = self.content[self.row].len().min(self.column);
                }
        }

        pub fn move_right(&mut self)
        {
                if self.column < self.content[self.row].len() {
                        self.column += 1;
                }
        }

        pub fn go_to_top(&mut self)
        {
                self.row = 0;
                self.column = self.content[self.row].len().min(self.column);
        }

        pub fn go_to_bottom(&mut self)
        {
                self.row = self.content.len().max(1) - 1;
                self.column = self.content[self.row].len().min(self.column);
        }

        pub fn update(&mut self, action: &Action)
        {
                match action {
                        Action::Insert(c) => self.insert(*c),
                        Action::Delete(Item::Char) => self.delete_char(),
                        Action::Delete(Item::Line) => self.delete_line(),
                        Action::NewLine => self.new_line(),
                        Action::Tab => self.tab(),
                        Action::Move(Direction::Left) => self.move_left(),
                        Action::Move(Direction::Down) => self.move_down(),
                        Action::Move(Direction::Up) => self.move_up(),
                        Action::Move(Direction::Right) => self.move_right(),
                        Action::GoTo(Location::Top) => self.go_to_top(),
                        Action::GoTo(Location::Bottom) => self.go_to_bottom(),
                        _ => {}
                }
        }
}

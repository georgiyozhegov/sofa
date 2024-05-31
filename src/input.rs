use raylib::prelude::{
        KeyboardKey,
        RaylibHandle,
};

#[derive(Debug, PartialEq)]
pub enum Mode
{
        Insert,
        Base,
}

#[derive(Debug, PartialEq)]
pub enum Direction
{
        Left,
        Down,
        Up,
        Right,
}

#[derive(Debug, PartialEq)]
pub enum Action
{
        Insert(char),
        Delete,
        NewLine,
        Quit,
        Move(Direction),
        Switch(Mode),
}

pub struct Input
{
        pub mode: Mode,
}

impl Input
{
        pub fn new() -> Self
        {
                Self { mode: Mode::Base }
        }

        fn insert_mode_action(
                &mut self,
                key: KeyboardKey,
                context: &mut RaylibHandle,
        ) -> Option<Action>
        {
                match key {
                        KeyboardKey::KEY_ESCAPE => {
                                self.mode = Mode::Base;
                                Some(Action::Switch(Mode::Base))
                        }
                        KeyboardKey::KEY_BACKSPACE => Some(Action::Delete),
                        KeyboardKey::KEY_ENTER => Some(Action::NewLine),
                        _ => Some(Action::Insert(context.get_char_pressed()?)),
                }
        }

        fn base_mode_action(&mut self, key: KeyboardKey) -> Option<Action>
        {
                match key {
                        KeyboardKey::KEY_ESCAPE => {
                                self.mode = Mode::Insert;
                                Some(Action::Switch(Mode::Insert))
                        }
                        KeyboardKey::KEY_Q => Some(Action::Quit),
                        KeyboardKey::KEY_H => Some(Action::Move(Direction::Left)),
                        KeyboardKey::KEY_J => Some(Action::Move(Direction::Down)),
                        KeyboardKey::KEY_K => Some(Action::Move(Direction::Up)),
                        KeyboardKey::KEY_L => Some(Action::Move(Direction::Right)),
                        _ => None,
                }
        }

        pub fn action(&mut self, context: &mut RaylibHandle) -> Option<Action>
        {
                if let Some(key) = context.get_key_pressed() {
                        match self.mode {
                                Mode::Insert => self.insert_mode_action(key, context),
                                Mode::Base => self.base_mode_action(key),
                        }
                }
                else {
                        None
                }
        }
}

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
pub enum Modifier
{
        Delete,
        GoTo,
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
pub enum Location
{
        Top,
        Bottom,
}

#[derive(Debug, PartialEq)]
pub enum Item
{
        Char,
        Line,
}

#[derive(Debug, PartialEq)]
pub enum Action
{
        Insert(char),
        Delete(Item),
        NewLine,
        Tab,
        Quit,
        Move(Direction),
        GoTo(Location),
        Switch(Mode),
}

pub struct Input
{
        pub mode: Mode,
        pub modifier: Option<Modifier>,
}

impl Input
{
        pub fn new() -> Self
        {
                Self {
                        mode: Mode::Base,
                        modifier: None,
                }
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
                        KeyboardKey::KEY_BACKSPACE => Some(Action::Delete(Item::Char)),
                        KeyboardKey::KEY_ENTER => Some(Action::NewLine),
                        KeyboardKey::KEY_TAB => Some(Action::Tab),
                        _ => Some(Action::Insert(context.get_char_pressed()?)),
                }
        }

        fn base_mode_action_with_delete_modifier(&mut self, key: KeyboardKey) -> Option<Action>
        {
                match key {
                        KeyboardKey::KEY_C => Some(Action::Delete(Item::Char)),
                        KeyboardKey::KEY_L => Some(Action::Delete(Item::Line)),
                        KeyboardKey::KEY_Q => {
                                self.modifier = None;
                                None
                        }
                        _ => None,
                }
        }

        fn base_mode_action_with_go_to_modifier(&mut self, key: KeyboardKey) -> Option<Action>
        {
                match key {
                        KeyboardKey::KEY_T => Some(Action::GoTo(Location::Top)),
                        KeyboardKey::KEY_B => Some(Action::GoTo(Location::Bottom)),
                        KeyboardKey::KEY_Q => {
                                self.modifier = None;
                                None
                        }
                        _ => None,
                }
        }

        fn base_mode_action_with_modifier(&mut self, key: KeyboardKey) -> Option<Action>
        {
                let action = match self.modifier {
                        Some(Modifier::Delete) => self.base_mode_action_with_delete_modifier(key),
                        Some(Modifier::GoTo) => self.base_mode_action_with_go_to_modifier(key),
                        _ => None,
                };
                if action.is_some() {
                        self.modifier = None;
                }
                action
        }

        fn base_mode_action(&mut self, key: KeyboardKey) -> Option<Action>
        {
                if self.modifier.is_some() {
                        self.base_mode_action_with_modifier(key)
                }
                else {
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
                                KeyboardKey::KEY_D => {
                                        self.modifier = Some(Modifier::Delete);
                                        None
                                }
                                KeyboardKey::KEY_G => {
                                        self.modifier = Some(Modifier::GoTo);
                                        None
                                }
                                _ => None,
                        }
                }
        }

        pub fn action(&mut self, context: &mut RaylibHandle) -> Option<Action>
        {
                let key = context.get_key_pressed()?;
                match self.mode {
                        Mode::Insert => self.insert_mode_action(key, context),
                        Mode::Base => self.base_mode_action(key),
                }
        }
}

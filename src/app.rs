use crate::asset::CompanyInfo;
use crate::util::TabsState;
use std::collections::HashMap;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App<'a> {
    pub title: &'a str,
    pub show_chart: bool,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub input_mode: InputMode,
    pub input: String,
    pub company: Option<CompanyInfo>,
    pub symbol: String,
    pub config: HashMap<String, String>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, symbol: String, config: HashMap<String, String>) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Tab0", "Tab1"]),
            show_chart: true,
            input_mode: InputMode::Normal,
            input: String::new(),
            company: None,
            symbol,
            config,
        }
    }

    pub fn on_up(&mut self) {
        //self.tasks.previous();
    }

    pub fn on_down(&mut self) {
        //self.tasks.next();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                if let InputMode::Normal = self.input_mode {
                    self.should_quit = true;
                } else {
                    self.input.push('q');
                }
            }
            '/' => match self.input_mode {
                InputMode::Normal => self.input_mode = InputMode::Editing,
                InputMode::Editing => self.input_mode = InputMode::Normal,
            },
            _ => {
                if let InputMode::Editing = self.input_mode {
                    self.input.push(c);
                }
            }
        }
    }

    pub fn on_backspace(&mut self) {
        self.input.pop();
    }

    pub fn on_enter(&mut self) {
        self.symbol = self.input.to_uppercase().clone();
        self.company = None;
        self.input.clear();
    }

    pub fn on_escape(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub fn on_tick(&mut self) {
        // Update progress
    }
}

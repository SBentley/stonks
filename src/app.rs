use crate::asset::Company;
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
    pub company: Option<Company>,
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
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update progress
    }
}

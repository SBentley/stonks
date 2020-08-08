use crate::asset::{CompanyInfo, Stock};
use crate::search_page::SearchEngine;
use crate::util::TabsState;
use simsearch::SimSearch;
use std::collections::HashMap;

pub enum InputMode {
    Normal,
    Editing,
}

pub enum State {
    Search,
    Normal,
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
    pub state: State,
    pub securities: Vec<Stock>,
    pub search_engine: Option<SearchEngine>,
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
            state: State::Normal,
            securities: Vec::new(),
            search_engine: None,
        }
    }

    pub fn on_up(&mut self) {}

    pub fn on_down(&mut self) {}

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
                    self.search_text_input(c);
                }
            }
            '/' => match self.input_mode {
                InputMode::Normal => self.input_mode = InputMode::Editing,
                InputMode::Editing => self.input_mode = InputMode::Normal,
            },
            _ => {
                if let InputMode::Editing = self.input_mode {
                    self.search_text_input(c);
                }
            }
        }
    }

    fn search_text_input(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn on_backspace(&mut self) {
        self.input.pop();
    }

    pub fn on_enter(&mut self) {
        self.state = State::Search;
        //self.symbol = self.input.to_uppercase().clone();
        self.company = None;
        //self.input.clear();
    }

    pub fn on_escape(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    pub fn on_tick(&mut self) {
        // Update progress
    }
}

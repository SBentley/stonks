use crate::app::{App, InputMode};
use crate::asset;
use crate::asset::Stock;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use simsearch::SimSearch;
#[allow(unused_imports)]
use std::{sync::mpsc::Receiver, time::Duration};

pub struct SearchEngine {
    pub engine: SimSearch<Stock>,
}

impl SearchEngine {
    pub fn new(stocks: &mut Vec<Stock>) -> SearchEngine {
        SearchEngine {
            engine: fill_engine(stocks),
        }
    }
}

fn fill_engine(stocks: &mut Vec<Stock>) -> SimSearch<Stock> {
    let mut engine: SimSearch<Stock> = SimSearch::new();
    for stock in stocks {
        engine.insert(stock.clone(), &stock.description);
    }
    engine
}

pub fn search(query: &str) {}

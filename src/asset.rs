//#[macro_use] extern crate hyper;

use std::collections::HashMap;
pub struct Equity {
    name: String,
    ticker: String,
    current_price: f64,
}

impl Equity {
    pub fn new(name: String, ticker: String, current_price: f64) -> Equity {
        Equity {
            name,
            ticker,
            current_price,
        }
    }
}

pub fn get_test_equity() -> Equity {
    Equity {
        name: String::from("placeholder"),
        ticker: String::from("PLC"),
        current_price: 1.0,
    }
}


#[tokio::main]
pub async fn get_equity() -> Result<Equity, Box<dyn std::error::Error>> {
    let symbol = String::from("AAPL");
    
    let resp = reqwest::get("https://finnhub.io/api/v1/stock/profile2?symbol=AAPL")
    .await?
    .json::<HashMap<String, String>>()
    .await?;
    
    //request.header("X-Finnhub-Token", api_key);
    println!("{:#?}", resp);
    Ok(get_test_equity())
}


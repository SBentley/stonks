use chrono::{offset::Utc, DateTime, Duration};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use serde::Deserialize;
use std::time::{SystemTime};
use reqwest::Client;

#[derive(Deserialize, Debug)]
pub struct Company {
    pub name: String,
    pub ticker: String,
    pub country: String,
    #[serde(rename(deserialize = "marketCapitalization"))]
    pub market_capitalization: f64,
    pub exchange: String,
    pub currency: String,
    #[serde(rename(deserialize = "finnhubIndustry"))]
    pub industry: String,
    #[serde(skip)]
    pub prices: Prices,
}

#[derive(Deserialize, Debug, Default)]
pub struct Prices {
    #[serde(rename(deserialize = "c"))]
    pub close: Vec<f32>,
}

pub fn get_error_company() -> Company {
    Company {
        name: String::from("Error getting comapny info"),
        ticker: String::from("PLC"),
        country: String::from("UK"),
        market_capitalization: 100.0,
        exchange: String::from("LSE"),
        currency: String::from("USD"),
        industry: String::from("Technology"),
        prices: Prices {
            close: vec![1.0, 2.0, 3.0],
        },
    }
}

#[tokio::main]
pub async fn get_equity(
    api_key: &str,
    symbol: &str,
) -> Result<Company, Box<dyn std::error::Error>> {    
    
    let url = format!("https://finnhub.io/api/v1/stock/profile2?symbol={}", symbol);
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("X-Finnhub-Token", api_key)
        .send()
        .await?;
    info!("{}", resp.status());

    if resp.status().is_success() {
        let mut company: Company = resp.json().await?;
        company.market_capitalization *= 1000000.0;
        match  get_price_history(api_key, symbol, "1", client) { 
            Ok(res) => company.prices = res,
            _ => {}
        }
        info!("{:#?}", company);
        Ok(company)
    } else {
        // TODO: fix error handling here, do not return ok if not ok
        Ok(get_error_company())
    }
}

#[tokio::main]
pub async fn get_price_history(
    api_key: &str,
    symbol: &str,
    resolution: &str,
    client: Client
) -> Result<Prices, Box<dyn std::error::Error>> {
    
    let now = SystemTime::now();    
    let now: DateTime<Utc> = now.into();
    let one_year_ago = now - Duration::days(365);
    let now = now.format("%s").to_string();
    let one_year_ago = one_year_ago.format("%s").to_string();

    info!("Unix timestamp: {}", now);
    info!("Unix - 1 year: {}", one_year_ago);
    let url = format!("https://finnhub.io/api/v1/stock/candle?symbol={}&resolution={}&from={}&to={}", symbol, resolution, now, one_year_ago);
    //let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("X-Finnhub-Token", api_key)
        .send()
        .await?;
    info!("{}", resp.status());

    //request.header("X-Finnhub-Token", api_key);
    if resp.status().is_success() {
        let prices: Prices = resp.json().await?;
        info!("{:#?}", prices);
        Ok(prices)
    } else {
        // TODO: fix error handling here, do not return ok if not ok
        Ok(Prices { close: vec![0.0] })
    }
}

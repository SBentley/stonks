use chrono::{offset::Utc, DateTime, Duration};
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use serde::Deserialize;
use std::{net::TcpStream, sync::mpsc, time::SystemTime};
use tungstenite::{http, stream::Stream, Message, WebSocket};
//use serde_json::{Result};
use http::Response;
use mpsc::Sender;
use native_tls::TlsStream;

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
#[derive(Deserialize, Debug)]
pub struct LiveData {
    #[serde(rename(deserialize = "p"))]
    pub price: f32,
    #[serde(rename(deserialize = "s"))]
    pub symbol: String,
    #[serde(rename(deserialize = "t"))]
    pub time: u64,
    #[serde(rename(deserialize = "v"))]
    pub volume: f32,
}

#[derive(Deserialize, Debug)]
pub struct Feed {
    pub data: Option<Vec<LiveData>>,
    #[serde(rename(deserialize = "type"))]
    pub message_type: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Prices {
    #[serde(rename(deserialize = "c"))]
    pub close: Vec<f64>,
    #[serde(skip)]
    pub live_price: f64,
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
            live_price: 1.0,
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
) -> Result<Prices, Box<dyn std::error::Error>> {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    let one_year_ago = now - Duration::days(365);
    let now = now.format("%s").to_string();
    let one_year_ago = one_year_ago.format("%s").to_string();

    info!("Unix timestamp: {}", now);
    info!("Unix - 1 year: {}", one_year_ago);
    let url = format!(
        "https://finnhub.io/api/v1/stock/candle?symbol={}&resolution={}&from={}&to={}",
        symbol, resolution, one_year_ago, now
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("X-Finnhub-Token", api_key)
        .send()
        .await?;
    info!("Prices status {}", resp.status());

    if resp.status().is_success() {
        info!("price success");
        let prices: Prices = resp.json().await?;
        info!("Prices: {:#?}", prices.close.len());
        Ok(prices)
    } else {
        // TODO: fix error handling here, do not return ok if not ok
        Ok(Prices {
            close: vec![0.0],
            live_price: 1.0,
        })
    }
}

// TODO: Create type alias.
pub fn live_price(
    symbol: &str,
    mut socket: WebSocket<Stream<TcpStream, TlsStream<TcpStream>>>,
    response: Response<()>,
    tx: Sender<f64>,
) {
    loop {
        let msg = socket.read_message().expect("Error reading message");
        //        println!("{}",msg);
        if let Message::Text(text) = msg {
            let msg: Feed = serde_json::from_str(&text).unwrap();
            if let Some(data) = msg.data {
                let price: f64 = data[0].price as f64;
                info!("ws prices: {}", price);
                tx.send(price)
                    .expect("Error sending ws data between threads");
            }
        }
    }
}

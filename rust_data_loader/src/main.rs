use std::{thread, time};
use serde_json;
use ureq;

#[derive(Debug)]
struct Bitcoin {
    api_address: String,
    file_name: String,
}

#[derive(Debug)]
struct Ethereum {
    api_address: String,
    file_name: String,
}

#[derive(Debug)]
struct SP500 {
    api_address: String,
    file_name: String,
}

pub trait Pricing {
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self, price: f32);
}

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> f32 {
        let response = ureq::get(&self.api_address).call().unwrap();
        let body = response.into_string().unwrap();
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        json["bitcoin"]["usd"].as_f64().unwrap() as f32
    }

    fn save_to_file(&self, price: f32) {
        std::fs::write(&self.file_name, format!("{{\"price\": {}}}\n", price)).unwrap();
        println!("Saved Bitcoin price to {}", self.file_name);
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&self) -> f32 {
        let response = ureq::get(&self.api_address).call().unwrap();
        let body = response.into_string().unwrap();
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        json["ethereum"]["usd"].as_f64().unwrap() as f32
    }

    fn save_to_file(&self, price: f32) {
        std::fs::write(&self.file_name, format!("{{\"price\": {}}}\n", price)).unwrap();
        println!("Saved Ethereum price to {}", self.file_name);
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> f32 {
        let response = ureq::get(&self.api_address).call().unwrap();
        let body = response.into_string().unwrap();
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        json["chart"]["result"][0]["indicators"]["quote"][0]["close"]
            .as_array()
            .and_then(|arr| arr.last())
            .and_then(|value| value.as_f64())
            .unwrap_or_else(|| {
                println!("Failed to fetch S&P 500 price.");
                0.0
            }) as f32
    }

    fn save_to_file(&self, price: f32) {
        std::fs::write(&self.file_name, format!("{{\"price\": {}}}\n", price)).unwrap();
        println!("Saved S&P 500 price to {}", self.file_name);
    }
}

#[derive(Debug)]
enum Asset {
    Bitcoin(Bitcoin),
    Ethereum(Ethereum),
    SP500(SP500),
}

impl Asset {
    fn fetch_price(&self) -> f32 {
        match self {
            Asset::Bitcoin(asset) => asset.fetch_price(),
            Asset::Ethereum(asset) => asset.fetch_price(),
            Asset::SP500(asset) => asset.fetch_price(),
        }
    }

    fn save_to_file(&self, price: f32) {
        match self {
            Asset::Bitcoin(asset) => asset.save_to_file(price),
            Asset::Ethereum(asset) => asset.save_to_file(price),
            Asset::SP500(asset) => asset.save_to_file(price),
        }
    }
}

fn main() {
    let bitcoin = Asset::Bitcoin(Bitcoin {
        api_address: "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string(),
        file_name: "bitcoin_prices.json".to_string(),
    });

    let ethereum = Asset::Ethereum(Ethereum {
        api_address: "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string(),
        file_name: "ethereum_prices.json".to_string(),
    });

    let sp500 = Asset::SP500(SP500 {
        api_address: "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d".to_string(),
        file_name: "sp500_prices.json".to_string(),
    });

    let assets = vec![bitcoin, ethereum, sp500];

    let interval = time::Duration::from_secs(10);
    loop {
        for asset in &assets {
            let price = asset.fetch_price();
            asset.save_to_file(price);
        }
        thread::sleep(interval);
    }
}

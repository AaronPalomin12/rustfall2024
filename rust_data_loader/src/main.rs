// I'm using thread and time to handle the delays and being able to sleep intervals.
use std::{thread, time};
// I'm using serde_json so it able to parse JSON as it responses from APIs.
use serde_json;
// I'm using ureq to help for the HTTP requests as it fetching APIs data.
use ureq;


// I'm using the api_address for the API URL for fetching prices.
// I'm using file_name so it's able to save the price data.
// I'm using the #[derive(Debug)] so it helps me make debugging points so see struct values.

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


// I created Pricing as a shared interface with the made fetch_price to it being saved into a file named save_to_file.
pub trait Pricing {
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self, price: f32);
}

// I created Pricing for Bitcoin to save and save the price while also fethcing Bitcoin price from API and parses.
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

// I created Pricing for Ethereum to save and save the price while also fethcing Ethereum price from API and parses.
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

// I created Pricing for SP500 to save and save the price while also fethcing SP500 price from API and parses.
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

// I used enum as it represents the assest and also being their implementation of Pricing.
#[derive(Debug)]
enum Asset {
    Bitcoin(Bitcoin),
    Ethereum(Ethereum),
    SP500(SP500),
}


// I implemented methods for the Asset num to be able to help me handle the saving the prices and fetching.
impl Asset {
    // It fetches the asset as it delegating to make sure it is appropriate for implementation.
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
    // I created this to initialize the Bitcoin asset as it handles the endpoint of the API and the output file.
    let bitcoin = Asset::Bitcoin(Bitcoin {
        api_address: "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd".to_string(),
        file_name: "bitcoin_prices.json".to_string(),
    });

    // I created this to initialize the Ethereum asset as it handles the endpoint of the API and the output file.
    let ethereum = Asset::Ethereum(Ethereum {
        api_address: "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd".to_string(),
        file_name: "ethereum_prices.json".to_string(),
    });

    // I created this to initialize the SP500 asset as it handles the endpoint of the API and the output file.
    let sp500 = Asset::SP500(SP500 {
        api_address: "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d".to_string(),
        file_name: "sp500_prices.json".to_string(),
    });

    // I made this to create a list that contains all the assets that need to be monitored.
    let assets = vec![bitcoin, ethereum, sp500];

    // I made this to set the interval so it's able to save and fetch the data needed.
    let interval = time::Duration::from_secs(10);

    // I made this infinite loop that is able to periodically fetch and save the needed prices for all the assets that are available.
    loop {
        for asset in &assets {
            let price = asset.fetch_price();
            asset.save_to_file(price);
        }
        thread::sleep(interval);
    }
}

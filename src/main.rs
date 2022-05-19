use log::{info};
use log4rs;

mod rest_client;


#[tokio::main]
async fn main() {
    log4rs::init_file("logconfig.yaml", Default::default()).expect("Log config file not found.");
    info!("We now have nice logging!");
    let client = rest_client::Client::new("https://api.kucoin.com".to_string());
    let result: serde_json::Value = client.get("/api/v1/market/orderbook/level1?symbol=BTC-USDT", None).await.unwrap();
    info!("KuCoin result: {:?}", result);
    let bnb_client = rest_client::Client::new("https://api1.binance.com".to_string());
    let bnb_result: serde_json::Value = bnb_client.get("/api/v3/ticker/price?symbol=BTCUSDT", None).await.unwrap();
    info!("bnb  result: {:?}", bnb_result);
    let ftx_client = rest_client::Client::new("https://ftx.com/api".to_string());
    let ftx_result: serde_json::Value = ftx_client.get("/markets/BTC/USDT", None).await.unwrap();
    info!("ftx result: {:?}", ftx_result);
}
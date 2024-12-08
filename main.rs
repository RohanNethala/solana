mod monitor;
mod signal;
mod route;
mod transaction;

use solana_client::rpc_client::RpcClient;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    let (price_sender, mut price_receiver) = tokio::sync::broadcast::channel(100);

    // Subscribe to price updates
    monitor::subscribe_to_amm_and_serum(
        "wss://amm_pool_url".to_string(),
        "wss://serum_market_url".to_string(),
        price_sender,
    )
    .await;

    let mut previous_price = 0.0;

    while let Ok((price_amm, price_serum)) = price_receiver.recv().await {
        let current_volume = 1.0; // Example value
        let min_volume = 0.5;

        if signal::check_entry_signal(price_amm, price_serum, previous_price, 0.05, min_volume, current_volume) {
            let route = route::choose_route(0.02, 0.03);

            let instructions = match route {
                "AMM" => transaction::create_amm_instructions(),
                "Orderbook" => transaction::create_orderbook_instructions(),
                _ => vec![],
            };

            if let Err(e) = transaction::execute_transaction(&client, instructions).await {
                eprintln!("Transaction failed: {}", e);
            }
        }

        previous_price = price_amm;
    }
}

use solana_client::rpc_websocket::RpcWebSocketClient;
use tokio::sync::broadcast;

pub async fn subscribe_to_amm_and_serum(
    amm_pool: String,
    serum_market: String,
    sender: broadcast::Sender<(f64, f64)>,
) {
    let amm_client = RpcWebSocketClient::new(amm_pool).await.unwrap();
    let serum_client = RpcWebSocketClient::new(serum_market).await.unwrap();

    tokio::spawn(async move {
        loop {
            if let Ok(price_amm) = amm_client.recv().await {
                if let Ok(price_serum) = serum_client.recv().await {
                    sender.send((price_amm, price_serum)).unwrap();
                }
            }
        }
    });
}

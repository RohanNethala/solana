pub fn choose_route(
    amm_slippage: f64,
    orderbook_slippage: f64,
) -> &'static str {
    if amm_slippage < orderbook_slippage {
        "AMM"
    } else {
        "Orderbook"
    }
}

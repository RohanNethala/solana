pub fn check_entry_signal(
    price_amm: f64,
    price_serum: f64,
    previous_price: f64,
    delta_entry: f64,
    min_volume: f64,
    current_volume: f64,
) -> bool {
    let delta_price = (price_amm - previous_price).abs();
    let effective_price = price_amm.min(price_serum);

    delta_price >= delta_entry && current_volume >= min_volume
}

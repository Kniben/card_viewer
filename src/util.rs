pub fn is_within(x: f64, range_mid: f64, width: f64) -> bool {
    let half_width = width * 0.5;
    range_mid - half_width <= x && x < range_mid + half_width
}

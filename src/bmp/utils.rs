//converts BGR tuple to grayscale
pub fn rgb_to_greyscale(bgr: (u8, u8, u8)) -> (u8, u8, u8) {
    let (b, g, r) = bgr;
    let grey_value = (0.299 * f64::from(r) + 0.587 * f64::from(g) + 0.114 * f64::from(b)).round() as u8;

    (grey_value, grey_value, grey_value)
}


pub fn round_up_to_multiple_of_four(value: u32) -> u32 {
    ((value + 3) / 4) * 4
}
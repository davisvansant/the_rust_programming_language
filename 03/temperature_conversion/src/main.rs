fn convert_f_to_c(_t: f64) -> f64 {
    let f = _t - 32.0;
    let c = 0.5556;
    f * c
}

fn convert_c_to_f(_t: f64) -> f64 {
    let c = 1.8 * _t;
    c + 32.0
}

fn main() {
    let celsius = convert_f_to_c(85.0);
    println!("{:?}", celsius);

    let fahrenheit = convert_c_to_f(19.0);
    println!("{:?}", fahrenheit);
}

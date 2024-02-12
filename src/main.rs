use std::env;

use temperature::Temperature;
mod temperature;


fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let temperature = Temperature::new("/sys/class/thermal/thermal_zone0/temp".to_string());
    println!("{:#?}", temperature.temperature().unwrap());
}

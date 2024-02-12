use std::env;

mod temperature;
use crate::temperature::temperature::Temperature;



fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "full");
    env_logger::init();
    let path = "/sys/class/thermal/thermal_zone0/temp".to_string();
    let temperature = Temperature::new(path);
    println!("{:#?}", temperature.temperature().unwrap());
}

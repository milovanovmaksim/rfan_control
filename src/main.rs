use log::error;

mod temperature;
mod fan;
mod fan_control;
use fan::fan::Fan;
use fan_control::fan_control::FanControl;
use rppal::pwm::{Channel, Polarity, Pwm};
use crate::temperature::temperature::Temperature;



fn main(){
    // env::set_var("RUST_LOG", "debug");
    // env::set_var("RUST_BACKTRACE", "full");
    // env_logger::init();
    let temp_min = 40;
    let temp_max = 60;
    let delay = 2;
    let path = "/sys/class/thermal/thermal_zone0/temp";
    let fan_low = 0.2;
    let fan_high = 1.0;
    let pwm_freq = 1000.0;
    let fan_pwm = Pwm::with_frequency(Channel::Pwm0, pwm_freq, fan_low, Polarity::Normal, false).unwrap();
    let temperature = Temperature::new(path.to_string());
    let fan = Fan::new(temperature, temp_min, temp_max, fan_low, fan_high, fan_pwm);
    let mut fan_control = FanControl::new(fan, delay);
    match fan_control.run() {
        Ok(_) => {  },
        Err(errors) => {
            for error in errors {
                error!("main | error: {}", error)
            }
        }
    }
}
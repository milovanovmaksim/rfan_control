use std::{env, thread, time::Duration};

mod temperature;
mod fan;
use fan::fan::Fan;
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
    let mut fan = Fan::new(temperature, temp_min, temp_max, fan_low, fan_high, fan_pwm);
    fan.start().unwrap();
    loop {

        fan.update_duty_cycle();
        thread::sleep(Duration::from_secs(delay));
    }
}
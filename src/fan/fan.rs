use log::{debug, error};
use rppal::pwm::{Pwm, Result as PwmResult};

use crate::temperature::temperature::Temperature;


pub(crate) struct Fan {
    temperature: Temperature,
    temp_min: u64,
    temp_max: u64,
    fan_low: u64,
    fan_high: u64,
    fan_pwm: Pwm
}

impl Fan {
    pub fn new(temperature: Temperature, temp_min: u64, temp_max: u64, fan_low: u64, fan_high: u64, fan_pwm: Pwm) -> Self {
        Fan { temperature, temp_min, temp_max, fan_low, fan_high, fan_pwm}
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.fan_pwm.set_reset_on_drop(true);
        match self.fan_pwm.enable() {
            Ok(_) => {
                debug!("Fan::start | The fan has been started.");
                Ok(())
            }
            Err(error) => {
                error!("Fan::duty_cycle | error: {}", error);
                Err(error.to_string())
            }
        }
    }

    fn duty_cycle(&self) -> Result<f64, String> {
       let temperature = self.temperature.temperature();
        match temperature {
            Ok(temperature) => {
                let mut frequency = 0;
                if temperature > self.temp_max {
                    frequency = 100;
                } else if temperature > self.temp_min {
                    let delta = self.fan_high - self.fan_low;
                    frequency = delta * (temperature - self.temp_min) / (self.temp_max - self.temp_min) + self.fan_low;
                }
                debug!("Fan::duty_cycle | frequency = {}, temperature = {}", (frequency), temperature);
                Ok(frequency as f64 / 100.0)
            }
            Err(error) => {
                error!("Fan::duty_cycle | error: {}", error);
                Err(error)
            },
        }
    }

    pub fn update_duty_cycle(&mut self) -> Result<(), String> {
        match self.duty_cycle() {
            Ok(duty_cycle) => {
                self.fan_pwm.set_duty_cycle(duty_cycle).map_err(|error| {
                    error!("Fan::update_duty_cycle | error: {}", error);
                    error.to_string()
                })
            }
            Err(error) => {
                error!("Fan::update_duty_cycle | error: {}", error);
                Err(error)
            }
        }
    }

    pub fn stop(&self) -> PwmResult<()> {
        match self.fan_pwm.disable() {
            Ok(_) => {
                debug!("Fan::stop | The fan is stopping.");
                Ok(())
            },
            Err(error) => {
                error!("Fan::stop | error: {}", error);
                Err(error)
            }
        }
    }
}
use log::{debug, error};
use rppal::pwm::{Pwm, Result as PwmResult};

use crate::temperature::temperature::Temperature;


pub(crate) struct Fan {
    temperature: Temperature,
    temp_min: u32,
    temp_max: u32,
    fan_low: f64,
    fan_high: f64,
    fan_pwm: Pwm
}

impl Fan {
    pub fn new(temperature: Temperature, temp_min: u32, temp_max: u32, fan_low: f64, fan_high: f64, fan_pwm: Pwm) -> Self {
        Fan { temperature, temp_min, temp_max, fan_low, fan_high, fan_pwm}
    }

    pub fn start(&mut self) -> Result<(), String> {
        self.fan_pwm.set_reset_on_drop(true);
        debug!("Fan::start | The fan is starting.");
        self.fan_pwm.enable().map_err(|error| error.to_string())
    }

    fn duty_cycle(&self) -> Result<f64, String> {
       let temperature = self.temperature.temperature();
        match temperature {
            Ok(temperature) => {
                let mut frequency = 0.0;
                if temperature > self.temp_max {
                    frequency = 1.0;
                } else if temperature > self.temp_min {
                    let delta = self.fan_high - self.fan_low;
                    frequency = delta * (temperature - self.temp_min) as f64 / (self.temp_max - self.temp_min) as f64 + self.fan_low;
                }
                debug!("Fan::duty_cycle | frequency = {}, temperature = {}", frequency, temperature);
                Ok(frequency)
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
                self.fan_pwm.set_duty_cycle(duty_cycle).map_err(|err| err.to_string())
            }
            Err(error) => {
                error!("Fan::duty_cycle | error: {}", error);
                Err(error)
            }
        }
    }

    pub fn stop(&self) -> PwmResult<()> {
        match self.fan_pwm.disable() {
            Ok(_) => {
                debug!("Fan::stop | The fan is stoped.");
                Ok(())
            },
            Err(error) => {
                error!("Fan::stop | error: {}", error);
                Err(error)
            }
        }
    }
}
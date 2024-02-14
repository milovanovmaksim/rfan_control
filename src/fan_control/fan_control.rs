use std::{thread, time::Duration};
use log::{debug, error};

use crate::fan::fan::Fan;

pub(crate) struct FanControl {
    fan: Fan,
    delay: u64
}



impl FanControl {
    pub fn new(fan: Fan, delay: u64) -> Self {
        FanControl { fan, delay}
    }

    pub fn run(&mut self) -> Result<(), Vec<String>> {
        /// TODO Обработать Ctrl + C
        let mut errors = vec![];
        match self.fan.start() {
            Ok(_) => { debug!("The fan started."); },
            Err(start_error) => {
                error!("FanControl::run | error: {}", start_error);
                errors.push(start_error);
                return Err(errors);
            }
        }
        loop {
            match self.fan.update_duty_cycle() {
                Ok(_) => {
                    thread::sleep(Duration::from_secs(self.delay));
                },
                Err(update_duty_cycle_error) => {
                    error!("FanControl::run | error: {}", update_duty_cycle_error);
                    match self.fan.stop() {
                        Ok(_) => {  },
                        Err(stop_error) => {
                            error!("FanControl::run | error: {}", stop_error);
                            errors.push(stop_error.to_string());
                        }
                    }
                    errors.push(update_duty_cycle_error);
                    return Err(errors);
                }
            }
        }
    }
}
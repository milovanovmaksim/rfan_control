use std::{thread, time::Duration};
use log::{debug, error};
use simple_signal::Signal;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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
        let mut errors = vec![];
        match self.fan.start() {
            Ok(_) => { debug!("The fan started."); },
            Err(start_error) => {
                error!("FanControl::run | error: {}", start_error);
                errors.push(start_error);
                return Err(errors);
            }
        }
        let running = Arc::new(AtomicBool::new(true));
        simple_signal::set_handler(&[Signal::Int, Signal::Term], {
            let running = running.clone();
            move |_| {
                running.store(false, Ordering::SeqCst);
            }
        });
        while running.load(Ordering::SeqCst) {
            match self.fan.update_duty_cycle() {
                Ok(_) => {
                    thread::sleep(Duration::from_secs(self.delay));
                },
                Err(update_duty_cycle_error) => {
                    running.store(false, Ordering::SeqCst);
                    error!("FanControl::run | error: {}", update_duty_cycle_error);
                    errors.push(update_duty_cycle_error);
                }
            }
        }
        match self.fan.stop() {
            Ok(_) => {
                if errors.len() > 0 {
                    Err(errors)
                } else {
                    debug!("FanControl::run | The fan is stoped.");
                    Ok(())
                }
            },
            Err(stop_error) => {
                error!("FanControl::run | error: {}", stop_error);
                errors.push(stop_error.to_string());
                Err(errors)
            }
        }
    }
}
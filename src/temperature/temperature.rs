use std::{fs::File, io::Read};
use log::error;

pub(crate)  struct Temperature {
    path: String
}


///
/// Температура CPU Raspberry Pi4.
/// Parameters:
///     path - путь к файлу с которогу можно прочитать температуру процессоора.
///     Для Raspberry Pi /sys/class/thermal/thermal_zone0/temp.
impl Temperature {
    pub fn new(path: String) -> Self {
        Temperature { path }

    }

    ///
    /// Читает температуру CPU Raspberry pi4 из файла.
    pub fn temperature(&self) -> Result<u64, String> {
        let temperature: Result<u64, String> = {
            match File::open(self.path.clone()) {
                Ok(mut file) => {
                    let mut content = String::new();
                    match file.read_to_string(&mut content) {
                        Ok(_) => {
                            match content.trim().parse::<f64>() {
                                Ok(temperature) => {
                                    let temperature = (temperature / 1000.0) as u64;
                                    Ok(temperature)
                                },
                                Err(e) => { Err(e.to_string()) }
                            }
                        },
                        Err(e) => { Err(e.to_string())  },
                    }
                },
                Err(e) => { Err(e.to_string())  },
            }
        };
        match temperature {
            Ok(temperature) => Ok(temperature),
            Err(error) => {
                error!("Temperature::temperature | error: {}", error);
                Err(error)
            },
        }
    }
}
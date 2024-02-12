use std::{fs::File, io::Read};
use log::error;

pub(crate)  struct Temperature {
    path: String
}


impl Temperature {
    pub fn new(path: String) -> Self {
        Temperature { path }

    }

    pub fn temperature(&self) -> Result<u32, String> {
        let temperature = {
            match File::open(self.path.clone()) {
                Ok(mut file) => {
                    let mut content = String::new();
                    match file.read_to_string(&mut content) {
                        Ok(_) => {
                            match content.trim().parse::<f32>() {
                                Ok(temperature) => {
                                    let temperature = (temperature / 1000.0) as u32;
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
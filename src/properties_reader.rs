use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;

use java_properties::read;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref STATIC_CONFIG: Mutex<HashMap<String, String>> = HashMap::new().into();
}

pub fn red_config_from_file(file: String) -> HashMap<String, String> {
    let config_file = if !file.is_empty() { file } else { "./application.properties".to_owned() };

    let file = match File::open(&config_file) {
        Err(why) => panic!("Houve um erro abrindo {}: {}", config_file, why),
        Ok(file) => file,
    };

    match read(BufReader::new(file)) {
        Err(why) => panic!(
            "Houve um erro abrindo o arquivo de configuração {}: {}",
            config_file, why
        ),
        Ok(file) => file,
    }
}

pub fn initialize_config(file: String) {
    *STATIC_CONFIG.lock().unwrap() = red_config_from_file(file);
}


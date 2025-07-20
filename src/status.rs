use std::{fs::{create_dir_all, read_to_string, File}, io::Write, path::PathBuf};

use serde::{Serialize,Deserialize};
use dirs::home_dir;


#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    token: String,
    edited: bool
}

impl Status {
    pub fn new() -> Self {
        let config_path = Self::get_config_path();
        
        
        if config_path.exists() {
            return Self::read_from_path(config_path);
        }else {
            return Self::create_from_path(config_path)
        }
    }
    pub fn update_token(&mut self, token: String) {
        self.token = token;
        self.edited = true;
    }

    pub fn get_config_path() -> PathBuf {
        let home_path = match home_dir() {
            Some(path ) => path,
            None => panic!("1: there should be a home dir")
        };
        home_path.join(".config/sylrs/config.json")
    }
    pub fn read_from_path(config_path : PathBuf) -> Status {
        let content = match read_to_string(config_path) {
                Ok(c) => c,
                Err(e) => panic!("an error occurred while oping the file {e:?}")
        };
        println!("{:?}",content);
        let s = match serde_json::from_str::<Status>(content.as_str()) {
            Ok(s) => s,
            Err(e) => panic!("1: serde Deserialize error {e:?}")
        };
        s
    }
    fn write_status_to_desk(config_path : PathBuf, s : &Status) {
        let ser = match serde_json::to_string(s) {
            Ok(s) => s,
            Err(e) => panic!("1: serde serialize error {e:?}")
        };

        if let Some(parent) = config_path.parent() {
            create_dir_all(parent).expect("1 : Failed to create config directory")
        }
        let mut f = match File::create(config_path) {
            Ok(f) => f,
            Err(e) => panic!("1: fs open error {e:?}")
        };
        match f.write_all(ser.as_bytes()) {
            Ok(_) => (),
            Err(e) => panic!("1: fs write error {e:?}")
        };
    }
    fn create_from_path(config_path : PathBuf) -> Status {
        let s = Status{token: String::new(), edited:false};
        Self::write_status_to_desk(config_path, &s);
        s
    }
    pub fn write_update(&mut self) {
        if self.edited {
            Self::write_status_to_desk(Self::get_config_path(), &self);
        }
    }
}

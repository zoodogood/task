use dirs::home_dir;
use std::fs;
use std::io::Error;
use std::path::{Display, PathBuf};

const HASH: &str = "37f125ba7c";
const DIR_NAME: &str = ".task";

const TASK_FILE: &str = "task.toml";

pub fn create_folder() -> Result<(), std::io::Error> {
    fs::DirBuilder::new().create(directory_path())
}

pub fn directory_path() -> PathBuf {
    let home = home_dir().unwrap().display().to_string();
    PathBuf::from(format!("{home}/{DIR_NAME}-{HASH}"))
}

pub const CONFIG_FILE: &str = "config.toml";
pub struct Config;


impl Config {
    pub fn read() -> Result<Option<Vec<u8>>, Error> {
        let directory = directory_path().display().to_string();
        match fs::read(PathBuf::from(format!("{directory}/{CONFIG_FILE}"))) {
				Ok(value) => Ok(Some(value)),
				Err(error) => match error {
					 
				}
		  } 
    }

    pub fn write() {}
}

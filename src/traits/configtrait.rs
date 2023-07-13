use dirs::home_dir;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    error::Error,
    fs::{self, OpenOptions},
    io::Write,
};

pub const EXTENSION: &str = "yml";

/// Trait for loading, saving config files
pub trait ConfigTrait {
    const LABEL: &'static str;

    fn load() -> Result<Box<Self>, Box<dyn Error>>
    where
        Self: Serialize + DeserializeOwned + Default + Clone,
    {
        let config_path =
            home_dir()
                .unwrap()
                .join(format!(".config/gm/{}.{}", Self::LABEL, EXTENSION));

        // The config struct
        let config: Self = {
            || -> Result<Self, Box<dyn Error>> {
                let content = match fs::read_to_string(&config_path) {
                    Ok(content) => content,
                    // If the config file does not exist returns Self::defult()
                    Err(_) => return Ok(Self::default()),
                };

                match serde_yaml::from_str::<Self>(&content) {
                    Ok(config) => Ok(config),
                    // If there is error parsing the json file, create a backup with current time and returns
                    // Self::default()
                    Err(_) => {
                        let mut new_path = config_path.clone();
                        new_path.pop();
                        new_path.push(format!(
                            "{}-{}.{}",
                            Self::LABEL,
                            chrono::offset::Local::now(),
                            EXTENSION
                        ));
                        fs::rename(&config_path, &new_path)?;
                        Ok(Self::default())
                    }
                }
            }
        }()?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&config_path)?;
        file.write_all(serde_yaml::to_string(&config)?.as_bytes())?;

        Ok(Box::new(config))
    }
}

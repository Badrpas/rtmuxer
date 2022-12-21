use std::{collections::HashMap, path::Path, fmt::Display};

use path_absolutize::Absolutize;
use serde::Deserialize;

use crate::tmux::{Session, Window};




#[derive(Deserialize)]
pub struct Config(HashMap<String, SessionConfig>);

pub struct ConfigEntry {
    pub config: Config,
    pub config_path: String,
}

pub enum Error {
    Open(std::io::Error),
    Parse(serde_yaml::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Open(err) => write!(f, "Error while opening file: {err}"),
            Error::Parse(err) => write!(f, "Error while parsing file: {err}"),
        }
    }
}

impl ConfigEntry {
    pub fn load<P:Into<String>>(config_path: P) -> Result<Self, Error> {
        let config_path = config_path.into();
        let config_path = Path::new(&config_path).absolutize().unwrap().to_str().unwrap().to_string();
        println!("configpath: {config_path}");
        match std::fs::File::open(&config_path) {
            Err(err) => {
                Err(Error::Open(err))
            }
            Ok(f) => match serde_yaml::from_reader::<std::fs::File, Config>(f) {
                Err(err) => {
                    Err(Error::Parse(err))
                }
                Ok(config) => {
                    Ok(Self {
                        config,
                        config_path,
                    })
                }
            }
        }
    }
}

#[derive(Deserialize)]
pub struct SessionConfig(HashMap<String, WindowConfig>);

#[derive(Deserialize)]
pub struct WindowConfig {
    cmd: Option<String>,
    keys: Option<String>,
    cwd: Option<String>,
    env: Option<HashMap<String, String>>,
}


impl From<ConfigEntry> for Vec<Session> {
    fn from(entry: ConfigEntry) -> Self {
        let mut sessions = vec![];
        let config = entry.config;
        for (name, session_conf) in config.0 {
            let mut session = Session::new(name);
            for (name, w_conf) in session_conf.0 {
                let name = name.clone();
                let cwd = w_conf.cwd.map(|v| {
                    Path::new(&shellexpand::tilde(&v).to_string())
                        .absolutize_from(Path::new(&entry.config_path).parent().unwrap())
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string()
                });
                session.windows.push(Window {
                    name,
                    cwd,
                    cmd: w_conf.cmd,
                    keys: w_conf.keys,
                    env: w_conf.env,
                });
            }
            sessions.push(session);
        }
        sessions
    }
}




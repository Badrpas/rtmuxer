use std::{env, fs::File};

mod tmux;
use tmux::*;

mod config;
use config::Config;

mod helpers;
mod errors;


fn main() {
    let mut args = env::args();
    args.next();
    let config_path = args.next();
    if config_path.is_none() {
        println!("Provide a config file path.");
        return;
    }

    match std::fs::File::open(config_path.unwrap()) {
        Err(err) => {
            println!("Error while reading config: {err}");
        }
        Ok(f) => match serde_yaml::from_reader::<File, Config>(f) {
            Err(err) => {
                println!("Error while parsing config: {err}");
            }
            Ok(conf) => {
                let conf: Vec<Session> = conf.into();
                for session in &conf {
                    if let Err(err) = session.sync() {
                        println!("{err}");
                    }
                }
            }
        }
    };
}


use std::{env, fs::File};

mod tmux;
use tmux::*;

mod config;
use config::Config;

mod helpers;
mod errors;


fn main() {
    let mut args = env::args();

    if args.len() == 1 {
        const DEFAULT_FILE: &str = "rtmuxer.yaml";
        if std::path::Path::new(DEFAULT_FILE).exists() {
            run(DEFAULT_FILE);
        } else {
            println!("Provide a config file");
            return;
        }
    }

    args.next(); // Skip self

    for config_path in args {
        run(&config_path);
    }
}

fn run(config_path: &str) {
    match std::fs::File::open(config_path) {
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


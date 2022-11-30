use std::{env, fs::File};

mod tmux;
use tmux::*;

mod config;
use config::Config;

mod helpers;
mod errors;


fn main() {
    let mut args = env::args();
    args.next(); // Skip self

    for config_path in args {
        run(config_path);
    }
}

fn run(config_path: String) {
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


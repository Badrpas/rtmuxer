use std::env;

mod tmux;
use tmux::*;

mod config;
use config::ConfigEntry;

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
    match ConfigEntry::load(config_path) {
        Err(err) => println!("Error while loading config at {config_path}: {err}"),
        Ok(conf_entry) => {
            let conf: Vec<Session> = conf_entry.into();
            for session in &conf {
                if let Err(err) = session.sync() {
                    println!("{err}");
                }
            }
        }
    };
}


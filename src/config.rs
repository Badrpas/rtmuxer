use std::collections::HashMap;

use serde::Deserialize;

use crate::tmux::{Session, Window};




#[derive(Deserialize)]
pub struct Config(HashMap<String, SessionConfig>);


#[derive(Deserialize)]
pub struct SessionConfig(HashMap<String, WindowConfig>);

#[derive(Deserialize)]
pub struct WindowConfig {
    cmd: Option<String>,
    keys: Option<String>,
    cwd: Option<String>,
    env: Option<HashMap<String, String>>,
}


impl From<Config> for Vec<Session> {
    fn from(config: Config) -> Self {
        let mut sessions = vec![];
        for (name, session_conf) in config.0 {
            let mut session = Session::new(name);
            for (name, w_conf) in session_conf.0 {
                session.windows.push(Window {
                    name,
                    cmd: w_conf.cmd,
                    keys: w_conf.keys,
                    cwd: w_conf.cwd.map(|v|shellexpand::tilde(&v).into()),
                    env: w_conf.env,
                });
            }
            sessions.push(session);
        }
        sessions
    }
}




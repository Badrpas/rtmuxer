use std::{process::Command, collections::HashMap};
use fehler::*;

use serde::Deserialize;

use crate::helpers::StrFromVec;

#[derive(Deserialize, Default)]
pub struct Session {
    pub name: String,
    pub windows: Vec<Window>,
}

#[derive(Deserialize, Default)]
pub struct Window {
    pub name: String,
    pub cmd: Option<String>,
    pub keys: Option<String>,
    pub cwd: Option<String>,
    pub env: Option<HashMap<String, String>>,
}


impl Session {

    pub fn new<S:Into<String>>(name: S) -> Self {
        Session{
            name: name.into(),
            ..Default::default()
        }
    }

    #[throws(crate::errors::Error)]
    pub fn sync(&self) {
        let cmd = Command::new("tmux")
            .arg("list-sessions")
            .args(["-F", "#{session_name}"])
            .output()?;
        let sessions: Vec<String> = cmd.stdout.into_str().split('\n').map(|s|s.to_string()).collect();

        let is_first = if !sessions.iter().any(|s|*s == self.name) {
            println!("Creating session [{}]", self.name);
            Command::new("tmux")
                .arg("new-session")
                .args(["-s", &self.name])
                .arg("-d")
                .status()?;
            true
        } else {
            println!("Session [{}] exists", self.name);
            false
        };

        let cmd = Command::new("tmux")
            .arg("list-windows")
            .args(["-t", &self.name])
            .args(["-F", "#{window_name}"])
            .output()?; 
        let windows: Vec<String> = cmd.stdout.into_str().split('\n').map(|s|s.to_string()).collect();

        for window in &self.windows {
            window.sync(&self.name, &windows)?;
        }

        if is_first {
            for window_name in &windows {
                if window_name.is_empty() { continue; }
                if !self.windows.iter().any(|w|&w.name==window_name) {
                    println!("\tDeleting window [{window_name}] as its not needed");
                    Command::new("tmux")
                        .arg("kill-window")
                        .args(["-t", window_name])
                        .status()?; 
                }
            }
        }
    }
}



impl Window {
    #[throws(crate::errors::Error)]
    pub fn sync(&self, parent_name: &str, windows: &[String]) {
        if !windows.iter().any(|s|*s == self.name) {
            println!("\tCreating window [{}]", self.name);

            let mut command = Command::new("tmux");

            command
                .arg("new-window")
                .args(["-t", parent_name])
                .args(["-n", &self.name])
                .arg("-d");

            if let Some(cwd) = &self.cwd {
                println!("\t\tSetting CWD to \"{cwd}\"");
                command.args(["-c", cwd.as_str()]);
            }

            if let Some(envs) = &self.env {
                for (name, val) in envs {
                    println!("\t\tSetting ENV var {name}");
                    command.args(["-e", format!("{name}={val}").as_str()]);
                }
            }

            command.status()?; 

            if let Some(cmd) = &self.cmd {
                Command::new("tmux")
                    .arg("send-keys")
                    .arg("-l")
                    .args(["-t", &self.name])
                    .arg(cmd)
                    .status()?;

                Command::new("tmux")
                    .arg("send-keys")
                    .args(["-t", &self.name])
                    .arg("ENTER")
                    .status()?;
            }

            if let Some(keys) = &self.keys {
                Command::new("tmux")
                    .arg("send-keys")
                    .args(["-t", &self.name])
                    .arg(keys)
                    .status()?;
            }

        } else {
            println!("\t\tWindow [{}] exists", self.name);
        }
    }
}

fn _cmd_to_string(command: &Command) -> String {
    format!(
        "{} {}", 
       command.get_program().to_string_lossy(),
       command.get_args()
         .into_iter()
         .map(|a|a.to_str().unwrap())
         .collect::<Vec<&str>>()
         .join(" ")
     )
}



mod os {
    use eyre::{eyre, Result};

    use std::path::PathBuf;
    use std::ffi::OsString;
    use std::{env, fs};

    pub fn get_cwd() -> Result<OsString> {
        let cwd = env::current_dir()?;
        let home = PathBuf::from(env::var("HOME")?);
        if cwd == home {
            Ok(OsString::from("~"))
        } else {
            Ok(OsString::from(cwd.components().last().ok_or(eyre!("No last element"))?.as_os_str()))
        }
    }

    pub fn get_hostname() -> Result<String> {
        Ok(fs::read_to_string("/etc/hostname")?.strip_suffix("\n").ok_or(eyre!("None error"))?.to_string())
    }
}

use std::env;
use std::ffi::OsString;
use ansi_term::Colour;

mod git;

struct Properties {
    hostname: String,
    user: String,
    cwd: OsString,
    git: Option<git::GitInfo>,
    venv: String,
}

impl Properties {
    fn new() -> Properties {
        let hostname = os::get_hostname().unwrap_or(String::from("???"));
        let user = env::var("USER").unwrap_or(String::from("???"));
        let cwd = os::get_cwd().unwrap_or(OsString::from("???"));
        let git = git::GitInfo::from(".").ok();

        let venv = match env::var("VIRTUAL_ENV") {
            Ok(path) => path.split("/").last().unwrap_or_default().into(),
            Err(_) => String::new(),
        };
        Properties{hostname, user, cwd, git, venv}
    }
    
}


fn main() {
    let prop = Properties::new();
    prompt(prop);
}

fn prompt(ps: Properties) { 
    let user = Colour::Cyan.bold().paint(ps.user);
    let hostname = Colour::Cyan.bold().paint(ps.hostname);
    let directory = Colour::Blue.bold().paint(ps.cwd.to_string_lossy());
    let venv = ps.venv;
    let git = ps.git.unwrap_or_default(); // the default shows nothing
    if venv.len() > 0 {
        let s = format!(" {} ", venv);
        print!("{}", Colour::Red.paint(s));
    }
    println!(
        "{user} at {hostname} in {cwd} {git}", 
        user=user,
        hostname=hostname, 
        cwd=directory,
        git=git,
    );

    // Make sure to not use any styling here, as it will mess up the 
    // autocomplete of Zsh
    // Options for the shell prompt:
    // λ -> > $ :: ⟩ ⟫ ❱
    print!(" {shell} ", shell="❱");
}

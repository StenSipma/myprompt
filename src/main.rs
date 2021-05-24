mod os {
    use std::fmt;

    #[derive(Debug)]
    pub struct MyError {
        details: String
    }

    impl MyError {
        fn new(msg :String) -> MyError {
            MyError{details: msg}
        }
    }

    impl fmt::Display for MyError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"{}", self.details)
        }
    }

    impl Error for MyError {
        fn description(&self) -> &str {
            &self.details
        }
    }

    impl From<io::Error> for MyError {
        fn from(err: io::Error) -> MyError {
            MyError::new(err.to_string())
        }
    }

    impl From<env::VarError> for MyError {
        fn from(err: env::VarError) -> MyError {
            MyError::new(err.to_string())
        }
    }

    impl From<&str> for MyError {
        fn from(msg: &str) -> MyError {
            MyError::new(msg.to_string())
        }
    }

    use std::path::PathBuf;
    use std::ffi::OsString;
    use std::{env, io, fs};
    use std::error::Error;

    pub fn get_cwd() -> Result<OsString, MyError> {
        let cwd = env::current_dir()?;
        let home = PathBuf::from(env::var("HOME")?);
        if cwd == home {
            Ok(OsString::from("~"))
        } else {
            Ok(OsString::from(cwd.components().last().ok_or(MyError::from("No last element"))?.as_os_str()))
        }
    }

    pub fn get_hostname() -> Result<String, MyError> {
        Ok(fs::read_to_string("/etc/hostname")?.strip_suffix("\n").ok_or(MyError::from("None error"))?.to_string())
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
    git: git::GitInfo,
}

impl Properties {
    fn new() -> Properties {
        let hostname = os::get_hostname().unwrap_or(String::from("???"));
        let user = env::var("USER").unwrap_or(String::from("???"));
        let cwd = os::get_cwd().unwrap_or(OsString::from("???"));
        let git = git::GitInfo::new();
        Properties{hostname, user, cwd, git}
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
    // Options for the shell prompt:
    // λ -> > $ :: ⟩ ⟫ ❱
    let shell = Colour::White.bold().paint("❱");
    print!(
        "{user} at {hostname} in {cwd} {git}\n {shell} ", 
        user=user,
        hostname=hostname, 
        cwd=directory,
        git=ps.git,
        shell=shell,
    );
}

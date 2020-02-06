use std::{
    env,
    ffi::{OsStr, OsString},
    path::Path,
};

#[macro_export]
macro_rules! program_main {
    ($name:expr) => {
        fn main() -> ! {
            std::process::exit(match program() {
                Ok(code) => code,
                Err(error) => {
                    eprintln!("{}: {}", $crate::util::program_name($name), error);
                    1
                }
            });
        }
    };
}

pub fn program_args() -> Vec<String> {
    env::args_os()
        .map(|a| a.to_string_lossy().into_owned())
        .collect()
}

pub fn program_args_os() -> Vec<OsString> {
    env::args_os().collect()
}

pub fn program_name(default: &str) -> String {
    match env::args_os().next() {
        None => String::from(default),
        Some(os_string) => match Path::new(&os_string).file_name() {
            None => String::from(default),
            Some(os_str) => os_str.to_string_lossy().into_owned(),
        },
    }
}

pub fn program_name_os(default: &OsStr) -> OsString {
    match env::args_os().next() {
        None => OsString::from(default),
        Some(os_string) => match Path::new(&os_string).file_name() {
            None => OsString::from(default),
            Some(os_str) => os_str.to_os_string(),
        },
    }
}

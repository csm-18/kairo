use crate::diagnostics::Diagnostic;
use crate::diagnostics::create_diagnostic;
use std::env;
use std::fs;

// return command-line arguments provided by the user
pub fn get_cli_args() -> Vec<String> {
    std::env::args().skip(1).collect()
}

pub fn kairo_diagnostic_comments_env_variable() -> bool {
    match env::var("KAIRO_DIAGNOSTIC_COMMENTS") {
        Ok(_) => true,
        Err(_) => false,
    }
}

/* io */

// println!() wrapper macro
macro_rules! writeln {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}
pub(crate) use writeln;

/* file io */
pub fn read_file(path: &str) -> Result<Vec<u8>, Option<Diagnostic>> {
    let bytes: Vec<u8>;
    match fs::read(path) {
        Ok(bts) => {
            bytes = bts;
        }
        Err(_) => {
            // cannot read file error
            return Err(create_diagnostic(5083, &[path]));
        }
    };

    // check if the content of the file is valid utf-8 text
    match std::str::from_utf8(&bytes) {
        Ok(_) => {}
        Err(_) => {
            // cannot read file error
            return Err(create_diagnostic(5083, &[path]));
        }
    };

    Ok(bytes)
}

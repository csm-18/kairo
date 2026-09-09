use std::env;

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

/* native io abstraction */

// println!() wrapper macro
macro_rules! writeln {
    ($($arg:tt)*) => {
        println!($($arg)*)
    };
}
pub(crate) use writeln;

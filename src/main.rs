mod cli;
mod env;

// stand-alone cli compiler entry-point
fn main() {
    // command-line arguments
    let mut args: Vec<String> = env::get_cli_args();

    // start the compiler cli with command-line arguments
    cli::run(args);
}

mod cli;
mod diagnostics;
mod host;

// stand-alone cli compiler entry-point
fn main() {
    // command-line arguments
    let mut args: Vec<String> = host::get_cli_args();

    // start the compiler cli with command-line arguments
    cli::run(&mut args);
}

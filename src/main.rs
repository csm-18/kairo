mod cli;

// stand-alone cli compiler entry-point
fn main() {
    // command-line arguments
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    // start the compiler cli with command-line arguments
    cli::run(args);
}

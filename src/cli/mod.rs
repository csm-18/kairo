// parse and execute command-line arguments passed to the compiler
pub fn run(args: Vec<String>) {
    if !args.is_empty() && (args[0] == "--build" || args[0] == "-b") {
        // build mode
        println!("Build mode is not implemented yet!");
    } else {
        // normal mode
        dbg!(args);
    }
}

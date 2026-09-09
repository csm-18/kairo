mod arg_parser;
mod cli_flag_parser;
mod response_file_parser;
use arg_parser::parse_args;

// parse and execute command-line arguments passed to the compiler
pub fn run(args: &mut Vec<String>) {
    if !args.is_empty()
        && (args[0] == "--build" || args[0] == "-build" || args[0] == "-b" || args[0] == "--b")
    {
        // build mode
        println!("Build mode is not implemented yet!");
    } else {
        // normal mode
        let (errors, filenames) = parse_args(args);

        for file in filenames {
            dbg!(file);
        }
        for error in errors {
            match error {
                Some(err) => {
                    err.print();
                }
                None => {}
            }
        }
    }
}

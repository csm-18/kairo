use crate::{
    cli::{cli_flag_parser::parse_cli__flags, response_file_parser::expand_response_files},
    diagnostics::Diagnostic,
};

// parse command-line arguments passed to the compiler in normal mode
pub fn parse_args(args: &mut Vec<String>) -> (Vec<Option<Diagnostic>>, Vec<String>) {
    let mut errors: Vec<Option<Diagnostic>> = vec![];
    let mut filenames: Vec<String> = vec![];

    // expanding all response files and report any errors
    let response_file_errors = expand_response_files(args);
    // if circular reference error exists then return
    let circular_reference_error: bool = response_file_errors
        .iter()
        .flatten()
        .any(|error| error.code == 100000);
    errors.extend(response_file_errors);
    if circular_reference_error {
        return (errors, filenames);
    }

    // parse all command-line flags
    let flag_parsing_errors = parse_cli__flags(args);
    errors.extend(flag_parsing_errors);

    // get filenames
    filenames.extend(args.clone());

    (errors, filenames)
}

use crate::{cli::response_file_parser::expand_response_files, diagnostics::Diagnostic};

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

    (errors, filenames)
}

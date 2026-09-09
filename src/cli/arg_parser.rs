use crate::{cli::response_file_parser::expand_response_files, diagnostics::create_diagnostic};

pub fn parse_args(args: &mut Vec<String>) {
    // expand all response files
    let response_file_errors = expand_response_files(args);
    dbg!(response_file_errors);
}

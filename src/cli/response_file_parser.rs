use crate::diagnostics::{Diagnostic, create_diagnostic};
use crate::host;

pub fn expand_response_files(args: &mut Vec<String>) -> Vec<Option<Diagnostic>> {
    let mut errors: Vec<Option<Diagnostic>> = vec![];
    let mut response_files_count = 0;
    let mut x = 0;
    while x < args.len() {
        if args[x].starts_with('@') {
            response_files_count += 1;
            let filename = &args[x][1..];
            if response_files_count > 200 {
                // response file circular reference error
                let error = create_diagnostic(100000, &[filename]);
                errors.push(error);
                args.remove(x);
                return errors;
            }
            match parse_response_file(filename) {
                Ok(temp_args) => {
                    args.splice(x..x + 1, temp_args);
                    continue;
                }
                Err(parse_error) => {
                    errors.push(parse_error);
                    args.remove(x);
                    continue;
                }
            }
        }
        x += 1;
    }

    errors
}

fn parse_response_file(filename: &str) -> Result<Vec<String>, Option<Diagnostic>> {
    let mut args: Vec<String> = vec![];

    // read file
    let content = match host::read_file(filename) {
        Ok(text) => text,
        Err(read_error) => {
            return Err(read_error);
        }
    };

    let mut x = 0;
    while x < content.len() {
        if content[x] <= b' ' {
            // skip ascii whitespace and control characters
        } else if content[x] == b'"' {
            // parse quoted string
            let mut end_quote = false;
            let mut y = x + 1;
            while y < content.len() {
                if content[y] == b'"' {
                    end_quote = true;
                    break;
                }
                y += 1;
            }
            if end_quote {
                let arg = std::str::from_utf8(&content[x + 1..y]).unwrap().to_string();
                args.push(arg);
                x = y + 1;
                continue;
            } else {
                // unterminated quoted string error
                let error = create_diagnostic(6045, &[filename]);
                return Err(error);
            }
        } else {
            // parse unquoted string
            let mut y = x;
            while y < content.len() && content[y] > b' ' {
                y += 1;
            }
            let arg = std::str::from_utf8(&content[x..y]).unwrap().to_string();
            args.push(arg);
            x = y;
            continue;
        }
        x += 1;
    }

    Ok(args)
}

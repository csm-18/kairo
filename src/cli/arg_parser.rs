use crate::diagnostics::create_diagnostic;

pub fn parse_args(args: &mut Vec<String>) {
    match create_diagnostic(5083, &[&"hello.txt"]) {
        Some(error) => {
            error.print();
        }
        None => {}
    }
}

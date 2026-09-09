use crate::diagnostics::Diagnostic;

pub fn parse_cli__flags(args: &mut Vec<String>) -> Vec<Option<Diagnostic>> {
    let mut errors: Vec<Option<Diagnostic>> = vec![];
    let mut x = 0;
    while x < args.len() {
        if args[x].starts_with('-') {}
        x += 1;
    }
    errors
}

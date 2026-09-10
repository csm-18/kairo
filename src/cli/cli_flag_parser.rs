use crate::diagnostics::Diagnostic;

pub fn parse_cli__flags(args: &mut Vec<String>) -> Vec<Option<Diagnostic>> {
    let mut errors: Vec<Option<Diagnostic>> = vec![];
    let mut x = 0;
    while x < args.len() {
        if args[x].starts_with('-') {
            let user_input = &args[x];
            // remove '--' or '-' prefix
            let flag_name: String = if args[x].starts_with("--") {
                args[x][2..].to_string()
            } else {
                args[x][1..].to_string()
            };
        }
        x += 1;
    }
    errors
}

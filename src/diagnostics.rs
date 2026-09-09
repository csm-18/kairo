use std::sync::LazyLock;

pub struct Diagnostic {
    pub code: isize,
    pub message: String,
    pub category: String,
}

impl Diagnostic {
    pub fn print(&self) {
        println!("{} TS{}: {}", self.category, self.code, self.message);
    }
}

pub static DIAGNOSTICS: LazyLock<Vec<Diagnostic>> = LazyLock::new(|| {
    vec![
        Diagnostic {
            code: 5083,
            message: "Cannot read file '{}'.".to_string(),
            category: "error".to_string(),
        },
        Diagnostic {
            code: 6045,
            message: "Unterminated quoted string in response file '{}'.".to_string(),
            category: "error".to_string(),
        },
        Diagnostic {
            code: 100000,
            message: "Too many response files provided. Circular reference suspected in file '{}'."
                .to_string(),
            category: "error".to_string(),
        },
    ]
});

pub fn create_diagnostic(code: isize, args: &[&str]) -> Option<Diagnostic> {
    for d in DIAGNOSTICS.iter() {
        if d.code == code {
            let mut msg = d.message.clone();
            for arg in args {
                if let Some(pos) = msg.find("{}") {
                    msg.replace_range(pos..pos + 2, arg);
                }
            }
            return Some(Diagnostic {
                code: d.code,
                message: msg,
                category: d.category.clone(),
            });
        }
    }
    None
}

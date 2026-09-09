use std::sync::LazyLock;

use crate::host;

static KAIRO_DIAGNOSTIC_COMMENTS: LazyLock<bool> =
    LazyLock::new(|| host::kairo_diagnostic_comments_env_variable());

pub struct Diagnostic {
    pub code: isize,
    pub message: String,
    pub category: String,
    pub funny_comment: String,
}

impl Diagnostic {
    pub fn print(&self) {
        println!("{} TS{}: {}", self.category, self.code, self.message);
        if *KAIRO_DIAGNOSTIC_COMMENTS {
            println!(" {}", self.funny_comment);
        }
    }
}

pub static DIAGNOSTICS: LazyLock<Vec<Diagnostic>> = LazyLock::new(|| {
    vec![
        Diagnostic {
            code: 5083,
            message: "Cannot read file '{}'.".to_string(),
            category: "error".to_string(),
            funny_comment: "Oye, file kitthe hai? 😂".to_string(),
        },
        Diagnostic {
            code: 6045,
            message: "Unterminated quoted string in response file '{}'.".to_string(),
            category: "error".to_string(),
            funny_comment:
                "String close karna nahi aata… aur TypeScript developer ban gaya? Gajab! 😂"
                    .to_string(),
        },
        Diagnostic {
            code: 100000,
            message: "Too many response files provided. Circular reference suspected in file '{}'."
                .to_string(),
            category: "error".to_string(),
            funny_comment: "Bhai, teri file khulte-khulte toh main buddha ho jaunga. 😂"
                .to_string(),
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
                funny_comment: d.funny_comment.clone(),
            });
        }
    }
    None
}

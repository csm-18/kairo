use std::{collections::HashMap, sync::LazyLock};

pub struct CommandLineFlag {
    pub name: String,
    pub short_name: String,
    pub category: String,
    pub value_type: String,
    pub accepted_values: Vec<String>,
    pub default_value: String,
    pub meta_info: HashMap<String, String>,
}

pub static CLI_FLAGS: LazyLock<Vec<CommandLineFlag>> = LazyLock::new(|| {
    vec![CommandLineFlag {
        name: "locale".to_string(),
        short_name: "".to_string(),
        category: "cli".to_string(),
        value_type: "string".to_string(),
        accepted_values: vec![
            "pt-BR".to_string(), // Brazilian Portuguese
            "pt".to_string(),
            "zh-CN".to_string(), // Chinese (Simplified)
            "zh-TW".to_string(), // Chinese (Traditional)
            "zh".to_string(),
            "cs-CZ".to_string(), // Czech
            "cs".to_string(),
            "en-US".to_string(), // English (US)
            "en".to_string(),
            "fr-FR".to_string(), // French
            "fr".to_string(),
            "de-DE".to_string(), // German
            "de".to_string(),
            "it-IT".to_string(), // Italian
            "it".to_string(),
            "ja-JP".to_string(), // Japanese
            "ja".to_string(),
            "ko-KR".to_string(), // Korean
            "ko".to_string(),
            "pl-PL".to_string(), // Polish
            "pl".to_string(),
            "ru-RU".to_string(), // Russian
            "ru".to_string(),
            "es-ES".to_string(), // Spanish
            "es".to_string(),
            "tr-TR".to_string(), // Turkish
            "tr".to_string(),
        ],
        default_value: "en-US".to_string(),
        meta_info: HashMap::from([
            ("is_cli_only_flag".to_string(), "true".to_string()),
            (
                "description".to_string(),
                "Set the language of the messaging from TypeScript. This does not affect emit."
                    .to_string(),
            ),
        ]),
    }]
});

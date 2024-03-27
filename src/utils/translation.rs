use std::collections::HashMap;

pub fn languages() -> Vec<&'static str> {
    vec!["en", "de"]
}

pub fn translations() -> HashMap<String, serde_json::Value> {
    let mut translations = HashMap::new();

    let en = serde_json::from_str(include_str!("../assets/i18n/en.json")).unwrap();
    let de = serde_json::from_str(include_str!("../assets/i18n/de.json")).unwrap();

    translations.insert("en".to_string(), en);
    translations.insert("de".to_string(), de);
    translations
}

pub fn flag(lang: &'static str) -> &'static str {
    match lang {
        "en" => "🇺🇸",
        "de" => "🇩🇪",
        _ => "🌐",
    }
}

pub const LANGUAGE_KEY: &str = "selected_language";

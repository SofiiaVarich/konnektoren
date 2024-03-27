use std::collections::HashMap;

pub fn languages() -> Vec<&'static str> {
    vec!["ua", "en", "de"]
}

pub fn translations() -> HashMap<String, serde_json::Value> {
    let mut translations = HashMap::new();

    let en = serde_json::from_str(include_str!("../assets/i18n/en.json")).unwrap();
    let de = serde_json::from_str(include_str!("../assets/i18n/de.json")).unwrap();
    let ua = serde_json::from_str(include_str!("../assets/i18n/ua.json")).unwrap();

    translations.insert("en".to_string(), en);
    translations.insert("de".to_string(), de);
    translations.insert("ua".to_string(), ua);
    translations
}

pub fn flag(lang: &'static str) -> &'static str {
    match lang {
        "en" => "🇺🇸",
        "de" => "🇩🇪",
        "ua" => "🇺🇦",
        _ => "🌐",
    }
}

pub const LANGUAGE_KEY: &str = "selected_language";

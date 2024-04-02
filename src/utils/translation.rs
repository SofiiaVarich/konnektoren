use std::collections::HashMap;

pub fn languages() -> Vec<&'static str> {
    vec!["en", "ua", "ar", "de", "cn"]
}

pub fn translations() -> HashMap<String, serde_json::Value> {
    let mut translations = HashMap::new();

    let en = serde_json::from_str(include_str!("../assets/i18n/en.json")).unwrap();
    let de = serde_json::from_str(include_str!("../assets/i18n/de.json")).unwrap();
    let ua = serde_json::from_str(include_str!("../assets/i18n/ua.json")).unwrap();
    let cn = serde_json::from_str(include_str!("../assets/i18n/cn.json")).unwrap();
    let ar = serde_json::from_str(include_str!("../assets/i18n/ar.json")).unwrap();

    translations.insert("en".to_string(), en);
    translations.insert("de".to_string(), de);
    translations.insert("ua".to_string(), ua);
    translations.insert("cn".to_string(), cn);
    translations.insert("ar".to_string(), ar);
    translations
}

pub fn flag(lang: &'static str) -> &'static str {
    match lang {
        "en" => "🇺🇸",
        "de" => "🇩🇪",
        "ua" => "🇺🇦",
        "cn" => "🇨🇳",
        "ar" => "🇸🇦",
        _ => "🌐",
    }
}

pub const LANGUAGE_KEY: &str = "selected_language";

use std::collections::HashMap;

pub fn languages() -> Vec<&'static str> {
    vec!["en", "ua", "ar", "de", "cn", "pl"]
}

pub fn translations() -> HashMap<String, serde_json::Value> {
    let mut translations = HashMap::new();

    let en = serde_json::from_str(include_str!("../assets/i18n/en.json")).unwrap();
    let de = serde_json::from_str(include_str!("../assets/i18n/de.json")).unwrap();
    let ua = serde_json::from_str(include_str!("../assets/i18n/ua.json")).unwrap();
    let cn = serde_json::from_str(include_str!("../assets/i18n/cn.json")).unwrap();
    let ar = serde_json::from_str(include_str!("../assets/i18n/ar.json")).unwrap();
    let pl = serde_json::from_str(include_str!("../assets/i18n/pl.json")).unwrap();

    translations.insert("en".to_string(), en);
    translations.insert("de".to_string(), de);
    translations.insert("ua".to_string(), ua);
    translations.insert("cn".to_string(), cn);
    translations.insert("ar".to_string(), ar);
    translations.insert("pl".to_string(), pl);
    translations
}

pub fn supported_language(lang: Option<&str>) -> Option<String> {
    match lang {
        Some(lang) => {
            if languages().contains(&lang) {
                Some(lang.to_string())
            } else {
                None
            }
        }
        None => None,
    }
}

pub fn flag(lang: &'static str) -> &'static str {
    match lang {
        "en" => "🇺🇸",
        "de" => "🇩🇪",
        "ua" => "🇺🇦",
        "cn" => "🇨🇳",
        "ar" => "🇸🇦",
        "pl" => "🇵🇱",
        _ => "🌐",
    }
}

pub const LANGUAGE_KEY: &str = "selected_language";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_language() {
        assert_eq!(supported_language(Some("en")), Some("en".to_string()));
        assert_eq!(supported_language(Some("ua")), Some("ua".to_string()));
        assert_eq!(supported_language(Some("de")), Some("de".to_string()));
        assert_eq!(supported_language(Some("cn")), Some("cn".to_string()));
        assert_eq!(supported_language(Some("ar")), Some("ar".to_string()));
        assert_eq!(supported_language(Some("pl")), Some("pl".to_string()));
        assert_eq!(supported_language(Some("es")), None);
        assert_eq!(supported_language(None), None);
    }

    #[test]
    fn test_flag() {
        assert_eq!(flag("en"), "🇺🇸");
        assert_eq!(flag("de"), "🇩🇪");
        assert_eq!(flag("ua"), "🇺🇦");
        assert_eq!(flag("cn"), "🇨🇳");
        assert_eq!(flag("ar"), "🇸🇦");
        assert_eq!(flag("pl"), "🇵🇱");
        assert_eq!(flag("es"), "🌐");
    }
}

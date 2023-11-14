use lingua::{Language, LanguageDetectorBuilder};

pub fn lang(text: &str) -> String {
    let languages = vec![Language::Chinese, Language::Japanese, Language::English];
    let detector = LanguageDetectorBuilder::from_languages(&languages)
        .with_low_accuracy_mode()
        .build();
    let kind = if let Some(lang) = detector.detect_language_of(text) {
        match lang {
            Language::Chinese => "zh_cn",
            Language::Japanese => "ja",
            Language::English => "en",
        }
    } else {
        "en"
    };
    kind.to_string()
}

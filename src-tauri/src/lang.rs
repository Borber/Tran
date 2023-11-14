use lingua::{Language, LanguageDetectorBuilder};

pub fn lang(text: &str) -> String {
    let first_lang = "zh_cn";
    let second_lang = "en";
    let languages = vec![Language::Chinese, Language::English];
    let detector = LanguageDetectorBuilder::from_languages(&languages).build();
    // 识别到非第一语言则翻译为第一语言
    // 识别到第一语言则翻译为第二语言
    let kind = if let Some(lang) = detector.detect_language_of(text) {
        match lang {
            Language::Chinese => second_lang,
            Language::English => first_lang,
        }
    } else {
        second_lang
    };
    kind.to_string()
}

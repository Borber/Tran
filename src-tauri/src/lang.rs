use lingua::{Language, LanguageDetectorBuilder};

/// 获取翻译目标语言
///
/// Get the translation target language
pub fn lang(text: &str) -> String {
    let first_lang = "zh_cn";
    let second_lang = "en";
    let languages = vec![Language::Chinese, Language::English];
    let detector = LanguageDetectorBuilder::from_languages(&languages).build();
    // 第一语言翻译为第二语言，其他翻译为第一语言
    // Translate the first language into the second language, and the other language into the first
    let kind = match detector.detect_language_of(text) {
        Some(lang) => match lang {
            Language::Chinese => second_lang,
            Language::English => first_lang,
        },
        None => first_lang,
    };
    kind.to_string()
}

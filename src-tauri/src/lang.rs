use lingua::{Language, LanguageDetectorBuilder};

// 翻译目标语言
pub fn lang(text: &str) -> String {
    let first_lang = "zh_cn";
    let second_lang = "en";
    let languages = vec![Language::Chinese, Language::English];
    let detector = LanguageDetectorBuilder::from_languages(&languages).build();
    // 第一语言翻译为第二语言，其他翻译为第一语言
    let kind = match detector.detect_language_of(text) {
        Some(lang) => match lang {
            Language::Chinese => second_lang,
            Language::English => first_lang,
        },
        None => first_lang,
    };
    kind.to_string()
}

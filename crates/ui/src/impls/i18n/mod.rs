use dioxus_i18n::prelude::*;
use unic_langid::{langid, LanguageIdentifier};

pub const DEFAULT_LANGUAGE: LanguageIdentifier = langid!("en-US");
pub const ZH_CN_LANGUAGE: LanguageIdentifier = langid!("zh-CN");

pub fn build_i18n_config() -> I18nConfig {
    I18nConfig::new(DEFAULT_LANGUAGE)
        .with_locale((
            DEFAULT_LANGUAGE,
            include_str!("./locales/en-US.ftl"),
        ))
        .with_locale((ZH_CN_LANGUAGE, include_str!("./locales/zh-CN.ftl")))
        .with_fallback(DEFAULT_LANGUAGE)
}

pub fn parse_language(input: &str) -> LanguageIdentifier {
    input.parse().unwrap_or(DEFAULT_LANGUAGE)
}

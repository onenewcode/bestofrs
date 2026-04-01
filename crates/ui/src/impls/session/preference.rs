#[cfg(feature = "server")]
mod server {
    use axum::extract::FromRequestParts;
    use axum::http::{request::Parts, HeaderMap};
    use dioxus::prelude::ServerFnError;

    use crate::impls::i18n::{parse_language, DEFAULT_LANGUAGE};
    use crate::impls::session::consts::{
        SESSION_PREFERENCE_GRID_THEME_KEY, SESSION_PREFERENCE_LOCALE_KEY,
        SESSION_PREFERENCE_PRIVACY_KEY, SESSION_PREFERENCE_THEME_KEY,
    };
    use crate::impls::session::AppSession;

    const DEFAULT_GRID_THEME: &str = "green";

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PreferenceField {
        Locale,
        Theme,
        GridTheme,
        Privacy,
    }

    impl PreferenceField {
        fn key(self) -> &'static str {
            match self {
                Self::Locale => SESSION_PREFERENCE_LOCALE_KEY,
                Self::Theme => SESSION_PREFERENCE_THEME_KEY,
                Self::GridTheme => SESSION_PREFERENCE_GRID_THEME_KEY,
                Self::Privacy => SESSION_PREFERENCE_PRIVACY_KEY,
            }
        }
    }

    pub fn get(session: &AppSession, field: PreferenceField) -> Option<String> {
        session.get::<String>(field.key())
    }

    pub fn set(session: &AppSession, field: PreferenceField, value: &str) {
        session.set(field.key(), value.to_string());
        session.set_store(true);
    }

    fn seed_if_absent(session: &AppSession, field: PreferenceField, value: &str) {
        if get(session, field).is_none() {
            session.set(field.key(), value.to_string());
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct PreferenceSeed {
        pub accept_language: Option<String>,
        pub user_agent: Option<String>,
        pub sec_ch_ua_platform: Option<String>,
        pub sec_ch_ua_mobile: Option<String>,
        pub inferred_locale: Option<String>,
    }

    impl From<&HeaderMap> for PreferenceSeed {
        fn from(headers: &HeaderMap) -> Self {
            let accept_language = read_header(headers, "accept-language");
            let user_agent = read_header(headers, "user-agent");
            let sec_ch_ua_platform = read_header(headers, "sec-ch-ua-platform");
            let sec_ch_ua_mobile = read_header(headers, "sec-ch-ua-mobile");
            let inferred_locale = infer_locale(accept_language.as_deref());

            Self {
                accept_language,
                user_agent,
                sec_ch_ua_platform,
                sec_ch_ua_mobile,
                inferred_locale,
            }
        }
    }

    impl<S> FromRequestParts<S> for PreferenceSeed
    where
        S: Send + Sync,
    {
        type Rejection = ServerFnError;

        async fn from_request_parts(
            parts: &mut Parts,
            _state: &S,
        ) -> Result<Self, Self::Rejection> {
            Ok((&parts.headers).into())
        }
    }

    pub fn seed_session_if_absent(session: &AppSession, seed: &PreferenceSeed) {
        if let Some(locale) = seed.inferred_locale.as_deref() {
            seed_if_absent(session, PreferenceField::Locale, locale);
        }
        seed_if_absent(session, PreferenceField::GridTheme, DEFAULT_GRID_THEME);
    }

    pub fn resolve_locale(session: &AppSession, seed: &PreferenceSeed) -> String {
        seed_session_if_absent(session, seed);
        get(session, PreferenceField::Locale).unwrap_or_else(|| DEFAULT_LANGUAGE.to_string())
    }

    pub fn update_locale(session: &AppSession, raw_locale: &str) {
        let locale = parse_language(raw_locale).to_string();
        set(session, PreferenceField::Locale, &locale);
    }

    pub fn resolve_theme(session: &AppSession) -> Option<String> {
        get(session, PreferenceField::Theme)
    }

    pub fn update_theme(session: &AppSession, theme: &str) {
        set(session, PreferenceField::Theme, theme);
    }

    pub fn resolve_grid_theme(session: &AppSession) -> String {
        let theme = get(session, PreferenceField::GridTheme)
            .unwrap_or_else(|| DEFAULT_GRID_THEME.to_string());
        normalize_grid_theme(&theme).to_string()
    }

    pub fn update_grid_theme(session: &AppSession, grid_theme: &str) {
        set(
            session,
            PreferenceField::GridTheme,
            normalize_grid_theme(grid_theme),
        );
    }

    fn normalize_grid_theme(raw: &str) -> &'static str {
        match raw {
            "red" => "red",
            "orange" => "orange",
            "yellow" => "yellow",
            "green" => "green",
            "cyan" => "cyan",
            "blue" => "blue",
            "purple" => "purple",
            _ => DEFAULT_GRID_THEME,
        }
    }

    fn infer_locale(raw: Option<&str>) -> Option<String> {
        let raw = raw?;
        raw.split(',').find_map(|part| {
            let locale = part.split(';').next()?.trim();
            if locale.is_empty() {
                return None;
            }
            Some(parse_language(locale).to_string())
        })
    }

    fn read_header(headers: &HeaderMap, key: &'static str) -> Option<String> {
        headers
            .get(key)
            .and_then(|value| value.to_str().ok())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    }
}

#[cfg(feature = "server")]
pub use server::*;

#[cfg(not(feature = "server"))]
mod client {
    use crate::impls::i18n::DEFAULT_LANGUAGE;
    use crate::impls::session::AppSession;

    const DEFAULT_GRID_THEME: &str = "green";

    #[derive(Clone, Debug, Default)]
    pub struct PreferenceSeed;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PreferenceField {
        Locale,
        Theme,
        GridTheme,
        Privacy,
    }

    pub fn get(_session: &AppSession, _field: PreferenceField) -> Option<String> {
        None
    }

    pub fn set(_session: &AppSession, _field: PreferenceField, _value: &str) {}

    pub fn seed_session_if_absent(_session: &AppSession, _seed: &PreferenceSeed) {}

    pub fn resolve_locale(_session: &AppSession, _seed: &PreferenceSeed) -> String {
        DEFAULT_LANGUAGE.to_string()
    }

    pub fn update_locale(_session: &AppSession, _raw_locale: &str) {}

    pub fn resolve_theme(_session: &AppSession) -> Option<String> {
        None
    }

    pub fn update_theme(_session: &AppSession, _theme: &str) {}

    pub fn resolve_grid_theme(_session: &AppSession) -> String {
        DEFAULT_GRID_THEME.to_string()
    }

    pub fn update_grid_theme(_session: &AppSession, _grid_theme: &str) {}
}

#[cfg(not(feature = "server"))]
pub use client::*;

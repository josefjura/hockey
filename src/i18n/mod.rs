pub mod middleware;

use fluent_static::message_bundle;

// Define the message bundle with both languages
#[message_bundle(
    resources = [
        ("src/i18n/messages/en.ftl", "en"),
        ("src/i18n/messages/cs.ftl", "cs"),
    ],
    default_language = "en"
)]
pub struct Messages;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Locale {
    #[default]
    English,
    Czech,
}

impl Locale {
    pub fn from_code(code: &str) -> Self {
        match code {
            "cs" | "cz" => Locale::Czech,
            _ => Locale::English,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Locale::English => "en",
            Locale::Czech => "cs",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Locale::English => "English",
            Locale::Czech => "Čeština",
        }
    }
}

/// Translation context - provides direct access to type-safe translations
#[derive(Debug, Clone)]
pub struct TranslationContext {
    pub locale: Locale,
    pub messages: Messages,
}

impl TranslationContext {
    pub fn new(locale: Locale) -> Self {
        use fluent_static::MessageBundle;
        let messages = Messages::get(locale.code()).unwrap_or_default();
        Self { locale, messages }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_from_code() {
        assert_eq!(Locale::from_code("en"), Locale::English);
        assert_eq!(Locale::from_code("cs"), Locale::Czech);
        assert_eq!(Locale::from_code("cz"), Locale::Czech);
        assert_eq!(Locale::from_code("unknown"), Locale::English);
    }

    #[test]
    fn test_locale_code() {
        assert_eq!(Locale::English.code(), "en");
        assert_eq!(Locale::Czech.code(), "cs");
    }

    #[test]
    fn test_translation_context() {
        let ctx = TranslationContext::new(Locale::English);
        assert_eq!(ctx.messages.nav_dashboard().to_string(), "Dashboard");
        assert_eq!(ctx.messages.nav_teams().to_string(), "Teams");
    }

    #[test]
    fn test_translation_context_czech() {
        let ctx = TranslationContext::new(Locale::Czech);
        assert_eq!(ctx.messages.nav_dashboard().to_string(), "Přehled");
        assert_eq!(ctx.messages.nav_teams().to_string(), "Týmy");
    }
}

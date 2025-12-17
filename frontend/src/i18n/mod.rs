use fluent::{FluentArgs, FluentBundle, FluentResource};
use unic_langid::{langid, LanguageIdentifier};

const EN_MESSAGES: &str = include_str!("messages/en.ftl");
const CS_MESSAGES: &str = include_str!("messages/cs.ftl");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Locale {
    #[default]
    English,
    Czech,
}

impl Locale {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            Locale::English => "English",
            Locale::Czech => "Čeština",
        }
    }

    fn lang_id(&self) -> LanguageIdentifier {
        match self {
            Locale::English => langid!("en"),
            Locale::Czech => langid!("cs"),
        }
    }

    fn messages(&self) -> &'static str {
        match self {
            Locale::English => EN_MESSAGES,
            Locale::Czech => CS_MESSAGES,
        }
    }
}


// FluentBundle with IntlLangMemoizer is not Send because it uses RefCell internally
// We create bundles on-demand per translation instead of caching them
pub struct I18n;

impl I18n {
    pub fn new() -> Self {
        Self
    }

    fn create_bundle(locale: Locale) -> FluentBundle<FluentResource> {
        let resource = FluentResource::try_new(locale.messages().to_string())
            .expect("Failed to parse Fluent resource");

        let mut bundle = FluentBundle::new(vec![locale.lang_id()]);
        bundle
            .add_resource(resource)
            .expect("Failed to add resource to bundle");

        bundle
    }

    pub fn translate(&self, locale: Locale, key: &str) -> String {
        self.translate_with_args(locale, key, None)
    }

    pub fn translate_with_args(
        &self,
        locale: Locale,
        key: &str,
        args: Option<&FluentArgs>,
    ) -> String {
        let bundle = Self::create_bundle(locale);

        let message = bundle.get_message(key).unwrap_or_else(|| {
            panic!(
                "Message '{}' not found in {} locale",
                key,
                locale.code()
            )
        });

        let pattern = message.value().unwrap_or_else(|| {
            panic!(
                "Message '{}' has no value in {} locale",
                key,
                locale.code()
            )
        });

        let mut errors = vec![];
        let value = bundle.format_pattern(pattern, args, &mut errors);

        if !errors.is_empty() {
            eprintln!("Translation errors for key '{}': {:?}", key, errors);
        }

        value.to_string()
    }
}

impl Default for I18n {
    fn default() -> Self {
        Self::new()
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
    fn test_i18n_translate_english() {
        let i18n = I18n::new();
        assert_eq!(
            i18n.translate(Locale::English, "nav-dashboard"),
            "Dashboard"
        );
        assert_eq!(i18n.translate(Locale::English, "nav-teams"), "Teams");
    }

    #[test]
    fn test_i18n_translate_czech() {
        let i18n = I18n::new();
        assert_eq!(i18n.translate(Locale::Czech, "nav-dashboard"), "Přehled");
        assert_eq!(i18n.translate(Locale::Czech, "nav-teams"), "Týmy");
    }
}

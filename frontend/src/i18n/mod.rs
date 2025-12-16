// Internationalization module
// For now, we'll use simple string constants
// TODO: Implement fluent-rs integration for proper i18n

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
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

    #[allow(dead_code)]
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
}

pub struct Translations {
    pub locale: Locale,
}

impl Translations {
    pub fn new(locale: Locale) -> Self {
        Self { locale }
    }

    // Navigation items
    pub fn nav_dashboard(&self) -> &'static str {
        match self.locale {
            Locale::English => "Dashboard",
            Locale::Czech => "Přehled",
        }
    }

    pub fn nav_teams(&self) -> &'static str {
        match self.locale {
            Locale::English => "Teams",
            Locale::Czech => "Týmy",
        }
    }

    pub fn nav_players(&self) -> &'static str {
        match self.locale {
            Locale::English => "Players",
            Locale::Czech => "Hráči",
        }
    }

    pub fn nav_events(&self) -> &'static str {
        match self.locale {
            Locale::English => "Events",
            Locale::Czech => "Události",
        }
    }

    pub fn nav_seasons(&self) -> &'static str {
        match self.locale {
            Locale::English => "Seasons",
            Locale::Czech => "Sezóny",
        }
    }

    pub fn nav_matches(&self) -> &'static str {
        match self.locale {
            Locale::English => "Matches",
            Locale::Czech => "Zápasy",
        }
    }

    pub fn nav_management(&self) -> &'static str {
        match self.locale {
            Locale::English => "Management",
            Locale::Czech => "Správa",
        }
    }

    // User menu
    pub fn logout(&self) -> &'static str {
        match self.locale {
            Locale::English => "Logout",
            Locale::Czech => "Odhlásit se",
        }
    }

    pub fn language(&self) -> &'static str {
        match self.locale {
            Locale::English => "Language",
            Locale::Czech => "Jazyk",
        }
    }
}

impl Default for Translations {
    fn default() -> Self {
        Self::new(Locale::English)
    }
}

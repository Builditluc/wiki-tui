use serde::Serialize;

macro_rules! language {
    ($($lang:ident, $lang_name:expr, $lang_native:expr, $lang_code:expr);+) => {
        #[derive(Debug, Clone, Serialize)]
        pub enum Language {
            $($lang,)*
        }

        impl Language {
            /// Returns the data associated to the language. It's formatted like this:
            /// (Language name in English, Local Language name, Language Code)
            fn language_data(&self) -> (&str, &str, &str) {
                match self {
                    $(
                    Language::$lang => (stringify!($lang), stringify!($lang_name), stringify!($lang_code)),
                    )*
                }
            }

            /// Returns the English name of the language
            pub fn name(&self) -> &str {
                self.language_data().0
            }

            /// Returns the local name of the language
            pub fn local_name(&self) -> &str {
                self.language_data().1
            }

            /// Returns the language code
            pub fn code(&self) -> &str {
                self.language_data().2
            }
        }

        impl From<&str> for Language {
            fn from(s: &str) -> Self {
                match s.to_ascii_lowercase().as_ref() {
                    $(
                    stringify!($lang_code) => Language::$lang,
                    )*
                    _ => Language::default()
                }
            }
        }

        pub static LANGUAGES: &[Language] = &[$(Language::$lang,)*];
    };
}

// format:
// Identifier of the language, Language name in english, Local Language name, Language code
language!(
    Abkhaz, Abkhaz, аԥсшәа, ab;
    Acehnese, Acehnese, Acèh, ace;
    English, English, English, en;
    German, German, Deutsch, de
);

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

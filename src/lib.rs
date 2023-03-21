use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    io::BufReader,
};

use proc_macro::TokenStream;
use serde::{de::IgnoredAny, Deserialize, Serialize};
use syn::{
    export::{quote::quote, Span},
    parse_macro_input, Ident, LitStr,
};

#[derive(Debug, Serialize, Deserialize)]
struct WLanguage {
    code: String,
    name: String,
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    site: Vec<IgnoredAny>,
    dir: String,
    localname: String,
    #[serde(skip)]
    identifier: Option<Ident>,
}

#[derive(Debug, Deserialize)]
struct SiteMatrix {
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    count: usize,
    #[serde(flatten)]
    languages: HashMap<String, WLanguage>,
}
#[derive(Debug, Deserialize)]
struct SiteMatrixWrapper {
    #[serde(rename = "sitematrix")]
    site_matrix: SiteMatrix,
}

#[proc_macro]
pub fn parse_languages(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr).value();
    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);
    let sitematrix: SiteMatrixWrapper = serde_json::from_reader(reader).unwrap();
    let languages =
        sitematrix
            .site_matrix
            .languages
            .into_iter()
            .fold(BTreeMap::new(), |mut acc, e| {
                let mut e = e.1;
                let name = if acc.contains_key(&e.localname) {
                    normalize_string(&(e.localname.clone() + &e.code))
                } else {
                    normalize_string(&e.localname)
                };
                e.identifier = Some(Ident::new(&name, Span::call_site()));
                acc.insert(name, e);
                acc
            });
    let mut variants = quote!();
    let mut language_data_arms = quote!();
    let mut from_str_arms = quote!();
    let mut array_def = quote!();
    for (_key, value) in languages {
        let ident = value.identifier.clone().unwrap();
        let en_name = value.localname.clone();
        let lang_name = value.name.clone();
        let lang_code = value.code.clone();
        variants = quote! {
            #variants
            #ident,
        };
        language_data_arms = quote! {
            #language_data_arms
            Language::#ident => (#en_name, #lang_name, #lang_code),
        };
        from_str_arms = quote! {
            #from_str_arms
            #lang_code | #lang_name => Language::#ident,
        };
        array_def = quote! {
            #array_def
            Language::#ident,
        }
    }

    let expanded = quote! {

        #[derive(Debug, Clone, Serialize)]
        pub enum Language{
            #variants
        }

        impl Language{
            /// Returns the data associated to the language. It's formatted like this:
            /// (Language name in English, Local Language name, Language Code)
            fn language_data(&self) -> (&str, &str, &str) {
                match self {
                    #language_data_arms
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
                match s.as_ref() {
                    #from_str_arms
                    _ => Language::default()
                }
            }
        }

        pub static LANGUAGES: &[Language] = &[#array_def];

        impl Default for Language {
            fn default() -> Self {
                Language::English
            }
        }

    };
    proc_macro::TokenStream::from(expanded)
}

fn normalize_string(input: &str) -> String {
    input
        .chars()
        .enumerate()
        .map(|(index, c)| {
            if c.is_alphabetic() {
                if index == 0 {
                    c.to_uppercase().to_string()
                } else {
                    c.to_string()
                }
            } else {
                "".to_string()
            }
        })
        .collect()
}

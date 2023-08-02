use cursive::{theme::ColorStyle, Vec2, View};

use crate::wiki::language::Language;

const MIN_LENGTH_FULL: usize = 81;
const MIN_LENGTH_SHORT: usize = 39;

pub struct StatusBar {
    title: String,
    language: Language,
    available_languages: usize,
}

impl StatusBar {
    pub fn new() -> Self {
        StatusBar {
            title: String::new(),
            language: Language::default(),
            available_languages: 0,
        }
    }

    pub fn article_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn language(mut self, language: &Language) -> Self {
        self.language = language.to_owned();
        self
    }

    pub fn available_languages(mut self, available_languages: usize) -> Self {
        self.available_languages = available_languages;
        self
    }
}

impl View for StatusBar {
    fn draw(&self, printer: &cursive::Printer) {
        printer.with_color(ColorStyle::highlight_inactive(), |printer| {
            printer.print_hline((0, 0), printer.size.x, " ")
        });

        if printer.size.x >= MIN_LENGTH_FULL {
            printer.print(
                (1, 0),
                &(" ".to_string()
                    + format!(
                        "wiki-tui | {} article '{}' | {} other languages available",
                        self.language.name(),
                        self.title,
                        self.available_languages
                    )
                    .as_str()),
            );
            debug!("draw status bar finished");
            return;
        }

        if printer.size.x >= MIN_LENGTH_SHORT {
            printer.print(
                (1, 0),
                &(" ".to_string()
                    + format!(
                        "wiki-tui | {} | {}",
                        self.language.name(),
                        self.available_languages
                    )
                    .as_str()),
            );
            debug!("draw status bar finished");
            return;
        }

        printer.print((1, 0), " wiki-tui");
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        Vec2::new(MIN_LENGTH_FULL, 1)
    }
}

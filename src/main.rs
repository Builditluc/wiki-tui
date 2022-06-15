extern crate anyhow;
extern crate lazy_static;
extern crate log;

use cursive::align::HAlign;
use cursive::backends;
use cursive::theme::*;
use cursive::traits::*;
use cursive::view::Resizable;
use cursive::views::*;
use cursive::Cursive;
use cursive_buffered_backend::BufferedBackend;
use std::fs;
use std::io::Write;

use crate::wiki::search::SearchResult;

pub mod cli;
pub mod config;
pub mod error;
pub mod logging;
pub mod ui;
pub mod wiki;

pub const LOGO: &str = "
 _      __   (_)   / /__   (_)         / /_  __  __   (_) 
| | /| / /  / /   / //_/  / /  ______ / __/ / / / /  / /  
| |/ |/ /  / /   / ,<    / /  /_____// /_  / /_/ /  / /   
|__/|__/  /_/   /_/|_|  /_/          \\__/  \\__,_/  /_/    
";

#[cfg(feature = "blt-backend")]
fn backend() -> Box<BufferedBackend> {
    let blt_backend = backends::blt::Backend::init();
    let buffered_backend = BufferedBackend::new(blt_backend);
    Box::new(buffered_backend)
}

#[cfg(feature = "termion-backend")]
fn backend() -> Box<BufferedBackend> {
    let termion_backend = backends::termion::Backend::init().unwrap();
    let buffered_backend = BufferedBackend::new(termion_backend);
    Box::new(buffered_backend)
}

#[cfg(feature = "crossterm-backend")]
fn backend() -> Box<BufferedBackend> {
    let crossterm_backend = backends::crossterm::Backend::init().unwrap();
    let buffered_backend = BufferedBackend::new(crossterm_backend);
    Box::new(buffered_backend)
}

#[cfg(feature = "pancurses-backend")]
fn backend() -> Box<BufferedBackend> {
    let pancurses_backend = backends::curses::pan::Backend::init().unwrap();
    let buffered_backend = BufferedBackend::new(pancurses_backend);
    Box::new(buffered_backend)
}

#[cfg(feature = "ncurses-backend")]
fn backend() -> Box<BufferedBackend> {
    let ncurses_backend = backends::curses::n::Backend::init().unwrap();
    let buffered_backend = BufferedBackend::new(ncurses_backend);
    Box::new(buffered_backend)
}

fn main() {
    error::create_hook(|path, data| {
        if let Some(path) = path {
            let mut fs = fs::File::create(path).unwrap();
            fs.write_all(data.as_bytes())
                .expect("Unable to generate report");
        };
    });

    initialize();
    start_application();
}

fn initialize() {
    #[cfg(debug_assertions)]
    println!("{}", LOGO);

    // create and initialize the logger
    logging::Logger::new().initialize();
}

fn start_application() {
    let mut siv = Cursive::new();
    siv.add_global_callback('q', Cursive::quit);

    // get and apply the color theme
    let theme = Theme {
        palette: get_color_palette(),
        ..Default::default()
    };
    siv.set_theme(theme);

    // Create the views
    let search_bar = EditView::new()
        .on_submit(|s, q| ui::search::on_search(s, q.to_string()))
        .style({
            if let Some(search_theme) = &config::CONFIG.theme.search_bar {
                if search_theme.background == search_theme.secondary {
                    ColorStyle::new(search_theme.background, search_theme.text)
                } else {
                    ColorStyle::secondary()
                }
            } else {
                ColorStyle::secondary()
            }
        })
        .with_name("search_bar")
        .full_width();

    let search_layout = view_with_theme!(
        config::CONFIG.theme.search_bar,
        Dialog::around(LinearLayout::horizontal().child(search_bar))
            .title("Search")
            .title_position(cursive::align::HAlign::Left)
    );

    let logo_view = TextView::new(LOGO)
        .h_align(HAlign::Center)
        .with_name("logo_view")
        .full_screen();

    let article_layout = override_keybindings!(LinearLayout::horizontal()
        .child(Dialog::around(logo_view))
        .with_name("article_layout"));

    // Add a fullscreen layer, containing the search bar and the article view
    siv.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(search_layout)
                .child(article_layout),
        )
        .title("wiki-tui")
        .button("Quit", Cursive::quit)
        .full_screen(),
    );

    // Start the application
    let argument_callback = handle_arguments();
    if let Err(error) = siv.cb_sink().send(argument_callback) {
        log::error!("{:?}", error);
    }

    let siv_box = std::sync::Mutex::new(siv);
    #[allow(clippy::redundant_closure)]
    if std::panic::catch_unwind(|| siv_box.lock().unwrap().run_with(|| backend())).is_err() {
        error::print_panic();
    }
}

fn handle_arguments() -> Box<dyn FnOnce(&mut Cursive) + Send> {
    if let Some(search_query) = config::CONFIG.get_args().search_query.as_ref() {
        log::info!("searching for the article: {}", search_query);
        return Box::new(move |siv: &mut Cursive| {
            ui::search::on_search(siv, search_query.to_string());
        });
    } else if let Some(article_id) = config::CONFIG.get_args().article_id {
        log::info!("opening the article: {}", article_id);
        return Box::new(move |siv: &mut Cursive| {
            ui::article::on_article_submit(
                siv,
                &SearchResult::new(
                    String::new(),
                    0,
                    article_id,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
            );
        });
    }

    Box::new(|_: &mut Cursive| {})
}

fn get_color_palette() -> Palette {
    let mut custom_palette = Palette::default();

    custom_palette.set_color("View", config::CONFIG.theme.background);
    custom_palette.set_color("Primary", config::CONFIG.theme.text);
    custom_palette.set_color("TitlePrimary", config::CONFIG.theme.title);
    custom_palette.set_color("Highlight", config::CONFIG.theme.highlight);
    custom_palette.set_color("HighlightInactive", config::CONFIG.theme.highlight_inactive);
    custom_palette.set_color("HighlightText", config::CONFIG.theme.highlight_text);

    custom_palette
}

#[macro_use]
extern crate anyhow;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate cursive;

use anyhow::Context;
use cursive::event::Key;
use cursive::theme::*;
use cursive::Cursive;
use home::display_home;
use std::fs;
use std::io::Write;
use ui::language_selector::language_selection_popup;

use crate::backend::backend;

mod backend;
mod cli;
mod config;
mod error;
mod home;
mod logging;
mod ui;
mod wiki;

pub const LOGO: &str = "
 _      __   (_)   / /__   (_)         / /_  __  __   (_) 
| | /| / /  / /   / //_/  / /  ______ / __/ / / / /  / /  
| |/ |/ /  / /   / ,<    / /  /_____// /_  / /_/ /  / /   
|__/|__/  /_/   /_/|_|  /_/          \\__/  \\__,_/  /_/    
";

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
    if let Err(error) = logging::Logger::new()
        .initialize()
        .context("failed initializing the logger")
    {
        println!("Error: {:?}", error);
    }
}

fn start_application() {
    let mut siv = Cursive::new();
    siv.add_global_callback('q', Cursive::quit);
    siv.add_global_callback(Key::Esc, |s| {
        if s.pop_layer().is_none() || s.screen().is_empty() {
            s.quit();
        };
    });
    siv.add_global_callback(Key::F2, language_selection_popup);

    // get and apply the color theme
    let theme = Theme {
        palette: get_color_palette(),
        ..Default::default()
    };
    siv.set_theme(theme);

    // show the home screen
    if let Err(error) = siv.cb_sink().send(display_home()) {
        error!("{:?}", error);
    }

    // Start the application
    let argument_callback = handle_arguments();
    if let Err(error) = siv.cb_sink().send(argument_callback) {
        error!("{:?}", error);
    }

    let siv_box = std::sync::Mutex::new(siv);
    #[allow(clippy::redundant_closure)]
    if std::panic::catch_unwind(|| siv_box.lock().unwrap().run_with(|| backend())).is_err() {
        error::print_panic();
    }
}

fn handle_arguments() -> Box<dyn FnOnce(&mut Cursive) + Send> {
    if let Some(search_query) = config::CONFIG.get_args().search_query.as_ref() {
        info!("searching for the article: {}", search_query);
        return Box::new(move |siv: &mut Cursive| {
            ui::search::on_search(siv, search_query);
        });
    }
    // else if let Some(article_id) = config::CONFIG.get_args().article_id {
    //     info!("opening the article: {}", article_id);
    //     return Box::new(move |siv: &mut Cursive| {
    //         ui::article::on_article_submit(
    //             siv,
    //             &SearchResult::new(
    //                 String::new(),
    //                 0,
    //                 article_id,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //                 None,
    //             ),
    //         );
    //     });
    // }

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

use cursive::backends;
use cursive_buffered_backend::BufferedBackend;

#[cfg(feature = "blt-backend")]
pub fn backend() -> Box<BufferedBackend> {
    let blt_backend = backends::blt::Backend::init();
    let buffered_backend = BufferedBackend::new(blt_backend);
    Box::new(buffered_backend)
}

#[cfg(feature = "termion-backend")]
pub fn backend() -> Box<BufferedBackend> {
    let termion_backend = backends::termion::Backend::init().unwrap();
    let buffered_backend = BufferedBackend::new(termion_backend);
    Box::new(buffered_backend)
}

#[cfg(feature = "crossterm-backend")]
pub fn backend() -> Box<BufferedBackend> {
    let crossterm_backend = backends::crossterm::Backend::init().unwrap();
    let buffered_backend = BufferedBackend::new(crossterm_backend);
    Box::new(buffered_backend)
}

#[cfg(feature = "pancurses-backend")]
pub fn backend() -> Box<BufferedBackend> {
    let pancurses_backend = backends::curses::pan::Backend::init().unwrap();
    let buffered_backend = BufferedBackend::new(pancurses_backend);
    Box::new(buffered_backend)
}

#[cfg(feature = "ncurses-backend")]
pub fn backend() -> Box<BufferedBackend> {
    let ncurses_backend = backends::curses::n::Backend::init().unwrap();
    let buffered_backend = BufferedBackend::new(ncurses_backend);
    Box::new(buffered_backend)
}

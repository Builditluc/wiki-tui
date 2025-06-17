use std::sync::OnceLock;
use reqwest::Proxy;
use std::net::SocketAddr;

static PROXY: OnceLock<Proxy> = OnceLock::new();

fn validate_proxy(addr: &str) -> Result<(), String> {
    addr.parse::<SocketAddr>()
        .map(|_| ())
        .map_err(|_| format!("Invalid proxy address: {}", addr))
}

// Init proxy only once
pub fn init_proxy(str: &str) -> Result<(), String> {
    validate_proxy(&str)?;
    let proxy = match Proxy::all(str) {
        Ok(o) => o,
        Err(e) => return Err(e.to_string()),
    };
    PROXY.set(proxy).expect("init error");
    Ok(())
}

pub fn get_proxy() -> Option<&'static Proxy> {
    PROXY.get()
}

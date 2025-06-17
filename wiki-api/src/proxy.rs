use std::sync::OnceLock;
use reqwest::Proxy;
use std::net::SocketAddr;

static PROXY: OnceLock<Proxy> = OnceLock::new();

fn validate_proxy(addr: &str) -> Result<(), String> {
    let addr_clean = if let Some(addr) = addr.strip_prefix("socks5://") {
        addr
    } else if let Some(addr) = addr.strip_prefix("http://") {
        addr
    } else if let Some(addr) = addr.strip_prefix("https://") {
        addr
    } else {
        return Err(format!("Invalid proxy protocol: {}", addr));
    };

    addr_clean.parse::<SocketAddr>()
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

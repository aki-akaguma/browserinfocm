#[cfg(not(feature = "backend_next"))]
mod db_sqlite;
#[cfg(not(feature = "backend_next"))]
pub use db_sqlite::*;

#[cfg(feature = "backend_next")]
mod forwarder;
#[cfg(feature = "backend_next")]
pub use forwarder::*;

pub use super::SaveBroInfoRequest;
#[cfg(feature = "backend_user_agent")]
pub use super::SaveUserAgentRequest;

#[allow(unused)]
#[cfg(feature = "server")]
pub fn get_ip_address_string(headers: &dioxus::fullstack::HeaderMap) -> String {
    use std::net::IpAddr;
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim())
        .filter(|s| s.parse::<IpAddr>().is_ok())
        .unwrap_or("")
        .to_string()
}

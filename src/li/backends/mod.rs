#[cfg(not(feature = "backend_next"))]
mod db_sqlite;
#[cfg(not(feature = "backend_next"))]
pub use db_sqlite::*;

#[cfg(feature = "backend_next")]
mod forwarder;
#[cfg(feature = "backend_next")]
pub use forwarder::*;

/*
#[cfg(not(feature = "next_backend"))]
mod debug;
#[cfg(not(feature = "next_backend"))]
pub use debug::*;
*/

#[allow(unused)]
#[cfg(feature = "server")]
pub fn get_ipaddress_string(headers: &dioxus::fullstack::HeaderMap) -> String {
    let ip = if let Some(s) = headers.get("x-forwarded-for") {
        // format:
        //     X-Forwarded-For: client1, proxy1, proxy2, ...
        let ss = s.to_str().unwrap();
        if let Some(idx) = ss.find(',') {
            ss[..idx].to_string()
        } else {
            ss.to_string()
        }
    } else {
        "".to_string()
    };
    ip
}

//! Entry point for the browserinfocm application.
//! Sets up the logger, server URL, and launches the Dioxus app.

use dioxus::prelude::*;

mod li;

use li::BrowserInfoCm;
use li::BrowserInfoState;

fn main() {
    #[cfg(feature = "server")]
    li::backend_init().expect("faile to init backend");

    // you can set the ports and IP manually with env vars:
    // server launch:
    // IP="0.0.0.0" PORT=8080 ./server

    #[cfg(not(debug_assertions))]
    let level = dioxus::logger::tracing::Level::INFO;
    #[cfg(debug_assertions)]
    let level = dioxus::logger::tracing::Level::DEBUG;
    dioxus::logger::init(level).expect("failed to init logger");

    // Configure the server URL for desktop/mobile builds to connect to the backend.
    #[cfg(not(debug_assertions))]
    #[cfg(any(feature = "desktop", feature = "mobile"))]
    {
        let backend_url = std::env::var("BROWSERINFOCM_SERVER_URL")
            .ok()
            .or(option_env!("BROWSERINFOCM_SERVER_URL").map(|s| s.to_string()))
            .unwrap_or_else(|| "https://aki.omusubi.org/broinfo".to_string());
        let static_url: &'static str = Box::leak(backend_url.into_boxed_str());
        dioxus::fullstack::set_server_url(static_url);
    }

    dioxus::launch(App);
}

/// The root component of the application.
#[component]
fn App() -> Element {
    rsx! {
        BroInfoHome {}
    }
}

/// A demonstration component that displays gathered browser information.
#[component]
fn BroInfoHome() -> Element {
    let mut db_path_sig = use_signal(String::new);
    use_future(move || async move {
        match li::get_db_path().await {
            Ok(s) => db_path_sig.set(s),
            Err(e) => dioxus::logger::tracing::error!("Failed to get DB path: {e}"),
        }
    });
    let db_path_s = db_path_sig.read().clone();

    let state_sig = use_signal(BrowserInfoState::default);

    let state = state_sig.read();
    let brg_s = format!("{:?}", state.browser);
    let bim_s = format!("{:?}", state.broinfo);
    let bicmid_s = state.bicmid.clone();
    let user_s = state.user.clone();

    rsx! {
        BrowserInfoCm {
            state: state_sig,
        }
        div {
            h3 { "System Information" }
            div { "Database: {db_path_s}" }
            div { "Browser Info (Condensed): {brg_s}" }
            hr {}
            h3 { "Session Details" }
            div { "BICMID: {bicmid_s}" }
            div { "User: {user_s}" }
            hr {}
            h3 { "Full Browser Information" }
            div { "{bim_s}" }
        }
    }
}

use dioxus::prelude::*;

mod li;

use browserinfo::{BroInfo, Browser};
use li::BrowserInfoCm;

fn main() {
    // you can set the ports and IP manually with env vars:
    // server launch:
    // IP="0.0.0.0" PORT=8080 ./server

    #[cfg(not(debug_assertions))]
    let level = dioxus_logger::tracing::Level::INFO;
    #[cfg(debug_assertions)]
    let level = dioxus_logger::tracing::Level::DEBUG;
    dioxus_logger::init(level).expect("failed to init logger");

    #[cfg(not(debug_assertions))]
    #[cfg(any(feature = "desktop", feature = "mobile"))]
    {
        let backend_url = "https://aki.omusubi.org/broinfo";
        dioxus_fullstack::set_server_url(backend_url);
    }

    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        MyStyle {}
        BroInfoHome {}
    }
}

#[component]
fn MyStyle() -> Element {
    rsx! {}
}

#[component]
fn BroInfoHome() -> Element {
    let mut db_path_sig = use_signal(String::new);
    use_future(move || async move {
        let s = li::get_db_path().await;
        db_path_sig.set(s.unwrap());
    });
    let db_path_s = format!("{:?}", db_path_sig.read().clone());

    let broinfo_sig = use_signal(BroInfo::default);
    let browser_sig = use_signal(Browser::default);
    let bicmid_sig = use_signal(String::new);
    let user_sig = use_signal(String::new);

    let brg = browser_sig.read().clone();
    let bim = broinfo_sig.read().clone();
    let brg_s = format!("{:?}", brg);
    let bim_s = format!("{:?}", bim);

    let bicmid = bicmid_sig.read().clone();
    let bicmid_s = format!("{:?}", bicmid);
    let user = user_sig.read().clone();
    let user_s = format!("{:?}", user);

    rsx! {
        BrowserInfoCm {
            broinfo: broinfo_sig,
            browser: browser_sig,
            bicmid: bicmid_sig,
            user: user_sig,
        }
        div { "{db_path_s}" }
        div { "{brg_s}" }
        div {}
        div { "{bim_s}" }
        div {}
        div { "{bicmid_s}" }
        div {}
        div { "{user_s}" }
    }
}

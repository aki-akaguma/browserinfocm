use dioxus::prelude::*;
#[cfg(all(not(debug_assertions), feature = "desktop"))]
use dioxus_desktop::{Config, WindowBuilder};

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
    let broinfo_sig = use_signal(BroInfo::default);
    let browser_sig = use_signal(Browser::default);

    let brg = browser_sig.read().clone();
    let bim = broinfo_sig.read().clone();
    let brg_s = format!("{:?}", brg);
    let bim_s = format!("{:?}", bim);

    rsx! {
        BrowserInfoCm {
            broinfo: broinfo_sig,
            browser: browser_sig,
        }
        div {
            "{brg_s}"
        }
        div {}
        div {
            "{bim_s}"
        }
    }
}

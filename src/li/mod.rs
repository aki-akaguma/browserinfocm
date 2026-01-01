use anyhow::Result;
use browserinfo::{broinfo_js, BroInfo, Browser};
use dioxus::prelude::*;

#[cfg(feature = "backend_user_agent")]
use browserinfo::{user_agent_js, UserAgent};

mod backends;

#[cfg(feature = "server")]
#[allow(unused_imports)]
pub use backends::get_ipaddress_string;

#[derive(Props, Debug, Clone, PartialEq)]
pub struct BrowserInfoProps {
    //// `broinfo` is set by this package.
    broinfo: Signal<BroInfo>,
    //// `browser` is set by this package.
    browser: Signal<Browser>,
    //// `bicmid` is a base64 encoded uuid stored in the browser's local storage.
    //// `bicmid` is set by this package.
    bicmid: Signal<String>,
    //// `user` can be used freely.
    //// `user` is NOT set by this package. The value set to `user` is passed to the backend.
    user: Signal<String>,
}

#[component]
pub fn BrowserInfoCm(mut props: BrowserInfoProps) -> Element {
    use_future(move || async move {
        let (broinfo, browser, bicmid) = get_browserinfo((props.user)()).await.unwrap();
        props.broinfo.set(broinfo);
        props.browser.set(browser);
        props.bicmid.set(bicmid);
    });

    rsx! {}
}

pub async fn get_browserinfo(user: String) -> Result<(BroInfo, Browser, String)> {
    let bicmid = get_or_create_bicmid();
    //
    #[cfg(feature = "backend_user_agent")]
    {
        let js_ua: &str = user_agent_js();
        let v = document::eval(js_ua).await?;
        let s = v.to_string();
        let user_agent = UserAgent::from_json_str(&s)?;
        let _ = backends::save_user_agent(user_agent).await;
    }
    //
    let js_bro: &str = broinfo_js();
    let v = document::eval(js_bro).await?;
    let s = v.to_string();
    let broinfo = BroInfo::from_json_str(&s)?;
    //dioxus_logger::tracing::debug!("{s:?}");
    let browser = backends::save_broinfo(broinfo.clone(), bicmid.clone(), user, true)
        .await?
        .unwrap();
    Ok((broinfo, browser, bicmid))
}

//// a base64 encoded uuid on browser's local strage.
#[cfg(target_arch = "wasm32")]
fn get_or_create_bicmid() -> String {
    use base64::Engine;

    let window = web_sys::window().expect("no window");
    let storage = window
        .local_storage()
        .expect("localStorage error")
        .expect("localStorage not available");
    if let Ok(Some(uuid_s)) = storage.get_item("anon_bicmid") {
        uuid_s
    } else {
        // generate a uuid (128bits:16byte)
        let uuid = uuid::Uuid::new_v4();
        let uuid_s = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(uuid.as_bytes());
        storage.set_item("anon_bicmid", &uuid_s).unwrap();
        uuid_s
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_or_create_bicmid() -> String {
    "".to_string()
}

pub async fn get_db_path() -> Result<String> {
    backends::get_db_path().await
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_01() {
        let s = broinfo_js();
        assert_ne!(s, "");
    }
}

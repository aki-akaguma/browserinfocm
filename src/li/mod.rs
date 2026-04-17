use anyhow::Result;
use browserinfo::{broinfo_js, BroInfo, Browser};
use dioxus::prelude::*;

#[cfg(feature = "backend_user_agent")]
use browserinfo::{user_agent_js, UserAgent};

mod backends;

#[cfg(feature = "server")]
#[allow(unused_imports)]
pub use backends::get_ip_address_string;

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
        let bicmid = get_or_create_bicmid().await.unwrap();
        props.bicmid.set(bicmid.clone());
        let (broinfo, browser) = get_browserinfo(bicmid, (props.user)()).await.unwrap();
        props.broinfo.set(broinfo);
        props.browser.set(browser);
    });

    rsx! {}
}

pub async fn get_browserinfo(bicmid: String, user: String) -> Result<(BroInfo, Browser)> {
    use browserinfo::FromJsonStr;
    //
    #[cfg(feature = "backend_user_agent")]
    {
        let js_ua: &str = user_agent_js();
        let v = document::eval(js_ua).await?;
        let s = v.to_string();
        dioxus::logger::tracing::debug!("Raw JSON from JS: {s}");
        let user_agent = UserAgent::from_json_str(&s)?;
        let _ = backends::save_user_agent(user_agent).await;
    }
    //
    let js_bro: &str = broinfo_js();
    let v = document::eval(js_bro).await?;
    let s = v.to_string();
    dioxus::logger::tracing::debug!("Raw JSON from JS: {s}");
    let broinfo = BroInfo::from_json_str(&s)?;
    let browser = backends::save_broinfo(broinfo.clone(), bicmid, user, true)
        .await?
        .unwrap();
    Ok((broinfo, browser))
}

// a base64 encoded uuid on browser's local strage.
async fn get_or_create_bicmid() -> Result<String> {
    use base64::Engine;

    // check 'localStrage'
    let v =
        document::eval(r#"{var r=false;if('localStorage' in window){r=true}return r;}"#).await?;
    if !v.as_bool().unwrap_or(false) {
        return Ok("".to_string());
    }
    // get from localStrage
    let js_get: &str = r#"{return window.localStorage.getItem('anon_bicmid');}"#;
    let v = document::eval(js_get).await?;
    let ss = v.as_str().unwrap_or("");
    //dioxus::logger::tracing::debug!("anon_bicmid: '{ss}'");
    if !ss.is_empty() {
        Ok(ss.to_string())
    } else {
        // generate a uuid (128bits:16byte)
        let uuid = uuid::Uuid::new_v4();
        let uuid_s = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(uuid.as_bytes());
        // set into localStrage
        let js_set =
            format!(r#"{{window.localStorage.setItem('anon_bicmid','{uuid_s}');return '';}}"#);
        let _v = document::eval(&js_set).await?;
        Ok(uuid_s)
    }
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

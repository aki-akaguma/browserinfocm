//! Main module for browser information management.
//! Provides the Dioxus component and utilities for gathering and saving browser data.

use anyhow::Result;
use browserinfo::{broinfo_js, BroInfo, Browser};
use dioxus::prelude::*;

#[cfg(feature = "backend_user_agent")]
use browserinfo::{user_agent_js, UserAgent};

mod backends;

use serde::{Deserialize, Serialize};

/// Request structure for saving browser information to the backend.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct SaveBroInfoRequest {
    /// The detailed browser information gathered from the client.
    pub broinfo: BroInfo,
    /// The anonymous browser identifier (BICMID).
    pub bicmid: String,
    /// Custom user identifier string.
    pub user: String,
    /// Whether to return the parsed Browser struct in the response.
    pub return_browser: bool,
}

/// Request structure for saving only the user agent string.
#[cfg(feature = "backend_user_agent")]
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub struct SaveUserAgentRequest {
    /// The user agent information.
    pub ua: UserAgent,
}

#[cfg(feature = "server")]
#[allow(unused_imports)]
pub use backends::get_ip_address_string;

/// Properties for the `BrowserInfoCm` component.
#[derive(Props, Debug, Clone, PartialEq)]
pub struct BrowserInfoProps {
    /// Signal to store the gathered `BroInfo`. Set by the component.
    broinfo: Signal<BroInfo>,
    /// Signal to store the parsed `Browser` information. Set by the component.
    browser: Signal<Browser>,
    /// Signal to store the `bicmid` (anonymous identifier). Set by the component.
    bicmid: Signal<String>,
    /// Signal for the user identifier. This is passed from the parent to the backend.
    user: Signal<String>,
}

/// A Dioxus component that automatically gathers browser information and an anonymous ID (BICMID).
/// It persists this data to the configured backend on mount.
#[component]
pub fn BrowserInfoCm(mut props: BrowserInfoProps) -> Element {
    use_future(move || async move {
        match get_or_create_bicmid().await {
            Ok(bicmid) => {
                props.bicmid.set(bicmid.clone());
                match get_browserinfo(bicmid, (props.user)()).await {
                    Ok((broinfo, browser)) => {
                        props.broinfo.set(broinfo);
                        props.browser.set(browser);
                    }
                    Err(e) => dioxus::logger::tracing::error!("Failed to get browser info: {e}"),
                }
            }
            Err(e) => dioxus::logger::tracing::error!("Failed to get or create bicmid: {e}"),
        }
    });

    rsx! {}
}

/// Gathers browser information using JavaScript execution and saves it to the backend.
///
/// Returns a tuple of `(BroInfo, Browser)` on success.
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
        let _ = backends::save_user_agent(SaveUserAgentRequest { ua: user_agent }).await;
    }
    //
    let js_bro: &str = broinfo_js();
    let v = document::eval(js_bro).await?;
    let s = v.to_string();
    dioxus::logger::tracing::debug!("Raw JSON from JS: {s}");
    let broinfo = BroInfo::from_json_str(&s)?;
    let browser = backends::save_broinfo(SaveBroInfoRequest {
        broinfo: broinfo.clone(),
        bicmid,
        user,
        return_browser: true,
    })
    .await?
    .unwrap();
    Ok((broinfo, browser))
}

/// Retrieves or creates an anonymous browser identifier (BICMID) from `localStorage`.
///
/// If it doesn't exist, a new UUID (V4) is generated and stored.
async fn get_or_create_bicmid() -> Result<String> {
    use base64::Engine;

    // check 'localStrage' support
    let v =
        document::eval(r#"{var r=false;if('localStorage' in window){r=true}return r;}"#).await?;
    if !v.as_bool().unwrap_or(false) {
        return Ok("".to_string());
    }
    // get from localStrage
    let js_get: &str = r#"{return window.localStorage.getItem('anon_bicmid');}"#;
    let v = document::eval(js_get).await?;
    let ss = v.as_str().unwrap_or("");
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

/// Server function to retrieve the current database path.
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

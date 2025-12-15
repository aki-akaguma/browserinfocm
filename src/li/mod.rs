use anyhow::Result;
use browserinfo::{broinfo_js, BroInfo, Browser};
use dioxus::prelude::*;

#[cfg(feature = "backend_user_agent")]
use browserinfo::{user_agent_js, UserAgent};

mod backends;

pub async fn get_db_path() -> String {
    backends::get_db_path().await.unwrap()
}

#[derive(Props, Debug, Clone, PartialEq)]
pub struct BrowserInfoProps {
    broinfo: Signal<BroInfo>,
    browser: Signal<Browser>,
}

#[component]
pub fn BrowserInfoCm(mut props: BrowserInfoProps) -> Element {
    use_future(move || async move {
        let (broinfo, browser) = get_browserinfo().await.unwrap();
        props.broinfo.set(broinfo);
        props.browser.set(browser);
    });

    rsx! {}
}

pub async fn get_browserinfo() -> Result<(BroInfo, Browser)> {
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
    let browser = backends::save_broinfo(broinfo.clone(), true)
        .await?
        .unwrap();
    Ok((broinfo, browser))
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

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use dioxus::prelude::*;

#[allow(unused_imports)]
use browserinfo::{BroInfo, Browser, UserAgent};

const NEXT_URL: &str = "http://hcc-desktop.local:8080";
//const NEXT_URL: &str = "http://192.168.116.102:8080";
//const NEXT_URL_API_TAG: &str = "2751944067970052790";
const NEXT_URL_API_TAG: &str = "3771007982502153373";

//#[cfg_attr(not(feature = "desktop"), server(input=cbor, output=cbor))]
#[cfg_attr(not(feature = "desktop"), server(input=json, output=json))]
pub async fn save_user_agent(ua: UserAgent) -> Result<()> {
    use std::time::Duration;

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    struct UserAgentProps {
        pub ua: UserAgent,
    }
    let a_props = UserAgentProps { ua };

    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_millis(1000))
        .build()?;
    let _res = client
        .post(&format!("{NEXT_URL}/api/save_user_agent{NEXT_URL_API_TAG}"))
        .header("x-request-client", "dioxus")
        .timeout(Duration::from_millis(5000))
        .json(&a_props)
        .send()
        .await?;
    //dioxus_logger::tracing::info!("save_user_agent next: {_res:?}");
    Ok(())
}

//#[cfg_attr(not(feature = "desktop"), server(input=cbor, output=cbor))]
#[cfg_attr(not(feature = "desktop"), server(input=json, output=json))]
pub async fn save_broinfo(broinfo: BroInfo, return_browser: bool) -> Result<Option<Browser>> {
    use std::time::Duration;

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    struct BroInfoProps {
        pub broinfo: BroInfo,
        pub return_browser: bool,
    }
    let a_props = BroInfoProps {
        broinfo,
        return_browser,
    };

    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_millis(1000))
        .build()?;

    let resp = client
        .post(&format!("{NEXT_URL}/api/save_broinfo{NEXT_URL_API_TAG}"))
        .header("x-request-client", "dioxus")
        .timeout(Duration::from_millis(5000))
        .json(&a_props)
        .send()
        .await?
        .json::<Option<Browser>>()
        .await?;
    //dioxus_logger::tracing::info!("save_broinfo next: {_res:?}");
    Ok(resp)
}

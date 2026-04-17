use anyhow::Result;
use browserinfo::{BroInfo, Browser};
use dioxus::prelude::*;

#[cfg(feature = "backend_user_agent")]
use browserinfo::UserAgent;

#[cfg(feature = "server")]
use std::sync::LazyLock;

#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use std::time::Duration;

#[cfg(feature = "server")]
use reqwest;

#[cfg(feature = "server")]
pub static NEXT_URL: LazyLock<Result<String, String>> = LazyLock::new(|| {
    // NEXT_URL: "http://aki-desktop.local:8080/"
    std::env::var("NEXT_URL")
        .map(|val| val.trim_end_matches('/').to_string())
        .map_err(|_| {
            "Environment variable 'NEXT_URL' is not set. Please set it to the target backend URL."
                .to_string()
        })
});

#[cfg(feature = "server")]
pub static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .connect_timeout(Duration::from_millis(1000))
        .build()
        .expect("Failed to create reqwest client")
});

#[post("/api/v1/mikan1")]
pub async fn get_db_path() -> Result<String> {
    let base_url = NEXT_URL.as_ref().map_err(|e| anyhow::anyhow!(e.clone()))?;
    let url_s = format!("{}/api/v1/mikan1", base_url);

    let resp = CLIENT
        .post(&url_s)
        .header("x-request-client", "dioxus")
        .timeout(Duration::from_millis(5000))
        .send()
        .await?
        .json::<String>()
        .await?;
    Ok(resp)
}

#[post("/api/v1/ringo1")]
pub async fn get_ipaddr() -> Result<String> {
    let base_url = NEXT_URL.as_ref().map_err(|e| anyhow::anyhow!(e.clone()))?;
    let url_s = format!("{}/api/v1/ringo1", base_url);

    let resp = CLIENT
        .post(&url_s)
        .header("x-request-client", "dioxus")
        .timeout(Duration::from_millis(5000))
        .send()
        .await?
        .json::<String>()
        .await?;
    Ok(resp)
}

#[cfg(feature = "backend_user_agent")]
#[post("/api/v1/useragent1")]
pub async fn save_user_agent(ua: UserAgent) -> Result<()> {
    let base_url = NEXT_URL.as_ref().map_err(|e| anyhow::anyhow!(e.clone()))?;
    let url_s = format!("{}/api/v1/useragent1", base_url);

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    struct UserAgentProps {
        pub ua: UserAgent,
    }
    let a_props = UserAgentProps { ua };

    let _resp = CLIENT
        .post(&url_s)
        .header("x-request-client", "dioxus")
        .timeout(Duration::from_millis(5000))
        .json(&a_props)
        .send()
        .await?;
    //dioxus_logger::tracing::info!("save_user_agent next: {_res:?}");
    Ok(())
}

#[post("/api/v1/browserinfo1")]
pub async fn save_broinfo(
    broinfo: BroInfo,
    bicmid: String,
    user: String,
    return_browser: bool,
) -> Result<Option<Browser>> {
    let base_url = NEXT_URL.as_ref().map_err(|e| anyhow::anyhow!(e.clone()))?;
    let url_s = format!("{}/api/v1/browserinfo1", base_url);

    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    struct BroInfoProps {
        pub broinfo: BroInfo,
        pub bicmid: String,
        pub user: String,
        pub return_browser: bool,
    }
    let a_props = BroInfoProps {
        broinfo,
        bicmid,
        user,
        return_browser,
    };

    let resp = CLIENT
        .post(&url_s)
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

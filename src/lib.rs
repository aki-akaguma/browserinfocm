/*!
This is the dioxus component of the browser information.

### Example

#### The Component

```rust
use dioxus::prelude::*;
use browserinfo::{BroInfo, Browser};
use browserinfocm::{BrowserInfoCm, BrowserInfoState};

#[component]
fn BroInfoHome() -> Element {
    let state_sig = use_signal(BrowserInfoState::default);

    let state = state_sig.read();
    let brg_s = format!("{:?}", state.browser);
    let bim_s = format!("{:?}", state.broinfo);
    let bicmid_s = format!("{:?}", state.bicmid);
    let user_s = format!("{:?}", state.user);

    rsx! {
        BrowserInfoCm { state: state_sig }
        div { "{brg_s}" }
        div {}
        div { "{bim_s}" }
        div {}
        div { "{bicmid_s}" }
        div {}
        div { "{user_s}" }
    }
}
```

#### `Cargo.toml`

```text
[features]
default = []

web = ["dioxus/web", "browserinfocm/web"]
desktop = ["dioxus/desktop", "browserinfocm/desktop"]
mobile = ["dioxus/mobile", "browserinfocm/mobile"]
server = ["dioxus/server", "browserinfocm/server"]
```

### Runtime Environment
+ `BROWSERINFOCM_DB_PATH`: ex.) `/var/local/data/broinfo/browserinfocm.db`
+ `BROWSERINFOCM_DB_BASE_PATH`:  ex.) `/var/local/mydata/broinfo`
+ `BROWSERINFOCM_DB_FILE`: ex.) `browserinfocm.db`
*/
/// Internal module containing components and backends.
mod li;
pub use browserinfo;
pub use li::*;

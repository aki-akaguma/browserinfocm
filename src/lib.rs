/*!
This is the dioxus component of the browser information.

### Example

#### The Component

```rust
use dioxus::prelude::*;
use browserinfo::{BroInfo, Browser};
use browserinfocm::BrowserInfoCm;

#[component]
fn BroInfoHome() -> Element {
    let broinfo_sig = use_signal(BroInfo::default);
    let browser_sig = use_signal(Browser::default);
    let brg = browser_sig.read().clone();
    let bim = broinfo_sig.read().clone();
    let brg_s = format!("{:?}", brg);
    let bim_s = format!("{:?}", bim);
    rsx! {
        BrowserInfoCm { broinfo: broinfo_sig, browser: browser_sig }
        div { "{brg_s}" }
        div {}
        div { "{bim_s}" }
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
mod li;
pub use browserinfo;
pub use li::*;

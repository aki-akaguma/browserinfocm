# browserinfocm

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Rust Version][rustc-image]
![Apache2/MIT licensed][license-image]
[![Test ubu][test-ubuntu-image]][test-ubuntu-link]
[![Test mac][test-windows-image]][test-windows-link]
[![Test win][test-macos-image]][test-macos-link]

This is the dioxus component of the browser information.

#### Example

##### The Component

Call `browserinfocm::backend_init()` from `fn main()` as follows.

```rust
#[cfg(feature = "server")]
browserinfocm::backend_init().expect("faile to init backend");
```

Next, use the `BrowserInfoCm {}` component.

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

##### `Cargo.toml`

```
[features]
default = []

web = ["dioxus/web", "browserinfocm/web"]
desktop = ["dioxus/desktop", "browserinfocm/desktop"]
mobile = ["dioxus/mobile", "browserinfocm/mobile"]
server = ["dioxus/server", "browserinfocm/server"]
```

#### Runtime Environment
+ `BROWSERINFOCM__DATABSE__BASE_PATH`:  ex.) `/var/local/mydata/broinfo`
+ `BROWSERINFOCM__DATABSE__DB_FILE`: ex.) `browserinfocm.db`

## Patches
### dioxus-fullstack (0.7.6)
- **File:** \`patches/dioxus-fullstack+0.7.6.patch\`
- **Reason:** Fixed an issue where URL construction was incorrect when \`get_server_url()\` returned a relative path (starting with \`/\`). This ensures stability in environments where the server URL is dynamically configured, such as mobile or specific deployment setups.


# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/browserinfocm/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/browserinfocm.svg
[crate-link]: https://crates.io/crates/browserinfocm
[docs-image]: https://docs.rs/browserinfocm/badge.svg
[docs-link]: https://docs.rs/browserinfocm/
[rustc-image]: https://img.shields.io/badge/rustc-1.90+-blue.svg
[license-image]: https://img.shields.io/badge/license-Apache2.0/MIT-blue.svg
[test-ubuntu-image]: https://github.com/aki-akaguma/browserinfocm/actions/workflows/test-ubuntu.yml/badge.svg
[test-ubuntu-link]: https://github.com/aki-akaguma/browserinfocm/actions/workflows/test-ubuntu.yml
[test-macos-image]: https://github.com/aki-akaguma/browserinfocm/actions/workflows/test-macos.yml/badge.svg
[test-macos-link]: https://github.com/aki-akaguma/browserinfocm/actions/workflows/test-macos.yml
[test-windows-image]: https://github.com/aki-akaguma/browserinfocm/actions/workflows/test-windows.yml/badge.svg
[test-windows-link]: https://github.com/aki-akaguma/browserinfocm/actions/workflows/test-windows.yml

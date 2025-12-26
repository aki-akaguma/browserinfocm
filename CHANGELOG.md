# Changelog: browserinfocm

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
* the `User` table into database


## [0.1.5] (2025-12-26)
### Added
* `patched` by using `patch-crate`
* the `IpAddress` table into db
* env::var() to backend db path: `BROWSERINFOCM_DB_PATH`
* `brotli` on `backend_next`

### Changed
* depend dioxus to aki's github to fixed a bug of `base_path`

### Fixed
* the missing depend on `backend_next`

## [0.1.4] (2025-12-15)
### Changed
* cleanup backends

### Added
* a `user_id` column into a `Logs` table
* a `ipaddr` column into a `Logs` table
* tests

## [0.1.3] (2025-12-11)
### Added
* `backend_user_agent` to features for cfg

## [0.1.2] (2025-12-08)
### Changed
* splited `component` and backends from broinfo.

## [0.1.1] (2025-12-06)
### Added
* first commit

[Unreleased]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.5..HEAD
[0.1.5]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/browserinfocm/releases/tag/v0.1.0

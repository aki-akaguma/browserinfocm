# Changelog: browserinfocm

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
* migrated from `rusqlite` to `sqlx with sqlite`


## [0.1.10] (2026-01-02)
### Fixed
* invalid `get_or_create_bicmid()` on android

## [0.1.9] (2026-01-01)
### Fixed
* `README.md`

## [0.1.8] (2026-01-01)
### Added
* the `Bicmid` table into databse
* `bicmid` on browser's local strage

### Changed
* `anon_uuid` to `anon_bicmid`
* `Logs` table to `Log` table

### Fixed
* primary key AUTOINCREMENT on create table
* `get_or_store_xxx()` must return -1 if it errors

## [0.1.7] (2025-12-27)
### Changed
* `README.md`

## [0.1.6] (2025-12-27)
### Added
* the `User` table into database

### Fixed
* doc comment: missing user param

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

[Unreleased]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.10..HEAD
[0.1.10]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.9..v0.1.10
[0.1.9]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.8..v0.1.9
[0.1.8]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.7..v0.1.8
[0.1.7]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.6..v0.1.7
[0.1.6]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.5..v0.1.6
[0.1.5]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.4..v0.1.5
[0.1.4]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.3..v0.1.4
[0.1.3]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.2..v0.1.3
[0.1.2]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.1..v0.1.2
[0.1.1]: https://github.com/aki-akaguma/browserinfocm/compare/v0.1.0..v0.1.1
[0.1.0]: https://github.com/aki-akaguma/browserinfocm/releases/tag/v0.1.0

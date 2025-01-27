# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.2](https://github.com/umccr/htsget-rs/compare/htsget-http-v0.4.1...htsget-http-v0.4.2) - 2023-06-08

### Other
- remove unused dependencies and update msrv
- update remaining dependencies, hold back tokio-rustls due to conflicting versions with actix-web

## [0.4.1](https://github.com/umccr/htsget-rs/compare/htsget-http-v0.4.0...htsget-http-v0.4.1) - 2023-06-02

### Other
- updated the following local packages: htsget-config

## [0.3.0](https://github.com/umccr/htsget-rs/compare/htsget-http-v0.2.0...htsget-http-v0.3.0) - 2023-05-29

### Added
- format parsing is now case-insensitive when validating query parameters
- [**breaking**] add request header information to post handlers
- [**breaking**] add request header information to get handlers
- *(config)* add url-storage feature flag
- add error type to config
- *(config)* add automatic config inference for local and s3 storage, and rearrange modules
- [**breaking**] simplify storage config by allowing untagged enum representation

### Fixed
- [**breaking**] headers should allow multiple values for the same key
- *(http)* return InvalidInput when query parameters are present for a post request
- use correct help context for a crate using htsget-config
- *(release)* Bump all crates to 0.1.2 as explored in https://github.com/MarcoIeni/release-plz/issues/507#issuecomment-1422187766

### Other
- update for UrlStorage
- [**breaking**] rename AwsS3Storage to S3Storage in search
- [**breaking**] http refactor, pass request with query
- remove s3-storage as default
- Merge branch 'main' of https://github.com/umccr/htsget-rs into feat/actix-tls
- [**breaking**] rename `HttpTicketFormatter` and remove `UrlFormatter` implementation for it
- [**breaking**] move htsget structs to config, and resolve storage type in config
- manually bump crate versions to 0.1.4
- make htsget-test a regular dependency
- bump crate versions to 0.1.3 manually
- specify htsget-test version
- *(http)* make naming of service info fields consistent
- release
- Downgrade release after fixing cargo publish circular dep issues as discussed in https://github.com/MarcoIeni/release-plz/issues/507#issuecomment-1420254400
- Update cargo files ([#152](https://github.com/umccr/htsget-rs/pull/152))
- *(deps)* bump tokio from 1.24.0 to 1.24.2 ([#151](https://github.com/umccr/htsget-rs/pull/151))
- release ([#148](https://github.com/umccr/htsget-rs/pull/148))
- Set MSRV on all sub-crates ([#146](https://github.com/umccr/htsget-rs/pull/146))
- Better CI ([#98](https://github.com/umccr/htsget-rs/pull/98))

## [0.2.0](https://github.com/umccr/htsget-rs/compare/htsget-http-v0.1.4...htsget-http-v0.2.0) - 2023-04-28

### Added
- *(config)* add automatic config inference for local and s3 storage, and rearrange modules
- [**breaking**] simplify storage config by allowing untagged enum representation

### Other
- [**breaking**] rename `HttpTicketFormatter` and remove `UrlFormatter` implementation for it
- [**breaking**] move htsget structs to config, and resolve storage type in config

## [0.1.1](https://github.com/umccr/htsget-rs/compare/htsget-http-v0.1.0...htsget-http-v0.1.1) - 2023-01-27

### Other
- Set MSRV on all sub-crates (#146)
- Better CI (#98)

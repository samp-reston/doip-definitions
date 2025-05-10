# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.0.7](https://github.com/samp-reston/doip-definitions/compare/v3.0.6...v3.0.7) - 2025-05-10

### Other

- Merge branch 'main' of https://github.com/samp-reston/doip-definitions
- add DoIP message to exposed API

## [3.0.6](https://github.com/samp-reston/doip-definitions/compare/v3.0.5...v3.0.6) - 2025-05-10

### Other

- Merge branch 'main' of https://github.com/samp-reston/doip-definitions
- update to latest pyo3

## [3.0.5](https://github.com/samp-reston/doip-definitions/compare/v3.0.4...v3.0.5) - 2025-05-10

### Fixed

- fix clippy lints

### Other

- add std support and organise module system

## [3.0.4](https://github.com/samp-reston/doip-definitions/compare/v3.0.3...v3.0.4) - 2025-03-04

### Other

- replace lifetime with N buffer size for diagmsg

## [3.0.3](https://github.com/samp-reston/doip-definitions/compare/v3.0.2...v3.0.3) - 2025-03-02

### Other

- remove python build
- chore add clone
- add github test actions

## [3.0.2](https://github.com/samp-reston/doip-definitions/compare/v3.0.1...v3.0.2) - 2025-03-02

### Fixed

- fix html tags
- fix eof

### Other

- removing duplicate 2.0.0 changelog
- add partialeq to message
- add bindings for python dev
- flesh out defaults
- set default templates
- add templates
- remove sphinx
- test sphinx
- add VENV_DIR: "venv"
- spin up venv
- test new job
- test different pip installs
- debug pipeline
- uv venv
- add build-essential
- target uv image
- remove pip install
- add source
- image rust and install uv
- add cargo to path
- remove interactive
- correct spacing
- apt update
- add curl
- add before install cargo and rustup
- add uv iamge
- try before script on pipeline
- local test of pipeline
- add not test to bindings module
- test new pipeline with uv support
- add pyproject details
- remove sample test, reorder pipeline, add support for uv
- add more args to tarpaulin
- add image to lint
- add test module
- remove duplicate coverage
- add more tests lints and coverage report
- change name to pages
- change lib name
- echo html into public index
- change mv target
- update docs pipeline
- add cargo docs stage
- add base rust

## [3.0.0](https://github.com/samp-reston/doip-definitions/compare/v2.0.1...v3.0.0) - 2025-02-26

### Other

- remove all encode decode
- remove
- remove encode decode
- decision to move back to to/from_bytes
- coverage
- clippy fixes
- attempt payload coverage
- add tests for full coverage
- remove old 3.0.0 notes
- allow module name repetitions
- added new consts to reduce lines of code
- auto clippy fixes
- implement from and try_from for all structs
- move DoipPayload trait to base crate

### Removed

- removed bad docs

## [2.0.0](https://github.com/samp-reston/doip-definitions/compare/v1.0.3...v2.0.0) - 2025-02-10

### Other

- rollback version
- add lifetime to message
- release v3.0.0
- change generic into dyn
- release v2.0.0
- update module name from message to payload
- rename doip_message to doip_payload, remove version bytes transitions
- removed all std-based impl, removed all bytes transitioning code to move to codec
- make non-std changes


## [1.0.3](https://github.com/samp-reston/doip-definitions/compare/v1.0.2...v1.0.3) - 2025-01-03

### Other

- add ignore .env files

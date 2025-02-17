# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [3.0.0](https://github.com/samp-reston/doip-definitions/compare/v2.0.1...v3.0.0) - 2025-02-17

### Other

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

## [2.0.0](https://github.com/samp-reston/doip-definitions/compare/v1.0.3...v2.0.0) - 2025-02-09

### Other

- update module name from message to payload
- rename doip_message to doip_payload, remove version bytes transitions
- removed all std-based impl, removed all bytes transitioning code to move to codec
- make non-std changes

## [1.0.3](https://github.com/samp-reston/doip-definitions/compare/v1.0.2...v1.0.3) - 2025-01-03

### Other

- add ignore .env files

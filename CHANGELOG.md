# Changelog

All notable changes to this project will be documented in this file.

## [2.1.1] - 2026-04-09

### Changed
- Removed `always-rotate-key` from the network level. This option is now only available at the peer level (servers and clients).

## [2.1.0] - 2026-04-08

### Added
- New configuration option `always-rotate-key` (boolean, default false) to automatically recreate keypairs on every run. This can be applied per-peer in `[servers.name]` or `[clients.name]`.
- Automatic merging of existing statefiles. Wired now looks for a `<network-name>.statefile` and uses its existing keys by default, making generation stable across runs.

### Changed
- Updated Nix configuration output to remove deprecated `wireguardPeerConfig` and `routeConfig` attributes.
- Upgraded Rust toolchain to latest stable (1.94.0) via `nixpkgs-unstable`.
- Updated and pinned all dependencies in `Cargo.toml`.

### Fixed
- Fixed tests in environments with strict GPG permissions.
- Improved `pass` integration to handle multiline input more reliably.

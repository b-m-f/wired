# Changelog

All notable changes to this project will be documented in this file.

## [2.2.2] - 2026-08-23

### Fixed
- Fixed Nix configuration generator to output space-separated `AllowedIPs` arrays instead of comma-separated ones, making the output syntactically valid in Nix.

## [2.2.1] - 2026-08-23

### Fixed
- Fixed hardcoded CLI version string to match package version
- Improved robustness of cram tests by suppressing noisy Nix gpg outputs

## [2.2.0] - 2026-08-21

### Added
- Feature: pass additional `allowedIPs` for both clients and servers in the configuration file

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

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

### Changed

### Fixed

## [v0.9.0] - 2022-08-19

### Added

### Changed

- Update MSRV to 1.56.1
- Update `nix` lower bound to `0.25.0`

### Fixed

- Expand to block-enclosed `return` in internal error macro

## [v0.8.0] - 2021-09-30

### Changed

- Update `nix` lower bound to `0.23.0`

## [0.7.0] - 2021-08-12

### Added

- Support for register access (including debug registers) on `aarch64`

### Changed

- MSRV is now 1.46.0
- Update `libc` lower bound to `0.2.99`
- Update `nix` lower bound to `0.22.0`

### Fixed

## [0.6.0] - 2021-06-28

### Added

- `Error::TraceeDied` variant, `Error::tracee_died()` method to improve error ergonomics
- x86 `Tracee` methods for reading and writing debug registers

### Changed

- Hide accidentally-`pub` marker field `Tracee._not_send`
- Remove `Error::Restart`, replace only use with `TraceeDied`
- Remove redundant PIDs from `Stop` variants
- Use named fields for `Stop` variants with data
- Remove redundant suffix from some `Stop` variant idents

### Fixed

- Condition `Register`-related definitions, exports on `target_arch`

## [0.5.0] - 2021-06-21

### Added

### Changed

- Update `nix` lower bound to 0.21.0
- Minimum supported `rustc` version is now 1.41.0, via `nix`
- Added `--quiet`, `--tsv` options to `syscalls` example

### Fixed

## [0.4.0] - 2020-11-16

### Added

- Add changelog ([#19](https://github.com/ranweiler/pete/pull/19))

### Changed

- Add context to `Error::Internal` ([#25](https://github.com/ranweiler/pete/pull/25))
- Rename and expand `Error` variants, replace internal panics with `Error::Internal` returns
- `Ptracer::spawn()` now takes a `std::process::Command`, returns `Child` instead of `Tracee` ([#21](https://github.com/ranweiler/pete/pull/21))
- Remove `cmd` module and custom `Command` struct
- Updated `nix` dependency to 0.19.0

### Fixed

- Don't treat seccomp event-stops as internal errors when tracees have non-`Attaching` state ([#25](https://github.com/ranweiler/pete/pull/25))

## [0.3.1] - 2020-11-14

### Fixed

- Handle attach-stops delivered before ptrace-event-stops ([#12](https://github.com/ranweiler/pete/pull/12))

- Register new tracee in `vfork` event handler (from [@travitch](https://github.com/travitch), [#8](https://github.com/ranweiler/pete/pull/8))

## [0.3.0] - 2020-08-13

### Added

- Enable setting `Command` environment ([#6](https://github.com/ranweiler/pete/pull/6))

### Changed

- `Ptracer::spawn()` now accepts a `Command` instead of an arg vector ([#6](https://github.com/ranweiler/pete/pull/6))

### Fixed

- Prevent `Tracee` from auto-deriving `Send` ([`23d7765`](https://github.com/ranweiler/pete/commit/23d77651f4badec449109aa7c02f97e768297bcb))

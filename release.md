# skiagram release history

This is the permanent, human-readable history of every public skiagram release. Entries are kept
newest first and are not removed when a newer version ships. The concise change log remains in
[`CHANGELOG.md`](CHANGELOG.md), while GitHub release pages preserve binaries, installers, and
checksums.

## Version index

| Version | Published (UTC) | Tag commit | Release | Compare |
| --- | --- | --- | --- | --- |
| `v0.1.2` | 2026-07-31 18:16 | [`44f4e0b`](https://github.com/TanvirAnjumApurbo/skiagram/commit/44f4e0b340fed12b4b7e52c401a144b535bc98bb) | [Artifacts and notes](https://github.com/TanvirAnjumApurbo/skiagram/releases/tag/v0.1.2) | [`v0.1.1...v0.1.2`](https://github.com/TanvirAnjumApurbo/skiagram/compare/v0.1.1...v0.1.2) |
| `v0.1.1` | 2026-06-22 05:33 | [`bb3bd8b`](https://github.com/TanvirAnjumApurbo/skiagram/commit/bb3bd8ba49850cf5a2d0d68439188179c183f9b5) | [Artifacts and notes](https://github.com/TanvirAnjumApurbo/skiagram/releases/tag/v0.1.1) | [`v0.1.0...v0.1.1`](https://github.com/TanvirAnjumApurbo/skiagram/compare/v0.1.0...v0.1.1) |
| `v0.1.0` | 2026-06-19 19:40 | [`eeb6414`](https://github.com/TanvirAnjumApurbo/skiagram/commit/eeb6414da76fa782d7c83e742d3d1e4d1f9ec055) | [Artifacts and notes](https://github.com/TanvirAnjumApurbo/skiagram/releases/tag/v0.1.0) | Initial public release |

## v0.1.2 — provider-aware pricing refresh

Released with a pricing snapshot dated 2026-07-31.

### Added

- Standard direct-API pricing for GPT-5.6 Sol, Terra, and Luna; GPT-5.5 and GPT-5.4 families;
  GPT-5.3 Codex; Claude Fable, Opus, Sonnet, and Mythos 5; Gemini 3.6 Flash, Gemini 3.5 Flash and
  Flash-Lite; and Gemini 3.1 Pro Preview.
- Published long-context tiers above 272K known prompt tokens for eligible GPT models and above
  200K for Gemini 3.1 Pro Preview.
- Codex support for `cache_write_input_tokens`, accounted for separately from uncached input and
  cached-read input.
- A detailed [`docs/pricing.md`](docs/pricing.md) reference containing exact model keys, rates,
  source links, thresholds, and snapshot limitations.

### Changed

- Cache-read and cache-write prices became optional per model and provider. Nonzero usage without
  a published, representable rate is reported as unpriced instead of being assigned a guessed
  multiplier.
- LiteLLM refresh imports only explicit cache and long-context prices.
- Model matching gained provider-prefix normalization and strict dated-suffix handling.
- Flamegraph and summary cost calculations now use the same effective pricing tier and missing-rate
  behavior.
- README, pricing documentation, changelog, release notes, and Windows package-manager guidance
  were refreshed for this release.

### Important pricing notes

- Claude Sonnet 5 uses its introductory price through 2026-08-31. Its scheduled 2026-09-01 price
  change requires a new snapshot.
- Package-manager binaries remain offline and use the embedded snapshot. Online refresh is
  available only in a custom build compiled with the `network` feature.
- Models whose billable modality cannot be reconstructed safely from agent logs remain unpriced;
  skiagram does not silently substitute a text-only rate.

### Distribution record

- GitHub published five platform archives, their checksums, source archive, shell and PowerShell
  installers, npm package, Homebrew formula, unified checksum file, and distribution manifest.
- `skiagram` and `skiagram-core` `0.1.2` were published to crates.io; `skiagram` `0.1.2` was
  published to npm and the Homebrew tap.
- Scoop `0.1.2` was published in
  [`scoop-bucket`](https://github.com/TanvirAnjumApurbo/scoop-bucket/commit/4a163bfdc55169e85e450225435582005e4f2f33).
- WinGet `0.1.2` was submitted in
  [`microsoft/winget-pkgs#410600`](https://github.com/microsoft/winget-pkgs/pull/410600). Microsoft
  review was still pending when this entry was recorded.

After publication, [`614b911`](https://github.com/TanvirAnjumApurbo/skiagram/commit/614b911cf9217932c5bc3e493a7b3a4e729aa222)
clarified the explicit-tag WinGet dispatch wording; it did not change release binaries.

## v0.1.1 — Windows npm installation fix

The first maintenance release across the package-manager channels.

### Fixed

- Made npm's Windows ZIP extraction work under a Restricted PowerShell execution policy.
- Corrected README shell and PowerShell installer asset URLs.

### Release infrastructure

- Added an explicit manual-dispatch path for WinGet publication.
- Refined npm and cargo-dist release jobs and bumped both Rust crates to `0.1.1`.
- The WinGet update was accepted through
  [`microsoft/winget-pkgs#401179`](https://github.com/microsoft/winget-pkgs/pull/401179).

## v0.1.0 — first public release

The initial public release established skiagram as a local-first Rust CLI and TUI for profiling AI
coding-agent token usage.

### Included

- Correct, deduplicated token and estimated-cost accounting.
- Adapters for Claude Code, Codex CLI, Gemini CLI, and Copilot CLI.
- Context-window bloat attribution, sub-agent attribution, anomaly detection, and task
  classification.
- Interactive TUI, live-tail watch mode, and flamegraph SVG export.
- Configuration support, optional online pricing refresh, and an offline default build.
- A two-crate workspace: `skiagram-core` for domain logic and `skiagram` for CLI/TUI I/O.

### Distribution foundation

- Established cargo-dist release artifacts for Apple Silicon macOS, Intel macOS, ARM64 Linux, x64
  Linux, and x64 Windows.
- Published platform checksums plus shell and PowerShell installers.
- Wired the initial crates.io and WinGet publication jobs.

## Maintaining this history

For every future public version:

1. Add a new entry at the top of the version index and above the previous detailed release.
2. Record the exact tag, tag commit, UTC publication time, comparison link, and GitHub release link.
3. Preserve previous entries. If a historical correction is necessary, describe the correction and
   link the correcting commit instead of silently rewriting the record.
4. Record meaningful user-facing changes, compatibility or pricing deadlines, and package-channel
   publication details.
5. Keep generated binaries and checksums on the corresponding GitHub release page rather than
   duplicating volatile asset metadata here.


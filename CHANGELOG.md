# Changelog

All notable changes to skiagram are documented here. Versions follow Semantic Versioning.

## [0.1.2] - 2026-07-31

### Added

- Embedded standard direct-API pricing for Claude Fable/Opus/Sonnet/Mythos 5; GPT-5.6
  Sol/Terra/Luna, GPT-5.5/5.4 families, and GPT-5.3 Codex; and Gemini 3.6 Flash,
  3.5 Flash/Flash-Lite, and 3.1 Pro Preview.
- Provider-published long-context pricing above 272K known prompt tokens for eligible GPT models
  and above 200K for Gemini 3.1 Pro Preview.
- Codex parsing for `cache_write_input_tokens`, kept disjoint from uncached and cached-read input.
- A detailed [pricing reference](docs/pricing.md) with source links and snapshot limitations.

### Changed

- Cache prices are optional per provider and billing dimension. Nonzero usage without a published,
  representable rate is now reported as unpriced instead of using a generic multiplier.
- LiteLLM refresh imports explicit cache prices and long-context tiers only; it no longer derives
  Anthropic-style cache multipliers for other providers.
- WinGet release documentation now records the required manual workflow dispatch and Microsoft
  review delay; the workflow no longer defaults to a stale release tag.

### Notes

- Claude Sonnet 5 uses its introductory price through 2026-08-31. Its scheduled 2026-09-01 price
  change requires a refreshed snapshot.
- Package-manager binaries remain fully offline and use the embedded snapshot. Online refresh is
  available only in a custom build compiled with the `network` feature.

## [0.1.1] - 2026-06-22

- First maintenance release across the package-manager channels.

## [0.1.0] - 2026-06-20

- First public release.

[0.1.2]: https://github.com/TanvirAnjumApurbo/skiagram/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/TanvirAnjumApurbo/skiagram/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/TanvirAnjumApurbo/skiagram/releases/tag/v0.1.0

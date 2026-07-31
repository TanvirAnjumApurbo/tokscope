# Pricing reference

Skiagram `v0.1.2` embeds the snapshot below, refreshed **2026-07-31**. Prices are
USD per 1 million tokens and represent each provider's standard, direct API rate:
OpenAI API Standard, Anthropic Claude API global standard, and Gemini Developer
API Paid Standard. They are estimates, not invoices.

The snapshot does not model batch, flex, fast/priority, regional or data-residency
uplifts, cloud-reseller pricing, tool/search charges, or separate image/audio rates.
Those choices require request metadata that local agent logs do not reliably expose.

`-` means that skiagram has no published, representable rate for that token
category. A nonzero count in such a category makes that request **unpriced**;
skiagram never substitutes a generic multiplier. Missing token counts are omitted
from the estimate and reported as an incomplete lower bound. Separately reported
thinking tokens use the output-token rate.

## Embedded rates

### Anthropic

| Model ID | Input | Cache read | Cache write (5m) | Cache write (1h) | Output |
|---|---:|---:|---:|---:|---:|
| `claude-fable-5` | $10 | $1 | $12.50 | $20 | $50 |
| `claude-mythos-5` | $10 | $1 | $12.50 | $20 | $50 |
| `claude-opus-5` | $5 | $0.50 | $6.25 | $10 | $25 |
| `claude-sonnet-5` | $2 | $0.20 | $2.50 | $4 | $10 |
| `claude-sonnet-4-6` | $3 | $0.30 | $3.75 | $6 | $15 |
| `claude-sonnet-4-5` | $3 | $0.30 | $3.75 | $6 | $15 |
| `claude-sonnet-4` | $3 | $0.30 | $3.75 | $6 | $15 |
| `claude-3-7-sonnet` | $3 | $0.30 | $3.75 | $6 | $15 |
| `claude-opus-4-8` | $5 | $0.50 | $6.25 | $10 | $25 |
| `claude-opus-4-7` | $5 | $0.50 | $6.25 | $10 | $25 |
| `claude-opus-4-6` | $5 | $0.50 | $6.25 | $10 | $25 |
| `claude-opus-4-5` | $5 | $0.50 | $6.25 | $10 | $25 |
| `claude-opus-4-1` | $15 | $1.50 | $18.75 | $30 | $75 |
| `claude-opus-4` | $15 | $1.50 | $18.75 | $30 | $75 |
| `claude-haiku-4-5` | $1 | $0.10 | $1.25 | $2 | $5 |
| `claude-3-5-haiku` | $0.80 | $0.08 | $1 | $1.60 | $4 |

Claude Sonnet 5's row is Anthropic's introductory price, valid through
**2026-08-31**. From **2026-09-01**, Anthropic's scheduled rates are $3 input,
$0.30 cache read, $3.75 five-minute cache write, $6 one-hour cache write, and $15
output. The embedded table is not date-aware and will not switch automatically;
a refreshed snapshot or cached override is required. A single flat rate also
cannot exactly price history spanning both sides of that cutoff.

### OpenAI

| Model ID | Input | Cached input | Cache write | Output |
|---|---:|---:|---:|---:|
| `gpt-5.6-sol` | $5 | $0.50 | $6.25 | $30 |
| `gpt-5.6-terra` | $2 | $0.20 | $2.50 | $12 |
| `gpt-5.6-luna` | $0.20 | $0.02 | $0.25 | $1.20 |
| `gpt-5.6` (alias of `gpt-5.6-sol`) | $5 | $0.50 | $6.25 | $30 |
| `gpt-5.5-pro` | $30 | - | - | $180 |
| `gpt-5.5` | $5 | $0.50 | - | $30 |
| `gpt-5.4-pro` | $30 | - | - | $180 |
| `gpt-5.4-mini` | $0.75 | $0.075 | - | $4.50 |
| `gpt-5.4-nano` | $0.20 | $0.02 | - | $1.25 |
| `gpt-5.4` | $2.50 | $0.25 | - | $15 |
| `gpt-5.3-codex` | $1.75 | $0.175 | - | $14 |

For the models below, a known prompt above 272,000 tokens switches the **whole
request** to the long-context rates. Skiagram counts input, cache reads, and
cache writes toward that threshold; exactly 272,000 remains at the base rate.

| Model ID | Long input | Long cached input | Long cache write | Long output |
|---|---:|---:|---:|---:|
| `gpt-5.6-sol` | $10 | $1 | $12.50 | $45 |
| `gpt-5.6-terra` | $4 | $0.40 | $5 | $18 |
| `gpt-5.6-luna` | $0.40 | $0.04 | $0.50 | $1.80 |
| `gpt-5.6` | $10 | $1 | $12.50 | $45 |
| `gpt-5.5-pro` | $60 | - | - | $270 |
| `gpt-5.5` | $10 | $1 | - | $45 |
| `gpt-5.4-pro` | $60 | - | - | $270 |
| `gpt-5.4` | $5 | $0.50 | - | $22.50 |

### Google

| Model ID | Input | Cached input | Cache write/storage | Output |
|---|---:|---:|---:|---:|
| `gemini-3.6-flash` | $1.50 | $0.15 | - | $7.50 |
| `gemini-3.5-flash-lite` | $0.30 | $0.03 | - | $2.50 |
| `gemini-3.5-flash` | $1.50 | $0.15 | - | $9 |
| `gemini-3.1-pro-preview-customtools` | $2 | $0.20 | - | $12 |
| `gemini-3.1-pro-preview` | $2 | $0.20 | - | $12 |

For both Gemini 3.1 Pro Preview IDs, prompts above 200,000 known prompt tokens
switch the whole request to $4 input, $0.40 cached input, and $18 output. Exactly
200,000 remains at the base rate.

Google bills context-cache storage by token-hour: $1 per million tokens per hour
for the listed Flash models and $4.50 for Gemini 3.1 Pro Preview. The agent logs
do not expose the duration needed to calculate that charge, so skiagram records
cached reads but deliberately leaves cache creation/storage unpriced.

`gemini-3-flash-preview` and `gemini-3.1-flash-lite` deliberately remain
unpriced: Google publishes different input/cache rates by modality, while the
Gemini CLI token totals do not identify which tokens were audio. The mutable
`gemini-flash-latest` alias is also not pinned to a price snapshot.

## Model matching

Keys are matched exactly after removing known Anthropic, OpenAI, Google, and
Gemini API wrappers such as `anthropic/`, `openai/`, `google/`, `gemini/`,
`models/`, and `publishers/google/models/`. Exactly eight-digit dated release suffixes such as
`-20250929` and `@20250929` are accepted. A new minor or generation suffix is
not inherited: for example, `claude-opus-5-1` stays unpriced until an explicit
rate exists.

## Embedded snapshot and optional refresh

Package-manager binaries, including WinGet, use the embedded snapshot and contain
no network fetch code. A valid local pricing cache is still read automatically and
overrides matching embedded rows; a missing or malformed cache falls back safely
to the embedded table.

`--refresh-pricing` is available only in a custom build compiled with
`--features network`. It fetches LiteLLM's public pricing table, writes provenance
and per-model overrides to `$SKIAGRAM_PRICING_CACHE` or the platform config
directory's `skiagram/pricing-cache.json`, and then uses that cache on later
offline runs. Fetch failure is non-fatal. Refresh data can override existing
models or add new ones, but it does not add historical effective-date handling.

## Official sources

- [OpenAI API pricing](https://developers.openai.com/api/docs/pricing)
- [Anthropic Claude pricing](https://platform.claude.com/docs/en/about-claude/pricing)
- [Google Gemini Developer API pricing](https://ai.google.dev/gemini-api/docs/pricing)

The optional refresh source is
[LiteLLM's model price table](https://raw.githubusercontent.com/BerriAI/litellm/main/model_prices_and_context_window.json),
which is an aggregator rather than a provider billing source.

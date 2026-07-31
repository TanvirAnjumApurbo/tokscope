//! Model pricing: embedded snapshot, USD per **million** tokens.
//!
//! Sources: the official OpenAI, Anthropic, and Google pricing pages, cross-checked
//! against LiteLLM's `model_prices_and_context_window.json`. The manually curated
//! snapshot was refreshed 2026-07-31. It uses standard direct-API rates; batch,
//! flex/fast/priority, regional, and cloud-reseller uplifts require request metadata
//! the local agent logs do not expose and are therefore not guessed.
//!
//! RULES (CLAUDE.md §8):
//! - cache-read and cache-creation are priced separately (§8.4). Cache rates are
//!   optional because providers expose different billable cache dimensions; a
//!   nonzero usage field with no published rate makes the request unpriced.
//! - models NOT in this table are never guessed at; they surface as "unpriced"
//!   (for example private product aliases or a future generation past this
//!   snapshot — a bare numeric suffix like `claude-opus-5-1` must NOT inherit
//!   `claude-opus-5`'s price).
//! - every cost figure traces to (model, token type, unit price) via this table —
//!   no magic numbers anywhere else (§8.7).

use crate::model::Usage;

/// Alternate rates applied when a request's known prompt tokens exceed a
/// provider-published threshold.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LongContextPricing {
    pub input_threshold: u64,
    pub input: f64,
    pub output: f64,
    pub cache_read: Option<f64>,
    pub cache_write_5m: Option<f64>,
    pub cache_write_1h: Option<f64>,
}

/// USD per 1,000,000 tokens.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModelPricing {
    pub input: f64,
    pub output: f64,
    /// Cached-input/read rate, when the provider publishes one.
    pub cache_read: Option<f64>,
    /// Default/unsplit cache-write rate; for Anthropic this is the 5-minute TTL.
    pub cache_write_5m: Option<f64>,
    /// Anthropic's separately published 1-hour cache-write rate. `None` for
    /// providers whose logs/rate cards do not expose that TTL dimension.
    pub cache_write_1h: Option<f64>,
    /// Higher rates for long prompts, when the provider publishes a threshold.
    pub long_context: Option<LongContextPricing>,
}

/// Rates selected for one request after applying any provider-published
/// long-context threshold. Kept crate-private so analyses that break cost into
/// token-type leaves use exactly the same tier selection as [`cost_usd`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct EffectivePricing {
    pub input: f64,
    pub output: f64,
    pub cache_read: Option<f64>,
    pub cache_write_5m: Option<f64>,
    pub cache_write_1h: Option<f64>,
}

const fn flat(
    input: f64,
    output: f64,
    cache_read: Option<f64>,
    cache_write_5m: Option<f64>,
    cache_write_1h: Option<f64>,
) -> ModelPricing {
    ModelPricing {
        input,
        output,
        cache_read,
        cache_write_5m,
        cache_write_1h,
        long_context: None,
    }
}

const fn tiered(
    input: f64,
    output: f64,
    cache_read: Option<f64>,
    cache_write_5m: Option<f64>,
    cache_write_1h: Option<f64>,
    long_context: LongContextPricing,
) -> ModelPricing {
    ModelPricing {
        input,
        output,
        cache_read,
        cache_write_5m,
        cache_write_1h,
        long_context: Some(long_context),
    }
}

/// Embedded standard-rate pricing snapshot (2026-07-31). Keys are model-id
/// prefixes; a dated
/// release suffix (`-20250929` / `@20250929`) is accepted, a minor-version
/// suffix is not (so `claude-opus-4-8` does NOT silently price as
/// `claude-opus-4`).
pub const SNAPSHOT: &[(&str, ModelPricing)] = &[
    // Anthropic standard API. Sonnet 5 uses the introductory rate effective
    // through 2026-08-31; the scheduled 2026-09-01 rate is documented in
    // docs/pricing.md and requires a refreshed snapshot for future usage.
    (
        "claude-fable-5",
        flat(10.0, 50.0, Some(1.0), Some(12.5), Some(20.0)),
    ),
    (
        "claude-mythos-5",
        flat(10.0, 50.0, Some(1.0), Some(12.5), Some(20.0)),
    ),
    (
        "claude-opus-5",
        flat(5.0, 25.0, Some(0.5), Some(6.25), Some(10.0)),
    ),
    (
        "claude-sonnet-5",
        flat(2.0, 10.0, Some(0.2), Some(2.5), Some(4.0)),
    ),
    (
        "claude-sonnet-4-6",
        flat(3.0, 15.0, Some(0.3), Some(3.75), Some(6.0)),
    ),
    (
        "claude-sonnet-4-5",
        flat(3.0, 15.0, Some(0.3), Some(3.75), Some(6.0)),
    ),
    (
        "claude-sonnet-4",
        flat(3.0, 15.0, Some(0.3), Some(3.75), Some(6.0)),
    ),
    (
        "claude-3-7-sonnet",
        flat(3.0, 15.0, Some(0.3), Some(3.75), Some(6.0)),
    ),
    (
        "claude-opus-4-8",
        flat(5.0, 25.0, Some(0.5), Some(6.25), Some(10.0)),
    ),
    (
        "claude-opus-4-7",
        flat(5.0, 25.0, Some(0.5), Some(6.25), Some(10.0)),
    ),
    (
        "claude-opus-4-6",
        flat(5.0, 25.0, Some(0.5), Some(6.25), Some(10.0)),
    ),
    (
        "claude-opus-4-5",
        flat(5.0, 25.0, Some(0.5), Some(6.25), Some(10.0)),
    ),
    (
        "claude-opus-4-1",
        flat(15.0, 75.0, Some(1.5), Some(18.75), Some(30.0)),
    ),
    (
        "claude-opus-4",
        flat(15.0, 75.0, Some(1.5), Some(18.75), Some(30.0)),
    ),
    (
        "claude-haiku-4-5",
        flat(1.0, 5.0, Some(0.1), Some(1.25), Some(2.0)),
    ),
    (
        "claude-3-5-haiku",
        flat(0.8, 4.0, Some(0.08), Some(1.0), Some(1.6)),
    ),
    // OpenAI standard direct-API pricing. GPT-5.6/5.5/5.4 switch the whole
    // request to long-context rates above 272K known prompt tokens.
    (
        "gpt-5.6-sol",
        tiered(
            5.0,
            30.0,
            Some(0.5),
            Some(6.25),
            None,
            LongContextPricing {
                input_threshold: 272_000,
                input: 10.0,
                output: 45.0,
                cache_read: Some(1.0),
                cache_write_5m: Some(12.5),
                cache_write_1h: None,
            },
        ),
    ),
    (
        "gpt-5.6-terra",
        tiered(
            2.0,
            12.0,
            Some(0.2),
            Some(2.5),
            None,
            LongContextPricing {
                input_threshold: 272_000,
                input: 4.0,
                output: 18.0,
                cache_read: Some(0.4),
                cache_write_5m: Some(5.0),
                cache_write_1h: None,
            },
        ),
    ),
    (
        "gpt-5.6-luna",
        tiered(
            0.2,
            1.2,
            Some(0.02),
            Some(0.25),
            None,
            LongContextPricing {
                input_threshold: 272_000,
                input: 0.4,
                output: 1.8,
                cache_read: Some(0.04),
                cache_write_5m: Some(0.5),
                cache_write_1h: None,
            },
        ),
    ),
    // Official alias: gpt-5.6 routes to gpt-5.6-sol.
    (
        "gpt-5.6",
        tiered(
            5.0,
            30.0,
            Some(0.5),
            Some(6.25),
            None,
            LongContextPricing {
                input_threshold: 272_000,
                input: 10.0,
                output: 45.0,
                cache_read: Some(1.0),
                cache_write_5m: Some(12.5),
                cache_write_1h: None,
            },
        ),
    ),
    (
        "gpt-5.5-pro",
        tiered(
            30.0,
            180.0,
            None,
            None,
            None,
            LongContextPricing {
                input_threshold: 272_000,
                input: 60.0,
                output: 270.0,
                cache_read: None,
                cache_write_5m: None,
                cache_write_1h: None,
            },
        ),
    ),
    (
        "gpt-5.5",
        tiered(
            5.0,
            30.0,
            Some(0.5),
            None,
            None,
            LongContextPricing {
                input_threshold: 272_000,
                input: 10.0,
                output: 45.0,
                cache_read: Some(1.0),
                cache_write_5m: None,
                cache_write_1h: None,
            },
        ),
    ),
    (
        "gpt-5.4-pro",
        tiered(
            30.0,
            180.0,
            None,
            None,
            None,
            LongContextPricing {
                input_threshold: 272_000,
                input: 60.0,
                output: 270.0,
                cache_read: None,
                cache_write_5m: None,
                cache_write_1h: None,
            },
        ),
    ),
    ("gpt-5.4-mini", flat(0.75, 4.5, Some(0.075), None, None)),
    ("gpt-5.4-nano", flat(0.2, 1.25, Some(0.02), None, None)),
    (
        "gpt-5.4",
        tiered(
            2.5,
            15.0,
            Some(0.25),
            None,
            None,
            LongContextPricing {
                input_threshold: 272_000,
                input: 5.0,
                output: 22.5,
                cache_read: Some(0.5),
                cache_write_5m: None,
                cache_write_1h: None,
            },
        ),
    ),
    ("gpt-5.3-codex", flat(1.75, 14.0, Some(0.175), None, None)),
    // Google Gemini Developer API standard text-token rates. Gemini bills cache
    // storage separately by token-hour; the CLI log has no such usage field, so
    // no cache-write/storage rate is invented here.
    ("gemini-3.6-flash", flat(1.5, 7.5, Some(0.15), None, None)),
    (
        "gemini-3.5-flash-lite",
        flat(0.3, 2.5, Some(0.03), None, None),
    ),
    ("gemini-3.5-flash", flat(1.5, 9.0, Some(0.15), None, None)),
    (
        "gemini-3.1-pro-preview-customtools",
        tiered(
            2.0,
            12.0,
            Some(0.2),
            None,
            None,
            LongContextPricing {
                input_threshold: 200_000,
                input: 4.0,
                output: 18.0,
                cache_read: Some(0.4),
                cache_write_5m: None,
                cache_write_1h: None,
            },
        ),
    ),
    (
        "gemini-3.1-pro-preview",
        tiered(
            2.0,
            12.0,
            Some(0.2),
            None,
            None,
            LongContextPricing {
                input_threshold: 200_000,
                input: 4.0,
                output: 18.0,
                cache_read: Some(0.4),
                cache_write_5m: None,
                cache_write_1h: None,
            },
        ),
    ),
];

/// Find the price for a model id, tolerating known provider/API prefixes and
/// dated release suffixes (`-20250929`, `@20250929`).
/// Returns `None` for unknown models — callers must surface that, not guess.
pub fn lookup(model: &str) -> Option<&'static ModelPricing> {
    let m = normalize_model(model);
    SNAPSHOT
        .iter()
        .filter(|(key, _)| key_matches(&m, key))
        .max_by_key(|(key, _)| key.len()) // longest prefix wins (sonnet-4-5 over sonnet-4)
        .map(|(_, p)| p)
}

/// Lower-case and strip provider/API prefixes so matching is provider-agnostic.
/// Shared by [`lookup`] and [`PricingTable`] so embedded and override matching
/// behave identically.
fn normalize_model(model: &str) -> String {
    let mut m = model.trim().to_ascii_lowercase();
    for prefix in [
        "publishers/google/models/",
        "anthropic/",
        "us.anthropic.",
        "anthropic.",
        "openai/",
        "google/",
        "gemini/",
        "models/",
    ] {
        if let Some(rest) = m.strip_prefix(prefix) {
            m = rest.to_string();
            break;
        }
    }
    m
}

/// Exact match, or `<key>-YYYYMMDD` / `<key>@YYYYMMDD`. A bare numeric suffix
/// like `-8` is a DIFFERENT model generation and must not match.
/// TODO(scope): Bedrock-style `...-v1:0` suffixes are not recognized yet.
fn key_matches(model: &str, key: &str) -> bool {
    match model.strip_prefix(key) {
        None => false,
        Some("") => true,
        Some(rest) => {
            (rest.starts_with('-') || rest.starts_with('@'))
                && rest.len() == 9
                && rest[1..].chars().all(|c| c.is_ascii_digit())
        }
    }
}

/// Cost of one request's usage in USD, or `None` when the model is unknown /
/// unpriced. Unknown usage fields contribute nothing (absence ≠ zero — the
/// result is a lower bound, and aggregation reports incompleteness separately).
pub fn cost_usd(model: Option<&str>, usage: &Usage) -> Option<f64> {
    price_usage(lookup(model?)?, usage)
}

/// Select the applicable base or long-context tier for one request.
pub(crate) fn effective_pricing(p: &ModelPricing, usage: &Usage) -> EffectivePricing {
    // Provider long-context thresholds apply to the whole known prompt, including
    // cached reads and writes. Missing fields remain a lower bound (§8.5).
    let prompt_tokens = usage
        .input
        .unwrap_or(0)
        .saturating_add(usage.cache_read.unwrap_or(0))
        .saturating_add(usage.cache_creation.unwrap_or(0));
    let long = p
        .long_context
        .filter(|tier| prompt_tokens > tier.input_threshold);
    EffectivePricing {
        input: long.map_or(p.input, |tier| tier.input),
        output: long.map_or(p.output, |tier| tier.output),
        cache_read: long.map_or(p.cache_read, |tier| tier.cache_read),
        cache_write_5m: long.map_or(p.cache_write_5m, |tier| tier.cache_write_5m),
        cache_write_1h: long.map_or(p.cache_write_1h, |tier| tier.cache_write_1h),
    }
}

fn price_usage(p: &ModelPricing, usage: &Usage) -> Option<f64> {
    let rates = effective_pricing(p, usage);

    // A nonzero token category with no published rate makes the whole request
    // unpriced. Zero/absent usage needs no rate and contributes zero.
    let per_m = |tokens: Option<u64>, rate: Option<f64>| -> Option<f64> {
        match (tokens.unwrap_or(0), rate) {
            (0, _) => Some(0.0),
            (n, Some(r)) => Some(n as f64 * r / 1e6),
            (_, None) => None,
        }
    };

    let mut cost = per_m(usage.input, Some(rates.input))?
        + per_m(usage.output, Some(rates.output))?
        // Thinking tokens bill at the output rate when an agent reports them.
        + per_m(usage.thinking, Some(rates.output))?
        + per_m(usage.cache_read, rates.cache_read)?;

    // Cache writes: use the per-TTL split when reported; otherwise assume the
    // provider's default/unsplit rate (`cache_write_5m`). We never derive a 1h
    // rate for providers that do not publish that dimension.
    cost += match (usage.cache_creation_5m, usage.cache_creation_1h) {
        (None, None) => per_m(usage.cache_creation, rates.cache_write_5m)?,
        (m5, h1) => per_m(m5, rates.cache_write_5m)? + per_m(h1, rates.cache_write_1h)?,
    };
    Some(cost)
}

/// A pricing lookup layering optional runtime `overrides` over the embedded
/// [`SNAPSHOT`]. Pure and owned — no global state (CLAUDE.md §9). The binary builds
/// one (from the `--refresh-pricing` cache / config) and threads it into the
/// analysis passes; [`PricingTable::embedded`] is byte-for-byte identical to the
/// free [`lookup`] / [`cost_usd`], so an empty table never changes a number.
#[derive(Debug, Clone, Default)]
pub struct PricingTable {
    /// `(model-id-prefix, price)`, matched with the same prefix / longest-wins rule
    /// as the snapshot and consulted BEFORE it (so an override wins on a tie).
    overrides: Vec<(String, ModelPricing)>,
}

impl PricingTable {
    /// Snapshot only — no overrides (identical to the free functions).
    pub fn embedded() -> Self {
        Self::default()
    }

    /// Snapshot plus the given overrides. Keys are normalized identically to
    /// lookup input; overrides take precedence over the snapshot.
    pub fn with_overrides(overrides: Vec<(String, ModelPricing)>) -> Self {
        let overrides = overrides
            .into_iter()
            .map(|(k, p)| (normalize_model(&k), p))
            .collect();
        Self { overrides }
    }

    /// Number of override entries (for the refresh report).
    pub fn override_count(&self) -> usize {
        self.overrides.len()
    }

    /// Look up a model: overrides first (longest-prefix wins), else the embedded
    /// snapshot. `None` is still surfaced for unknown models, never guessed.
    pub fn lookup(&self, model: &str) -> Option<&ModelPricing> {
        if !self.overrides.is_empty() {
            let m = normalize_model(model);
            if let Some((_, p)) = self
                .overrides
                .iter()
                .filter(|(key, _)| key_matches(&m, key))
                .max_by_key(|(key, _)| key.len())
            {
                return Some(p);
            }
        }
        lookup(model)
    }

    /// Cost of one request's usage under this table, or `None` when unpriced.
    pub fn cost_usd(&self, model: Option<&str>, usage: &Usage) -> Option<f64> {
        price_usage(self.lookup(model?)?, usage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dated_release_suffixes_match() {
        assert!(lookup("claude-sonnet-4-5-20250929").is_some());
        assert!(lookup("claude-haiku-4-5-20251001").is_some());
        assert!(lookup("anthropic/claude-sonnet-4-5").is_some());
        // Longest prefix wins: 4-5 pricing, not 4.
        assert_eq!(lookup("claude-sonnet-4-5").map(|p| p.input), Some(3.0));
        assert!(lookup("claude-sonnet-4-5-202509290").is_none());
        assert!(lookup("claude-sonnet-4-5-2025092").is_none());
    }

    #[test]
    fn known_provider_prefixes_and_override_keys_normalize() {
        for model in [
            "openai/gpt-5.6-sol",
            "google/gemini-3.6-flash",
            "gemini/gemini-3.6-flash",
            "models/gemini-3.6-flash",
            "publishers/google/models/gemini-3.6-flash",
        ] {
            assert!(lookup(model).is_some(), "provider-qualified id: {model}");
        }

        let custom = flat(0.01, 0.02, None, None, None);
        let table = PricingTable::with_overrides(vec![("openai/gpt-5.6-sol".to_owned(), custom)]);
        assert_eq!(table.lookup("gpt-5.6-sol"), Some(&custom));
    }

    #[test]
    fn unknown_models_are_never_guessed() {
        // A newer generation must NOT silently take an existing generation's price
        // (a bare numeric suffix is a different model, even when the base exists).
        assert!(lookup("claude-opus-4-9").is_none()); // not claude-opus-4
        assert!(lookup("claude-opus-5-0").is_none());
        assert!(lookup("claude-fable-6").is_none());
        assert!(lookup("<synthetic>").is_none());
        assert!(lookup("gpt-yolo").is_none());
        // Mutable aliases and modality-dependent models are not silently pinned
        // to whichever target/rate happens to be current today.
        assert!(lookup("gemini-flash-latest").is_none());
        assert!(lookup("gemini-3-flash-preview").is_none());
        assert!(lookup("gemini-3.1-flash-lite").is_none());
    }

    #[test]
    fn current_models_from_official_snapshot_are_priced() {
        // Exact standard rates from the providers' official tables, refreshed
        // 2026-07-31. These also guard model-generation boundaries.
        assert_eq!(lookup("claude-opus-5").map(|p| p.input), Some(5.0));
        assert_eq!(lookup("claude-sonnet-5").map(|p| p.output), Some(10.0));
        assert_eq!(lookup("claude-fable-5").map(|p| p.output), Some(50.0));
        assert_eq!(lookup("claude-opus-4-8").map(|p| p.input), Some(5.0));
        assert_eq!(lookup("claude-opus-4-7").map(|p| p.input), Some(5.0));
        assert_eq!(lookup("claude-opus-4-6").map(|p| p.input), Some(5.0));
        assert_eq!(lookup("claude-sonnet-4-6").map(|p| p.output), Some(15.0));
        assert_eq!(lookup("claude-mythos-5").map(|p| p.input), Some(10.0));
        assert_eq!(lookup("gpt-5.6-sol").map(|p| p.output), Some(30.0));
        assert_eq!(lookup("gpt-5.6-terra").map(|p| p.input), Some(2.0));
        assert_eq!(lookup("gpt-5.6-luna").map(|p| p.output), Some(1.2));
        assert_eq!(lookup("gpt-5.6").map(|p| p.input), Some(5.0));
        assert_eq!(lookup("gemini-3.6-flash").map(|p| p.output), Some(7.5));
        assert_eq!(lookup("gemini-3.5-flash-lite").map(|p| p.input), Some(0.3));
        assert_eq!(lookup("gemini-3.5-flash").map(|p| p.output), Some(9.0));
        // Cache rates follow Anthropic's multipliers (fable-5: 0.1x/1.25x/2x of $10).
        let fable = lookup("claude-fable-5").unwrap();
        assert_eq!(fable.cache_read, Some(1.0));
        assert_eq!(fable.cache_write_5m, Some(12.50));
        assert_eq!(fable.cache_write_1h, Some(20.0));

        let expected = [
            (
                "claude-fable-5",
                10.0,
                50.0,
                Some(1.0),
                Some(12.5),
                Some(20.0),
            ),
            (
                "claude-mythos-5",
                10.0,
                50.0,
                Some(1.0),
                Some(12.5),
                Some(20.0),
            ),
            (
                "claude-opus-5",
                5.0,
                25.0,
                Some(0.5),
                Some(6.25),
                Some(10.0),
            ),
            (
                "claude-sonnet-5",
                2.0,
                10.0,
                Some(0.2),
                Some(2.5),
                Some(4.0),
            ),
            ("gpt-5.6-sol", 5.0, 30.0, Some(0.5), Some(6.25), None),
            ("gpt-5.6-terra", 2.0, 12.0, Some(0.2), Some(2.5), None),
            ("gpt-5.6-luna", 0.2, 1.2, Some(0.02), Some(0.25), None),
            ("gpt-5.5-pro", 30.0, 180.0, None, None, None),
            ("gpt-5.5", 5.0, 30.0, Some(0.5), None, None),
            ("gpt-5.4-pro", 30.0, 180.0, None, None, None),
            ("gpt-5.4", 2.5, 15.0, Some(0.25), None, None),
            ("gpt-5.4-mini", 0.75, 4.5, Some(0.075), None, None),
            ("gpt-5.4-nano", 0.2, 1.25, Some(0.02), None, None),
            ("gpt-5.3-codex", 1.75, 14.0, Some(0.175), None, None),
            ("gemini-3.6-flash", 1.5, 7.5, Some(0.15), None, None),
            ("gemini-3.5-flash-lite", 0.3, 2.5, Some(0.03), None, None),
            ("gemini-3.5-flash", 1.5, 9.0, Some(0.15), None, None),
            ("gemini-3.1-pro-preview", 2.0, 12.0, Some(0.2), None, None),
            (
                "gemini-3.1-pro-preview-customtools",
                2.0,
                12.0,
                Some(0.2),
                None,
                None,
            ),
        ];
        for (model, input, output, read, write_5m, write_1h) in expected {
            let actual = lookup(model).unwrap_or_else(|| panic!("missing {model}"));
            assert_eq!(actual.input, input, "{model} input");
            assert_eq!(actual.output, output, "{model} output");
            assert_eq!(actual.cache_read, read, "{model} cache read");
            assert_eq!(actual.cache_write_5m, write_5m, "{model} cache write");
            assert_eq!(actual.cache_write_1h, write_1h, "{model} 1h write");
        }
    }

    #[test]
    fn provider_long_context_thresholds_are_applied() {
        let short = Usage {
            input: Some(272_000),
            output: Some(1_000_000),
            ..Usage::default()
        };
        let long = Usage {
            input: Some(272_001),
            output: Some(1_000_000),
            ..Usage::default()
        };
        let short_cost = cost_usd(Some("gpt-5.6-sol"), &short).unwrap();
        let long_cost = cost_usd(Some("gpt-5.6-sol"), &long).unwrap();
        assert!((short_cost - 31.36).abs() < 1e-9, "got {short_cost}");
        assert!((long_cost - 47.72001).abs() < 1e-9, "got {long_cost}");

        let gemini_long = Usage {
            input: Some(200_001),
            output: Some(1_000_000),
            ..Usage::default()
        };
        let gemini_cost = cost_usd(Some("gemini-3.1-pro-preview"), &gemini_long).unwrap();
        assert!((gemini_cost - 18.800004).abs() < 1e-9, "got {gemini_cost}");

        let gemini_boundary = Usage {
            input: Some(199_000),
            cache_read: Some(1_000),
            output: Some(1_000_000),
            ..Usage::default()
        };
        let boundary_cost =
            cost_usd(Some("gemini-3.1-pro-preview-customtools"), &gemini_boundary).unwrap();
        assert!(
            (boundary_cost - 12.3982).abs() < 1e-9,
            "got {boundary_cost}"
        );

        let gemini_crossed_by_cache = Usage {
            input: Some(199_000),
            cache_read: Some(1_001),
            output: Some(1_000_000),
            ..Usage::default()
        };
        let crossed_cost = cost_usd(
            Some("gemini-3.1-pro-preview-customtools"),
            &gemini_crossed_by_cache,
        )
        .unwrap();
        assert!(
            (crossed_cost - 18.7964004).abs() < 1e-9,
            "got {crossed_cost}"
        );
    }

    #[test]
    fn missing_provider_rate_never_becomes_a_guess() {
        let unsupported_gemini_cache_write = Usage {
            cache_creation: Some(1_000),
            ..Usage::default()
        };
        assert_eq!(
            cost_usd(Some("gemini-3.6-flash"), &unsupported_gemini_cache_write),
            None,
            "Gemini cache storage is token-hour billing, not an Anthropic-style write rate"
        );
    }

    #[test]
    fn cache_ttls_price_differently() {
        let split = Usage {
            cache_creation: Some(1_000_000),
            cache_creation_5m: Some(0),
            cache_creation_1h: Some(1_000_000),
            ..Usage::default()
        };
        // 1h write on sonnet-4-5 = $6/M, not the 5m $3.75/M.
        let cost = cost_usd(Some("claude-sonnet-4-5"), &split).unwrap();
        assert!((cost - 6.0).abs() < 1e-9, "got {cost}");

        let unsplit = Usage {
            cache_creation: Some(1_000_000),
            ..Usage::default()
        };
        let cost = cost_usd(Some("claude-sonnet-4-5"), &unsplit).unwrap();
        assert!(
            (cost - 3.75).abs() < 1e-9,
            "unsplit assumes 5m rate, got {cost}"
        );
    }

    #[test]
    fn cost_traces_to_unit_prices() {
        let usage = Usage {
            input: Some(1_000_000),
            output: Some(1_000_000),
            cache_read: Some(1_000_000),
            ..Usage::default()
        };
        let cost = cost_usd(Some("claude-haiku-4-5"), &usage).unwrap();
        assert!((cost - (1.0 + 5.0 + 0.10)).abs() < 1e-9);
        assert_eq!(cost_usd(None, &usage), None);
    }
}

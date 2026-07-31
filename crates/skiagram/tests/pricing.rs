//! End-to-end test that a cached pricing override (what `--refresh-pricing` writes)
//! actually flows through the whole pipeline and changes the cost numbers. The
//! public GPT-5.6 fixture is embedded-priced; a private Codex alias remains
//! unpriced until the synthetic cache supplies an explicit override. Fully
//! offline — `$SKIAGRAM_PRICING_CACHE` never touches the network.

use std::path::{Path, PathBuf};

use assert_cmd::Command;
use serde_json::Value;

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures")
}

/// Run `summary --json --agent codex` against the codex fixtures, optionally with
/// the synthetic price cache applied. The baseline forces a non-existent cache path
/// so no ambient `~/.config` cache can perturb it.
fn codex_summary(cache: Option<&str>) -> Value {
    let mut cmd = Command::cargo_bin("skiagram").expect("binary builds");
    cmd.env("CODEX_HOME", fixtures().join("codex"));
    match cache {
        Some(rel) => cmd.env("SKIAGRAM_PRICING_CACHE", fixtures().join(rel)),
        None => cmd.env(
            "SKIAGRAM_PRICING_CACHE",
            fixtures().join("pricing/__none__.json"),
        ),
    };
    let out = cmd
        .args(["summary", "--json", "--agent", "codex"])
        .output()
        .expect("runs");
    assert!(out.status.success(), "summary should succeed");
    serde_json::from_slice(&out.stdout).expect("valid JSON")
}

#[test]
fn cached_override_replaces_embedded_rates_and_prices_a_private_alias() {
    // Baseline: official GPT-5.6 is priced, while the private product alias has
    // no published API rate and stays visibly unpriced.
    let base = codex_summary(None);
    let base_cost = base["totals"]["cost_usd"].as_f64().expect("float");
    assert!(base_cost > 0.0, "public GPT-5.6 uses embedded pricing");
    assert_eq!(base["totals"]["unpriced_requests"], 1);
    assert!(
        !base["unpriced_models"].as_array().unwrap().is_empty(),
        "baseline surfaces the private alias"
    );

    // The cache overrides GPT-5.6's embedded rates and adds the private alias.
    let priced = codex_summary(Some("pricing/litellm-cache.json"));
    let override_cost = priced["totals"]["cost_usd"].as_f64().expect("float");
    assert!(
        (override_cost - base_cost).abs() > 1e-9,
        "the override must replace embedded rates end-to-end"
    );
    assert_eq!(
        priced["totals"]["unpriced_requests"].as_u64().expect("int"),
        0,
        "every gpt request is now priced by the override"
    );
    assert!(
        priced["unpriced_models"].as_array().unwrap().is_empty(),
        "no models remain unpriced once the cache supplies gpt prices"
    );
}

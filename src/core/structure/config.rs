//! Runtime configuration for adaptive structure change.
//!
//! This module defines *when* Lutufi is permitted to change a model's structure
//! on its own. It deliberately contains policy only — no detection, no scoring,
//! no mutation — so that the rules governing automatic change can be read,
//! reviewed, and serialized independently of the machinery that acts on them.
//!
//! # The governing asymmetry
//!
//! Adding a node that does not really exist inflates a model but leaves existing
//! inference intact — the spurious latent variable is marginalized out. Removing
//! a node that does matter destroys information the researcher specified, and the
//! damage is silent: the model still runs, still returns numbers, and those
//! numbers are wrong.
//!
//! Addition is therefore permitted on a confidence threshold alone. Removal is
//! permitted only when the researcher has explicitly enabled it, and even then it
//! is hedged with dwell time, hysteresis, a change budget, protected classes, and
//! an archive that makes every removal reversible.

use serde::{Deserialize, Serialize};

/// When the structure monitor is allowed to look for structural change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScanTrigger {
    /// Never scan. Structure changes only through the explicit model API.
    Never,
    /// Scan after each `fit()`.
    AfterFit,
    /// Scan after each `fit()` and each `query()`.
    ///
    /// Scanning after queries is what makes the system *adaptive at runtime*
    /// rather than only at training time, but the residual-dependence and
    /// posterior-irrelevance detectors are not free — expect a measurable
    /// per-query cost on large models.
    AfterFitAndQuery,
    /// Scan only when `scan()` is called explicitly.
    Manual,
}

/// Policy for introducing latent nodes the data implies but the model lacks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiscoveryPolicy {
    /// Do not look for hidden nodes.
    Disabled,
    /// Detect and report, but never modify the model.
    ///
    /// Useful on its own: "FCI found three probable latent confounders, here is
    /// the evidence" is a result, even with no structural change.
    ProposeOnly,
    /// Materialize a latent node automatically once the evidence clears the bar.
    Automatic {
        /// Minimum confidence in the proposal, in `[0, 1]`.
        ///
        /// The ratified default is 0.95. Lowering it is legitimate for
        /// exploratory work but should be a deliberate act — below roughly 0.8
        /// the model tends to accumulate latent variables that reflect sampling
        /// noise rather than structure.
        confidence_threshold: f64,
        /// Minimum BIC improvement required before the node is kept.
        ///
        /// Confidence says a hidden common cause is likely; BIC says the fitted
        /// node actually earns its parameters. Both must hold — confidence alone
        /// would admit nodes that explain nothing.
        min_bic_improvement: f64,
        /// Cap on nodes materialized in a single scan.
        ///
        /// A systematically misspecified model can produce many simultaneous
        /// proposals; adding them all at once compounds the misspecification
        /// instead of letting the first correction inform the next scan.
        max_additions_per_scan: usize,
    },
}

/// Policy for removing nodes the data no longer supports.
///
/// The `Automatic` variant cannot be constructed implicitly — see
/// [`RemovalPolicy::automatic`] — because enabling it *is* the researcher's
/// consent to structural deletion, given once as policy rather than per node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemovalPolicy {
    /// Never remove nodes, and do not spend time looking. **Default.**
    Disabled,
    /// Detect superfluous nodes and report them; never remove.
    ProposeOnly,
    /// Remove nodes automatically once every safeguard is satisfied.
    Automatic(AutomaticRemoval),
}

/// Safeguards governing automatic node removal.
///
/// Every field here exists because of a specific way naive automatic removal
/// fails. None of them is decorative.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutomaticRemoval {
    /// Maximum change any other variable's marginal may show when this node is
    /// removed, for the node to count as having no effect.
    ///
    /// Measured as the largest absolute difference in any marginal probability.
    pub max_marginal_impact: f64,

    /// Minimum confidence that the node is genuinely superfluous, in `[0, 1]`.
    pub confidence_threshold: f64,

    /// How many consecutive scans a node must stay below threshold before it is
    /// removed.
    ///
    /// **Dwell time.** A node's measured effect depends on current evidence: a
    /// mediator that is presently blocked looks inert, and would be deleted on a
    /// single observation — after which changing the evidence leaves the model
    /// permanently wrong. Requiring persistence across scans means a node must
    /// look irrelevant under varied conditions, not one lucky snapshot.
    pub required_consecutive_scans: usize,

    /// Multiplier making re-addition harder than removal was.
    ///
    /// **Hysteresis.** Removal changes the graph, which can make the resulting
    /// gap look like a hidden common cause — so discovery re-adds a latent node
    /// where one was just deleted, whose effect is then small, so it is removed
    /// again. Requiring re-addition to clear a strictly higher bar than removal
    /// did breaks that limit cycle, exactly as it does in a Schmitt trigger.
    /// Must be `>= 1.0`; 1.0 disables hysteresis and permits oscillation.
    pub readdition_hysteresis: f64,

    /// Cap on removals in a single scan.
    ///
    /// Effects are measured against the *current* graph. Removing several nodes
    /// at once means every measurement after the first was taken against a graph
    /// that no longer exists.
    pub max_removals_per_scan: usize,

    /// Keep removed nodes and their CPDs so removal can be undone.
    ///
    /// Strongly recommended. With archiving off, an incorrect automatic removal
    /// is unrecoverable without re-fitting from the original data.
    pub archive_removed: bool,

    /// Never remove a node that is currently observed.
    pub protect_evidence_nodes: bool,

    /// Never remove a node that has appeared as a query target.
    pub protect_query_targets: bool,

    /// Never remove a node marked causal.
    ///
    /// A causally-marked node encodes a mechanism the researcher asserted from
    /// domain knowledge. Weak statistical effect is not grounds for the library
    /// to overrule that.
    pub protect_causal_nodes: bool,
}

impl RemovalPolicy {
    /// Enable automatic removal with the recommended safeguards.
    ///
    /// This constructor is the point at which the researcher takes on the risk of
    /// automatic structural deletion. It is deliberately not `Default`.
    pub fn automatic() -> Self {
        RemovalPolicy::Automatic(AutomaticRemoval {
            max_marginal_impact: 0.001,
            confidence_threshold: 0.99,
            required_consecutive_scans: 3,
            readdition_hysteresis: 1.5,
            max_removals_per_scan: 1,
            archive_removed: true,
            protect_evidence_nodes: true,
            protect_query_targets: true,
            protect_causal_nodes: true,
        })
    }

    /// Automatic removal with every safeguard stood down.
    ///
    /// For studying the dynamics of adaptive structure itself — how models drift,
    /// whether add/remove cycles form, how sensitive the outcome is to threshold
    /// choice. Those are legitimate questions, and answering them requires being
    /// able to run the system without the guards that would suppress the very
    /// behaviour under study.
    ///
    /// Not for producing results about a domain. With no dwell time the model
    /// reacts to single observations, with no hysteresis it can oscillate
    /// indefinitely, with nothing protected it may delete evidence and query
    /// nodes, and with no archive none of it can be undone. Every one of these is
    /// a phenomenon worth studying and a defect worth avoiding, depending on what
    /// you are doing.
    ///
    /// [`AdaptiveStructureConfig::validate`] returns warnings for this
    /// configuration rather than rejecting it.
    pub fn unrestricted() -> Self {
        RemovalPolicy::Automatic(AutomaticRemoval {
            max_marginal_impact: 0.01,
            confidence_threshold: 0.5,
            required_consecutive_scans: 1,
            readdition_hysteresis: 1.0,
            max_removals_per_scan: usize::MAX,
            archive_removed: false,
            protect_evidence_nodes: false,
            protect_query_targets: false,
            protect_causal_nodes: false,
        })
    }
}

/// Complete runtime policy for adaptive structure change.
///
/// ```ignore
/// // Default: nothing changes on its own.
/// let config = AdaptiveStructureConfig::default();
///
/// // Recommended starting point: discover automatically, report removals.
/// let config = AdaptiveStructureConfig::adaptive();
///
/// // Fully automatic in both directions, with safeguards.
/// let config = AdaptiveStructureConfig::adaptive()
///     .with_removal(RemovalPolicy::automatic());
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdaptiveStructureConfig {
    /// When to scan for structural signals.
    pub scan_trigger: ScanTrigger,
    /// Policy for adding latent nodes.
    pub discovery: DiscoveryPolicy,
    /// Policy for removing superfluous nodes.
    pub removal: RemovalPolicy,
    /// Nodes that must never be removed or rewired, whatever the evidence says.
    pub pinned_nodes: Vec<String>,
}

impl Default for AdaptiveStructureConfig {
    /// Nothing happens automatically.
    ///
    /// A library that silently restructures a researcher's model by default
    /// would be indefensible; adaptivity is something you turn on.
    fn default() -> Self {
        AdaptiveStructureConfig {
            scan_trigger: ScanTrigger::Never,
            discovery: DiscoveryPolicy::Disabled,
            removal: RemovalPolicy::Disabled,
            pinned_nodes: Vec::new(),
        }
    }
}

impl AdaptiveStructureConfig {
    /// Detection only: report both additions and removals, change nothing.
    ///
    /// The right first setting for an unfamiliar dataset — it shows what the
    /// automatic policies *would* have done, at no risk.
    pub fn observe_only() -> Self {
        AdaptiveStructureConfig {
            scan_trigger: ScanTrigger::AfterFit,
            discovery: DiscoveryPolicy::ProposeOnly,
            removal: RemovalPolicy::ProposeOnly,
            pinned_nodes: Vec::new(),
        }
    }

    /// Automatic discovery at the ratified thresholds; removals reported only.
    pub fn adaptive() -> Self {
        AdaptiveStructureConfig {
            scan_trigger: ScanTrigger::AfterFitAndQuery,
            discovery: DiscoveryPolicy::Automatic {
                confidence_threshold: 0.95,
                min_bic_improvement: 10.0,
                max_additions_per_scan: 1,
            },
            removal: RemovalPolicy::ProposeOnly,
            pinned_nodes: Vec::new(),
        }
    }

    /// Fully autonomous in both directions, with safeguards retained.
    ///
    /// The living graph: nodes appear when the evidence supports them and retire
    /// when it stops. Dwell time, hysteresis, archiving, and protected classes
    /// stay on, so structure settles rather than churns.
    pub fn fully_adaptive() -> Self {
        Self::adaptive().with_removal(RemovalPolicy::automatic())
    }

    /// Fully autonomous with every safeguard stood down.
    ///
    /// For studying adaptive structure itself rather than a domain — see
    /// [`RemovalPolicy::unrestricted`]. [`AdaptiveStructureConfig::validate`]
    /// returns warnings for this mode; it does not refuse it.
    pub fn unrestricted() -> Self {
        AdaptiveStructureConfig {
            scan_trigger: ScanTrigger::AfterFitAndQuery,
            discovery: DiscoveryPolicy::Automatic {
                confidence_threshold: 0.5,
                min_bic_improvement: 0.0,
                max_additions_per_scan: usize::MAX,
            },
            removal: RemovalPolicy::unrestricted(),
            pinned_nodes: Vec::new(),
        }
    }

    /// Replace the removal policy.
    pub fn with_removal(mut self, removal: RemovalPolicy) -> Self {
        self.removal = removal;
        self
    }

    /// Replace the discovery policy.
    pub fn with_discovery(mut self, discovery: DiscoveryPolicy) -> Self {
        self.discovery = discovery;
        self
    }

    /// Replace the scan trigger.
    pub fn with_scan_trigger(mut self, trigger: ScanTrigger) -> Self {
        self.scan_trigger = trigger;
        self
    }

    /// Pin a node so it is never automatically removed.
    pub fn pin_node(mut self, node: impl Into<String>) -> Self {
        self.pinned_nodes.push(node.into());
        self
    }

    /// Whether this configuration can modify a model without further consent.
    pub fn can_modify_automatically(&self) -> bool {
        matches!(self.discovery, DiscoveryPolicy::Automatic { .. })
            || matches!(self.removal, RemovalPolicy::Automatic(_))
    }

    /// Check the configuration, returning any warnings it earns.
    ///
    /// The split matters. An **error** means the configuration has no coherent
    /// meaning — a probability outside `[0, 1]`, a budget of zero that would
    /// disable a policy while claiming to enable it. There is no experiment those
    /// settings support, so they are refused.
    ///
    /// A **warning** means the configuration is coherent but will behave in ways
    /// worth knowing about: it can oscillate, it can react to a single
    /// observation, it can delete nodes irreversibly. Those are legitimate things
    /// to want — studying structural drift requires being able to produce it — so
    /// they are permitted and reported, not blocked.
    ///
    /// The library's job here is to make sure nobody arrives at those settings by
    /// accident, not to decide which experiments are worth running.
    pub fn validate(&self) -> Result<Vec<ConfigWarning>, ConfigError> {
        let mut warnings = Vec::new();

        if let DiscoveryPolicy::Automatic {
            confidence_threshold, min_bic_improvement, max_additions_per_scan,
        } = &self.discovery
        {
            if !(0.0..=1.0).contains(confidence_threshold) || confidence_threshold.is_nan() {
                return Err(ConfigError::ThresholdOutOfRange {
                    field: "discovery.confidence_threshold",
                    value: *confidence_threshold,
                });
            }
            if *min_bic_improvement < 0.0 {
                return Err(ConfigError::NegativeBicImprovement { value: *min_bic_improvement });
            }
            if *max_additions_per_scan == 0 {
                return Err(ConfigError::ZeroBudget { field: "discovery.max_additions_per_scan" });
            }
            if *confidence_threshold < 0.8 {
                warnings.push(ConfigWarning::PermissiveDiscovery {
                    threshold: *confidence_threshold,
                });
            }
        }

        if let RemovalPolicy::Automatic(policy) = &self.removal {
            if !(0.0..=1.0).contains(&policy.confidence_threshold)
                || policy.confidence_threshold.is_nan()
            {
                return Err(ConfigError::ThresholdOutOfRange {
                    field: "removal.confidence_threshold",
                    value: policy.confidence_threshold,
                });
            }
            if policy.max_marginal_impact < 0.0 || policy.max_marginal_impact.is_nan() {
                return Err(ConfigError::ThresholdOutOfRange {
                    field: "removal.max_marginal_impact",
                    value: policy.max_marginal_impact,
                });
            }
            if policy.readdition_hysteresis.is_nan() || policy.readdition_hysteresis <= 0.0 {
                return Err(ConfigError::ThresholdOutOfRange {
                    field: "removal.readdition_hysteresis",
                    value: policy.readdition_hysteresis,
                });
            }
            if policy.max_removals_per_scan == 0 {
                return Err(ConfigError::ZeroBudget { field: "removal.max_removals_per_scan" });
            }

            if policy.required_consecutive_scans <= 1 {
                warnings.push(ConfigWarning::NoDwellTime);
            }
            if policy.readdition_hysteresis <= 1.0 {
                warnings.push(ConfigWarning::OscillationPossible {
                    hysteresis: policy.readdition_hysteresis,
                });
            }
            if !policy.archive_removed {
                warnings.push(ConfigWarning::RemovalIsIrreversible);
            }
            if !policy.protect_causal_nodes {
                warnings.push(ConfigWarning::CausalNodesUnprotected);
            }
            if !policy.protect_evidence_nodes || !policy.protect_query_targets {
                warnings.push(ConfigWarning::ActiveNodesUnprotected);
            }
        }

        Ok(warnings)
    }
}

/// A configuration that is coherent but will behave in a way worth knowing about.
///
/// Warnings never block a run. They exist so that a researcher who deliberately
/// chose an aggressive mode is not surprised by it, and so that one who arrived
/// there by accident finds out.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ConfigWarning {
    /// Discovery threshold low enough to admit noise-driven latent variables.
    #[error("discovery.confidence_threshold is {threshold}, below the 0.8 point \
             where proposals start reflecting sampling noise as often as structure; \
             expect latent nodes that do not replicate")]
    PermissiveDiscovery {
        /// The configured threshold.
        threshold: f64,
    },

    /// Nodes may be removed on a single observation.
    #[error("removal acts on a single scan; a mediator that is merely blocked by \
             the current evidence looks inert and will be deleted, after which \
             changing the evidence leaves the model wrong")]
    NoDwellTime,

    /// Add/remove limit cycles are possible.
    #[error("readdition_hysteresis is {hysteresis} (<= 1.0), so a removed node is \
             no harder to re-add than it was to remove; the structure may \
             oscillate indefinitely instead of settling")]
    OscillationPossible {
        /// The configured hysteresis multiplier.
        hysteresis: f64,
    },

    /// Removed nodes are not archived.
    #[error("archive_removed is off, so an incorrect removal cannot be undone \
             without re-fitting from the original data")]
    RemovalIsIrreversible,

    /// Causally-marked nodes may be deleted.
    #[error("protect_causal_nodes is off, so a node the researcher asserted as a \
             causal mechanism may be deleted for having weak statistical effect")]
    CausalNodesUnprotected,

    /// Evidence or query nodes may be deleted.
    #[error("evidence or query nodes are unprotected and may be removed mid-analysis")]
    ActiveNodesUnprotected,
}

/// A configuration that would produce unsafe or incoherent structural behaviour.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ConfigError {
    /// A probability-valued threshold fell outside its valid range.
    #[error("{field} must be in [0, 1], got {value}")]
    ThresholdOutOfRange {
        /// Configuration field at fault.
        field: &'static str,
        /// The offending value.
        value: f64,
    },

    /// A negative BIC improvement requirement would accept any node at all.
    #[error("min_bic_improvement must be non-negative, got {value}; \
             a negative requirement accepts latent nodes that worsen model fit")]
    NegativeBicImprovement {
        /// The offending value.
        value: f64,
    },

    /// A per-scan budget of zero silently disables the policy it belongs to.
    #[error("{field} is 0, which disables the policy without saying so; \
             use the Disabled variant to mean that explicitly")]
    ZeroBudget {
        /// Configuration field at fault.
        field: &'static str,
    },

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_changes_nothing() {
        let config = AdaptiveStructureConfig::default();
        assert!(!config.can_modify_automatically());
        assert_eq!(config.scan_trigger, ScanTrigger::Never);
        config.validate().unwrap();
    }

    #[test]
    fn observe_only_never_modifies() {
        let config = AdaptiveStructureConfig::observe_only();
        assert!(!config.can_modify_automatically());
        config.validate().unwrap();
    }

    #[test]
    fn adaptive_adds_but_does_not_remove() {
        let config = AdaptiveStructureConfig::adaptive();
        assert!(config.can_modify_automatically());
        assert!(matches!(config.removal, RemovalPolicy::ProposeOnly));
        config.validate().unwrap();
    }

    #[test]
    fn automatic_removal_is_opt_in_with_safeguards() {
        let config = AdaptiveStructureConfig::adaptive()
            .with_removal(RemovalPolicy::automatic());
        config.validate().unwrap();

        let RemovalPolicy::Automatic(policy) = &config.removal else {
            panic!("expected automatic removal");
        };
        assert!(policy.archive_removed, "removal must be reversible by default");
        assert!(policy.required_consecutive_scans >= 1, "dwell time required");
        assert!(policy.readdition_hysteresis > 1.0, "hysteresis required");
        assert!(policy.protect_causal_nodes);
    }

    #[test]
    fn fully_adaptive_removes_automatically_and_validates_clean() {
        let config = AdaptiveStructureConfig::fully_adaptive();
        assert!(config.can_modify_automatically());
        assert!(matches!(config.removal, RemovalPolicy::Automatic(_)));
        assert!(
            config.validate().unwrap().is_empty(),
            "recommended safeguards should produce no warnings"
        );
    }

    /// Aggressive settings are permitted. Studying structural drift requires
    /// being able to produce it; the library reports rather than refuses.
    #[test]
    fn unrestricted_is_allowed_and_warns_rather_than_erroring() {
        let config = AdaptiveStructureConfig::unrestricted();
        let warnings = config.validate().expect("unrestricted mode must be permitted");

        assert!(warnings.contains(&ConfigWarning::NoDwellTime));
        assert!(warnings.contains(&ConfigWarning::RemovalIsIrreversible));
        assert!(warnings.contains(&ConfigWarning::CausalNodesUnprotected));
        assert!(warnings.contains(&ConfigWarning::ActiveNodesUnprotected));
        assert!(warnings
            .iter()
            .any(|w| matches!(w, ConfigWarning::OscillationPossible { .. })));
        assert!(warnings
            .iter()
            .any(|w| matches!(w, ConfigWarning::PermissiveDiscovery { .. })));
    }

    /// Incoherent settings, by contrast, are refused: no experiment needs a
    /// confidence above 1.
    #[test]
    fn rejects_out_of_range_confidence() {
        let config = AdaptiveStructureConfig::default().with_discovery(
            DiscoveryPolicy::Automatic {
                confidence_threshold: 1.4,
                min_bic_improvement: 10.0,
                max_additions_per_scan: 1,
            },
        );
        assert!(matches!(
            config.validate(),
            Err(ConfigError::ThresholdOutOfRange { .. })
        ));
    }

    /// A zero budget would disable a policy while claiming to enable it.
    #[test]
    fn rejects_zero_budget_as_incoherent() {
        let config = AdaptiveStructureConfig::default().with_discovery(
            DiscoveryPolicy::Automatic {
                confidence_threshold: 0.95,
                min_bic_improvement: 10.0,
                max_additions_per_scan: 0,
            },
        );
        assert!(matches!(config.validate(), Err(ConfigError::ZeroBudget { .. })));
    }

    #[test]
    fn config_round_trips_through_json() {
        let config = AdaptiveStructureConfig::adaptive()
            .with_removal(RemovalPolicy::automatic())
            .pin_node("Treatment");
        let json = serde_json::to_string(&config).unwrap();
        let restored: AdaptiveStructureConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, restored);
    }
}

//! Adaptive structure: automatic discovery and retirement of network nodes.
//!
//! A model's structure is normally fixed when the researcher builds it. This
//! module lets a model instead *evolve* as evidence accumulates: latent nodes
//! that the data implies but the specification omits can be introduced, and nodes
//! the data no longer supports can be retired.
//!
//! # Status
//!
//! **Policy only.** [`config`] is complete and enforced. The `StructureMonitor`
//! trait and the proposal types that will drive detection and materialization
//! are not yet written. Nothing in this module modifies a model.
//!
//! Landing the interface first is deliberate — it is the same discipline that
//! produced [`ComputeBackend`](crate::core::backend::ComputeBackend) before the
//! GPU backend existed, and it means the safety rules governing automatic change
//! are reviewable before any code can act on them.
//!
//! # The two mechanisms
//!
//! **Discovery.** When two observed variables remain dependent after conditioning
//! on everything between them, an unobserved common cause is the most
//! parsimonious explanation. Lutufi already computes the signals — FCI produces
//! bidirected edges, and the ID algorithm detects hedge structures — but
//! currently discards them. Discovery converts those into proposals, fits the
//! latent variable's parameters, and materializes it when the evidence is strong
//! enough.
//!
//! **Retirement.** A node whose presence changes no other variable's marginal is
//! contributing nothing. Retiring it simplifies the model.
//!
//! # Why the two are not treated alike
//!
//! Discovery errors are recoverable: a spurious latent node inflates the model,
//! but marginalizing it out leaves existing inference correct. Retirement errors
//! are not: deleting a node that mattered destroys information the researcher
//! specified, and does so silently — the model still runs and still returns
//! numbers, and those numbers are simply wrong.
//!
//! So discovery is permitted on a confidence threshold. Retirement additionally
//! requires the researcher to enable it explicitly, and is then constrained by
//! dwell time, hysteresis, a per-scan budget, protected node classes, and an
//! archive that makes it reversible. See
//! [`AutomaticRemoval`](crate::core::structure::config::AutomaticRemoval), and
//! [`RemovalPolicy::unrestricted`](crate::core::structure::config::RemovalPolicy::unrestricted)
//! for the mode that stands those safeguards down deliberately.

/// Runtime policy governing automatic structural change.
pub mod config;

pub use config::{
    ConfigWarning,
    AdaptiveStructureConfig, AutomaticRemoval, ConfigError, DiscoveryPolicy, RemovalPolicy,
    ScanTrigger,
};

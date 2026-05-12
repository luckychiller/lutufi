//! Non-stationary Dynamic Bayesian Network models.
//!
//! This module provides DBN models where the transition dynamics
//! can change over time, supporting:
//! - `transition_model`: Time-varying, regime-switching, and fixed transitions
//! - `dbn`: The NonStationaryDBN type
//! - `simulation`: Forward simulation of trajectories
//! - `regime`: Regime posterior computation and analysis

/// Time-varying, regime-switching, and fixed transition model abstractions.
pub mod transition_model;
/// Non-stationary Dynamic Bayesian Network with changing transition dynamics.
pub mod dbn;
/// Forward simulation of trajectories from non-stationary DBNs.
pub mod simulation;
/// Regime state posterior computation and forecasting for regime-switching models.
pub mod regime;

pub use transition_model::TransitionModel;
pub use dbn::NonStationaryDBN;
pub use simulation::trajectory_log_likelihood;
pub use regime::{regime_posterior, regime_forecast};

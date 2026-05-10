//! Non-stationary Dynamic Bayesian Network models.
//!
//! This module provides DBN models where the transition dynamics
//! can change over time, supporting:
//! - `transition_model`: Time-varying, regime-switching, and fixed transitions
//! - `dbn`: The NonStationaryDBN type
//! - `simulation`: Forward simulation of trajectories
//! - `regime`: Regime posterior computation and analysis

pub mod transition_model;
pub mod dbn;
pub mod simulation;
pub mod regime;

pub use transition_model::TransitionModel;
pub use dbn::NonStationaryDBN;
pub use simulation::trajectory_log_likelihood;
pub use regime::{regime_posterior, regime_forecast};

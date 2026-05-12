//! Missing data handling for probabilistic graphical models.
//!
//! This module provides tools for:
//! - `types`: Shared types for missing data mechanisms and patterns
//! - `mcar_mar`: Tests for MCAR (Little's test) and MAR mechanism assumptions
//! - `imputation`: Imputation methods (mean/mode, regression, MICE, EM)
//! - `diagnostics`: Diagnostics for evaluating imputation quality
//! - `mnar`: MNAR modeling (pattern-mixture, shared-parameter, selection)
//! - `sensitivity`: Sensitivity analysis for missing data assumptions
//! - `reconstruction`: Data reconstruction from incomplete observations

/// Shared types representing missing data mechanisms, patterns, and masks.
pub mod types;
/// Statistical tests for MCAR (Little's test) and MAR mechanism assumptions.
pub mod mcar_mar;
/// Single and multiple imputation methods (mean/mode, regression, MICE, EM).
pub mod imputation;
/// Diagnostic tools for evaluating imputation quality and detecting bias.
pub mod diagnostics;
/// MNAR (Missing Not At Random) modeling using pattern-mixture and selection models.
pub mod mnar;
/// Sensitivity analysis for untestable missing data assumptions.
pub mod sensitivity;
/// Data reconstruction from partially observed probabilistic graphical model samples.
pub mod reconstruction;

pub use types::{
    MissingDataMechanism, MissingDataPattern, MissingValues, MissingMask,
    PatternSummary,
};
pub use mcar_mar::{
    little_mcar_test, mar_test_logistic, MissingDataTestResult,
};
pub use imputation::{
    ImputationMethod, ImputationResult, ImputationEngine,
};
pub use diagnostics::{
    ImputationDiagnostics, ImputationDiagnosticResult,
};
pub use mnar::{MnarModel, MnarModelType, MnarResult};
pub use sensitivity::{
    SensitivityAnalysis, SensitivityParameter, SensitivityResult,
};
pub use reconstruction::{
    ReconstructionEngine, ReconstructionResult, ReconstructionMethod,
};

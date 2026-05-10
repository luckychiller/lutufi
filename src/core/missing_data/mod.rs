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

pub mod types;
pub mod mcar_mar;
pub mod imputation;
pub mod diagnostics;
pub mod mnar;
pub mod sensitivity;
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

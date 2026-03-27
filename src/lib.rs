//! Lutufi - High-performance network analysis library with probabilistic reasoning

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod core;

#[cfg(feature = "python")]
#[path = "../bindings/ffi/mod.rs"]
pub mod ffi;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::core::*;
}

pub use crate::core::error::{LutufiError, LutufiResult};

/// Convenience alias for LutufiResult
pub type Result<T> = LutufiResult<T>;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Python extension module entry point.
#[cfg(feature = "python")]
#[pymodule]
fn _lutufi(m: &Bound<'_, PyModule>) -> PyResult<()> {
    ffi::register(m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }
}

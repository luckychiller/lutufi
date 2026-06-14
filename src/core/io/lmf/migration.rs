use crate::core::error::{LutufiError, LutufiResult};
use super::types::{LmfDocument, LMF_CURRENT_VERSION};

/// Migrates an [`LmfDocument`] from an older format version to the current
/// version, returning an error if the document version is too old or too new.
pub fn apply_migrations(mut doc: LmfDocument) -> LutufiResult<LmfDocument> {
    match doc.format_version.as_str() {
        "0.9.0" | "0.9.1" | "0.9.2" => {
            doc = migrate_0_9_to_1_0(doc)?;
        }
        "1.0.0" => {}
        v if v.starts_with("0.") => {
            return Err(LutufiError::DeserializationError {
                reason: format!(
                    "LMF format version {} is too old to migrate. Minimum supported: 0.9.0",
                    v
                ),
            });
        }
        v => {
            if semver_gt(v, LMF_CURRENT_VERSION) {
                return Err(LutufiError::DeserializationError {
                    reason: format!(
                        "LMF format version {} is from a newer Lutufi. Please upgrade Lutufi.",
                        v
                    ),
                });
            }
        }
    }
    doc.format_version = LMF_CURRENT_VERSION.to_string();
    Ok(doc)
}

fn migrate_0_9_to_1_0(doc: LmfDocument) -> LutufiResult<LmfDocument> {
    let mut doc = doc;
    if doc.results.is_some() {
        if let Some(ref mut results) = doc.results {
            if results.log_likelihood == 0.0 && !results.marginals.is_empty() {
                results.log_likelihood = f64::NEG_INFINITY;
            }
        }
    }
    if doc.metadata.lutufi_version == "0.1.0-dev" {
        doc.metadata.lutufi_version = "0.1.0".to_string();
    }
    Ok(doc)
}

fn semver_gt(a: &str, b: &str) -> bool {
    let parse = |s: &str| -> Option<(u64, u64, u64)> {
        let parts: Vec<&str> = s.splitn(3, '.').collect();
        if parts.len() != 3 {
            return None;
        }
        Some((
            parts[0].parse().ok()?,
            parts[1].parse().ok()?,
            parts[2].parse().ok()?,
        ))
    };
    match (parse(a), parse(b)) {
        (Some(va), Some(vb)) => va > vb,
        _ => false,
    }
}

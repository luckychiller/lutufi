use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::core::{
    error::{LutufiError, LutufiResult},
    factor::TabularFactor,
    variable::VariableId,
    numerics::ResourceBudget,
};

/// Storage backend for factors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageBackend {
    InMemory,
    MemoryMapped,
}

/// A factor store that can use disk-backed storage for large factors.
///
/// Small factors stay in memory. Factors that would exceed the memory
/// budget are serialized to disk and loaded on demand.
#[derive(Debug, Clone)]
pub struct MemoryMappedFactorStore {
    mmap_dir: PathBuf,
    in_memory: HashMap<(VariableId, usize), TabularFactor>,
    mmapped: HashMap<(VariableId, usize), PathBuf>,
    total_memory_bytes: usize,
    max_memory_bytes: usize,
    file_counter: usize,
}

impl MemoryMappedFactorStore {
    pub fn new<P: AsRef<Path>>(mmap_dir: P, budget: &ResourceBudget) -> Self {
        let dir = mmap_dir.as_ref().to_path_buf();
        let _ = std::fs::create_dir_all(&dir);
        MemoryMappedFactorStore {
            mmap_dir: dir,
            in_memory: HashMap::new(),
            mmapped: HashMap::new(),
            total_memory_bytes: 0,
            max_memory_bytes: budget.max_memory_mb * 1_024 * 1_024,
            file_counter: 0,
        }
    }

    pub fn insert(&mut self, key: (VariableId, usize), factor: TabularFactor) -> LutufiResult<()> {
        let factor_bytes = factor.scope().num_entries() * std::mem::size_of::<f64>();
        let will_exceed = self.total_memory_bytes + factor_bytes > self.max_memory_bytes;

        if will_exceed && factor_bytes > 1024 * 1024 {
            let filename = format!("factor_{}_{}.json", self.file_counter, key.1);
            self.file_counter += 1;
            let path = self.mmap_dir.join(&filename);
            let json = serde_json::to_string(&factor).map_err(|e| LutufiError::SerializationError {
                reason: format!("Failed to serialize factor: {}", e),
            })?;
            std::fs::write(&path, &json).map_err(|e| LutufiError::SerializationError {
                reason: format!("Failed to write factor file: {}", e),
            })?;
            self.mmapped.insert(key, path);
        } else {
            self.total_memory_bytes += factor_bytes;
            self.in_memory.insert(key, factor);
        }
        Ok(())
    }

    pub fn get(&self, key: &(VariableId, usize)) -> LutufiResult<Option<TabularFactor>> {
        if let Some(factor) = self.in_memory.get(key) {
            return Ok(Some(factor.clone()));
        }
        if let Some(path) = self.mmapped.get(key) {
            let json = std::fs::read_to_string(path).map_err(|e| LutufiError::DeserializationError {
                reason: format!("Failed to read factor file: {}", e),
            })?;
            let factor: TabularFactor = serde_json::from_str(&json).map_err(|e| LutufiError::DeserializationError {
                reason: format!("Failed to deserialize factor: {}", e),
            })?;
            return Ok(Some(factor));
        }
        Ok(None)
    }

    pub fn contains_key(&self, key: &(VariableId, usize)) -> bool {
        self.in_memory.contains_key(key) || self.mmapped.contains_key(key)
    }

    pub fn in_memory_count(&self) -> usize { self.in_memory.len() }
    pub fn mmapped_count(&self) -> usize { self.mmapped.len() }
    pub fn memory_usage_bytes(&self) -> usize { self.total_memory_bytes }

    pub fn cleanup(&mut self) -> LutufiResult<()> {
        for (_, path) in self.mmapped.drain() {
            let _ = std::fs::remove_file(&path);
        }
        self.in_memory.clear();
        self.total_memory_bytes = 0;
        Ok(())
    }
}

impl Drop for MemoryMappedFactorStore {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::factor::Scope;
    use crate::core::variable::Variable;
    use crate::core::domain::Domain;

    #[test]
    fn test_in_memory_storage() {
        let budget = ResourceBudget::default();
        let tmp = std::env::temp_dir().join("lutufi_mmap_test");
        let mut store = MemoryMappedFactorStore::new(&tmp, &budget);

        let v = Variable::new("X", Domain::binary());
        let scope = Scope::new(vec![&v]);
        let factor = TabularFactor::from_values(scope, vec![0.5, 0.5]).unwrap();
        let key = (v.id(), 0);
        store.insert(key, factor.clone()).unwrap();
        assert!(store.contains_key(&key));
        let retrieved = store.get(&key).unwrap().unwrap();
        assert!((retrieved.value_at(0) - 0.5).abs() < 1e-10);
        store.cleanup().unwrap();
        let _ = std::fs::remove_dir_all(&tmp);
    }
}

//! Memory management for the Gifaros kernel.
//!
//! Tracks the total available memory and per-process allocations. AI workloads
//! can request large contiguous regions for model weights.

use std::collections::HashMap;
use crate::process::ProcessId;

/// Simple flat-memory manager used by the Gifaros kernel.
///
/// In a real implementation this would manage page tables and virtual address
/// spaces. Here it tracks byte-level allocations per process.
pub struct MemoryManager {
    total_bytes: usize,
    allocations: HashMap<ProcessId, usize>,
}

impl MemoryManager {
    /// Create a manager with `total_bytes` of addressable memory.
    pub fn new(total_bytes: usize) -> Self {
        MemoryManager {
            total_bytes,
            allocations: HashMap::new(),
        }
    }

    /// Total memory the system has.
    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    /// Bytes currently allocated across all processes.
    pub fn used_bytes(&self) -> usize {
        self.allocations.values().sum()
    }

    /// Bytes not yet allocated.
    pub fn free_bytes(&self) -> usize {
        self.total_bytes.saturating_sub(self.used_bytes())
    }

    /// Attempt to allocate `bytes` for `pid`.  Returns `true` on success.
    pub fn allocate(&mut self, pid: ProcessId, bytes: usize) -> bool {
        if bytes > self.free_bytes() {
            return false;
        }
        *self.allocations.entry(pid).or_insert(0) += bytes;
        true
    }

    /// Release all memory held by `pid`.
    pub fn free(&mut self, pid: ProcessId) {
        self.allocations.remove(&pid);
    }

    /// Bytes allocated to a specific process.
    pub fn allocation_for(&self, pid: ProcessId) -> usize {
        *self.allocations.get(&pid).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state() {
        let mm = MemoryManager::new(1024);
        assert_eq!(mm.total_bytes(), 1024);
        assert_eq!(mm.used_bytes(), 0);
        assert_eq!(mm.free_bytes(), 1024);
    }

    #[test]
    fn allocate_and_free() {
        let mut mm = MemoryManager::new(1024);
        let pid = ProcessId(1);
        assert!(mm.allocate(pid, 512));
        assert_eq!(mm.used_bytes(), 512);
        assert_eq!(mm.free_bytes(), 512);
        mm.free(pid);
        assert_eq!(mm.used_bytes(), 0);
    }

    #[test]
    fn over_allocation_rejected() {
        let mut mm = MemoryManager::new(256);
        let pid = ProcessId(1);
        assert!(!mm.allocate(pid, 512));
        assert_eq!(mm.used_bytes(), 0);
    }

    #[test]
    fn multiple_processes() {
        let mut mm = MemoryManager::new(1024);
        assert!(mm.allocate(ProcessId(1), 300));
        assert!(mm.allocate(ProcessId(2), 300));
        assert!(!mm.allocate(ProcessId(3), 500));
        assert_eq!(mm.free_bytes(), 424);
    }
}

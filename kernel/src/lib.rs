//! Gifaros Kernel
//!
//! The core of the Gifaros AI-era operating system. Manages processes, memory,
//! and scheduling with first-class support for AI workloads.

pub mod memory;
pub mod process;
pub mod scheduler;

pub use memory::MemoryManager;
pub use process::{Process, ProcessId, ProcessPriority, ProcessState};
pub use scheduler::Scheduler;

/// Kernel version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialise the kernel subsystems and return a ready-to-use [`Kernel`].
pub struct Kernel {
    pub scheduler: Scheduler,
    pub memory: MemoryManager,
}

impl Kernel {
    /// Boot a new kernel instance.
    pub fn boot(total_memory_bytes: usize) -> Self {
        Kernel {
            scheduler: Scheduler::new(),
            memory: MemoryManager::new(total_memory_bytes),
        }
    }

    /// Tick the kernel forward one scheduling cycle.
    pub fn tick(&mut self) {
        self.scheduler.tick();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boot_creates_kernel() {
        let k = Kernel::boot(1024 * 1024 * 256);
        assert_eq!(k.scheduler.process_count(), 0);
        assert_eq!(k.memory.total_bytes(), 1024 * 1024 * 256);
    }
}

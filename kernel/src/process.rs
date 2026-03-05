//! Process management for the Gifaros kernel.
//!
//! Processes are the fundamental unit of execution. The kernel supports
//! AI-tagged processes so the scheduler can give them preferential access
//! to hardware accelerators.

/// Unique identifier for a process.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(pub u64);

/// Execution state of a process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessState {
    /// Waiting to be scheduled.
    Ready,
    /// Currently executing.
    Running,
    /// Blocked on I/O or an event.
    Blocked,
    /// Finished execution.
    Terminated,
}

/// Scheduling priority of a process.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProcessPriority {
    /// Low-priority background task.
    Low = 0,
    /// Standard user process.
    Normal = 1,
    /// Interactive or time-sensitive process.
    High = 2,
    /// AI inference / training workload – boosted by the AI-aware scheduler.
    AiAccelerated = 3,
}

/// A process in the Gifaros kernel.
#[derive(Debug, Clone)]
pub struct Process {
    /// Unique identifier.
    pub id: ProcessId,
    /// Human-readable name (may come from a natural-language description).
    pub name: String,
    /// Current execution state.
    pub state: ProcessState,
    /// Scheduling priority.
    pub priority: ProcessPriority,
    /// Number of CPU quanta this process has consumed.
    pub cpu_time: u64,
    /// Amount of memory allocated to this process (bytes).
    pub memory_bytes: usize,
}

impl Process {
    /// Create a new process in the [`ProcessState::Ready`] state.
    pub fn new(
        id: ProcessId,
        name: impl Into<String>,
        priority: ProcessPriority,
        memory_bytes: usize,
    ) -> Self {
        Process {
            id,
            name: name.into(),
            state: ProcessState::Ready,
            priority,
            cpu_time: 0,
            memory_bytes,
        }
    }

    /// Returns `true` if this is an AI-accelerated workload.
    pub fn is_ai_workload(&self) -> bool {
        self.priority == ProcessPriority::AiAccelerated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_process_is_ready() {
        let p = Process::new(ProcessId(1), "test", ProcessPriority::Normal, 4096);
        assert_eq!(p.state, ProcessState::Ready);
        assert_eq!(p.cpu_time, 0);
        assert!(!p.is_ai_workload());
    }

    #[test]
    fn ai_process_detected() {
        let p = Process::new(ProcessId(2), "llm-infer", ProcessPriority::AiAccelerated, 1024 * 1024);
        assert!(p.is_ai_workload());
    }

    #[test]
    fn priority_ordering() {
        assert!(ProcessPriority::AiAccelerated > ProcessPriority::High);
        assert!(ProcessPriority::High > ProcessPriority::Normal);
        assert!(ProcessPriority::Normal > ProcessPriority::Low);
    }
}

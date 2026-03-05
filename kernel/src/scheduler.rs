//! AI-aware process scheduler for the Gifaros kernel.
//!
//! The scheduler uses a priority queue that elevates AI-accelerated workloads.
//! Each call to [`Scheduler::tick`] selects the highest-priority [`ProcessState::Ready`]
//! process, marks it as [`ProcessState::Running`] for one quantum, then yields it back
//! to the [`ProcessState::Ready`] pool (round-robin within the same priority level).

use std::collections::VecDeque;
use crate::process::{Process, ProcessId, ProcessPriority, ProcessState};

/// Schedules processes, giving AI workloads preferential treatment.
pub struct Scheduler {
    /// Queues indexed by priority (index 0 = Low, index 3 = AiAccelerated).
    queues: [VecDeque<Process>; 4],
    /// The process that is currently executing, if any.
    running: Option<Process>,
    /// Monotonically increasing tick counter.
    tick_count: u64,
}

impl Scheduler {
    /// Create an empty scheduler.
    pub fn new() -> Self {
        Scheduler {
            queues: [
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
                VecDeque::new(),
            ],
            running: None,
            tick_count: 0,
        }
    }

    /// Add a process to the scheduler.
    pub fn spawn(&mut self, mut process: Process) {
        process.state = ProcessState::Ready;
        let idx = Self::priority_index(&process.priority);
        self.queues[idx].push_back(process);
    }

    /// Number of processes known to the scheduler (ready + running).
    pub fn process_count(&self) -> usize {
        let queued: usize = self.queues.iter().map(|q| q.len()).sum();
        queued + if self.running.is_some() { 1 } else { 0 }
    }

    /// Advance one scheduling quantum.
    ///
    /// The previously running process (if any) is re-queued as Ready, then
    /// the highest-priority Ready process is selected and marked Running.
    pub fn tick(&mut self) {
        self.tick_count += 1;

        // Re-queue the previously running process.
        if let Some(mut prev) = self.running.take() {
            prev.state = ProcessState::Ready;
            let idx = Self::priority_index(&prev.priority);
            self.queues[idx].push_back(prev);
        }

        // Pick the highest-priority ready process (highest index first).
        for queue in self.queues.iter_mut().rev() {
            if let Some(mut next) = queue.pop_front() {
                next.state = ProcessState::Running;
                next.cpu_time += 1;
                self.running = Some(next);
                return;
            }
        }
    }

    /// Remove a process from the scheduler by ID (wherever it is).
    pub fn terminate(&mut self, pid: ProcessId) {
        if let Some(ref p) = self.running {
            if p.id == pid {
                self.running = None;
                return;
            }
        }
        for queue in self.queues.iter_mut() {
            if let Some(pos) = queue.iter().position(|p| p.id == pid) {
                queue.remove(pos);
                return;
            }
        }
    }

    /// The currently running process, if any.
    pub fn running_process(&self) -> Option<&Process> {
        self.running.as_ref()
    }

    /// Total scheduler ticks elapsed.
    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    fn priority_index(p: &ProcessPriority) -> usize {
        match p {
            ProcessPriority::Low => 0,
            ProcessPriority::Normal => 1,
            ProcessPriority::High => 2,
            ProcessPriority::AiAccelerated => 3,
        }
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_process(id: u64, priority: ProcessPriority) -> Process {
        Process::new(ProcessId(id), format!("p{}", id), priority, 4096)
    }

    #[test]
    fn empty_scheduler_tick_is_noop() {
        let mut s = Scheduler::new();
        s.tick();
        assert!(s.running_process().is_none());
        assert_eq!(s.tick_count(), 1);
    }

    #[test]
    fn single_process_runs() {
        let mut s = Scheduler::new();
        s.spawn(make_process(1, ProcessPriority::Normal));
        s.tick();
        let r = s.running_process().unwrap();
        assert_eq!(r.id, ProcessId(1));
        assert_eq!(r.state, ProcessState::Running);
        assert_eq!(r.cpu_time, 1);
    }

    #[test]
    fn ai_workload_preempts_normal() {
        let mut s = Scheduler::new();
        s.spawn(make_process(1, ProcessPriority::Normal));
        s.spawn(make_process(2, ProcessPriority::AiAccelerated));
        s.tick();
        assert_eq!(s.running_process().unwrap().id, ProcessId(2));
    }

    #[test]
    fn round_robin_within_priority() {
        let mut s = Scheduler::new();
        s.spawn(make_process(1, ProcessPriority::Normal));
        s.spawn(make_process(2, ProcessPriority::Normal));
        s.tick(); // runs p1
        assert_eq!(s.running_process().unwrap().id, ProcessId(1));
        s.tick(); // runs p2
        assert_eq!(s.running_process().unwrap().id, ProcessId(2));
        s.tick(); // back to p1
        assert_eq!(s.running_process().unwrap().id, ProcessId(1));
    }

    #[test]
    fn process_count_tracked() {
        let mut s = Scheduler::new();
        assert_eq!(s.process_count(), 0);
        s.spawn(make_process(1, ProcessPriority::Low));
        s.spawn(make_process(2, ProcessPriority::High));
        assert_eq!(s.process_count(), 2);
        s.tick();
        assert_eq!(s.process_count(), 2);
    }

    #[test]
    fn terminate_removes_process() {
        let mut s = Scheduler::new();
        s.spawn(make_process(1, ProcessPriority::Normal));
        s.spawn(make_process(2, ProcessPriority::Normal));
        s.tick(); // p1 runs
        s.terminate(ProcessId(1)); // terminate running
        assert_eq!(s.process_count(), 1);
        s.tick();
        assert_eq!(s.running_process().unwrap().id, ProcessId(2));
    }
}

//! Gifaros Shell – the natural-language command interface.
//!
//! Reads lines from stdin, passes them through the AI assistant, and
//! executes the resulting intents against the kernel.

use std::io::{self, BufRead, Write};

use ai_core::{Assistant, intent::IntentKind};
use kernel::{
    Kernel,
    process::{Process, ProcessId, ProcessPriority},
};

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║  Gifaros OS  –  AI-era operating system      ║");
    println!("║  Type a command in plain English, or 'exit'  ║");
    println!("╚══════════════════════════════════════════════╝");
    println!();

    let mut assistant = Assistant::new();
    // Boot the kernel with 256 MiB of memory.
    let mut kernel = Kernel::boot(256 * 1024 * 1024);
    let mut next_pid: u64 = 1;

    let stdin = io::stdin();
    loop {
        print!("gifaros> ");
        io::stdout().flush().ok();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() || line.is_empty() {
            break;
        }
        let input = line.trim();
        if input.is_empty() {
            continue;
        }

        let response = assistant.process(input);
        println!("{}", response.text);

        match &response.intent.kind {
            IntentKind::Exit => break,

            IntentKind::Launch => {
                if let Some(ref name) = response.intent.target {
                    let pid = ProcessId(next_pid);
                    let process = Process::new(pid, name.clone(), ProcessPriority::Normal, 4 * 1024 * 1024);
                    if kernel.memory.allocate(pid, process.memory_bytes) {
                        next_pid += 1;
                        kernel.scheduler.spawn(process);
                        kernel.tick();
                        println!(
                            "[kernel] Process '{}' started (PID {}). Total processes: {}",
                            name,
                            pid.0,
                            kernel.scheduler.process_count()
                        );
                    } else {
                        println!(
                            "[kernel] Not enough memory to launch '{}' ({} bytes needed, {} free).",
                            name,
                            4 * 1024 * 1024,
                            kernel.memory.free_bytes()
                        );
                    }
                }
            }

            IntentKind::Terminate => {
                if let Some(ref name) = response.intent.target {
                    // Find by name – simple linear scan for this demo.
                    let found = find_pid_by_name(&kernel, name);
                    if let Some(pid) = found {
                        kernel.memory.free(pid);
                        kernel.scheduler.terminate(pid);
                        println!("[kernel] Process '{}' (PID {}) terminated.", name, pid.0);
                    } else {
                        println!("[kernel] No process named '{}' found.", name);
                    }
                }
            }

            IntentKind::Query => {
                println!("[kernel] Processes running : {}", kernel.scheduler.process_count());
                println!("[kernel] Memory used       : {} bytes", kernel.memory.used_bytes());
                println!("[kernel] Memory free       : {} bytes", kernel.memory.free_bytes());
                println!("[kernel] Scheduler ticks   : {}", kernel.scheduler.tick_count());
                if let Some(p) = kernel.scheduler.running_process() {
                    println!("[kernel] Currently running : {} (PID {})", p.name, p.id.0);
                }
            }

            _ => {}
        }

        println!();
    }

    println!("Session ended.");
}

/// Walk all scheduler queues looking for a process with `name`.
fn find_pid_by_name(kernel: &Kernel, name: &str) -> Option<ProcessId> {
    // The scheduler exposes the running process; for queued ones we rely on
    // a probe-and-return approach. In a production kernel the scheduler would
    // maintain a separate process table. Here we keep it simple.
    if let Some(p) = kernel.scheduler.running_process() {
        if p.name == name {
            return Some(p.id);
        }
    }
    None
}

# Gifaros OS

> A next-generation operating system designed from the ground up for the AI era.  
> Reimagining human-computer interaction.

Gifaros is a Rust workspace that demonstrates the core abstractions of an
AI-era operating system: a priority-aware kernel, an AI assistant engine, and
a natural-language shell.

## Crates

| Crate | Description |
|-------|-------------|
| `kernel` | Core OS kernel – process management, flat memory manager, AI-aware scheduler |
| `ai_core` | Intent recognition, conversational context, and the AI assistant |
| `shell` | Natural-language shell binary (`gifaros-shell`) |

## Architecture

```
┌────────────────────────────────────────────┐
│              gifaros-shell                 │
│  reads plain-English commands from stdin   │
└──────────────┬────────────────┬────────────┘
               │                │
        ┌──────▼──────┐  ┌──────▼──────┐
        │   ai_core   │  │   kernel    │
        │  Intent     │  │  Scheduler  │
        │  Recognizer │  │  Memory Mgr │
        │  Assistant  │  │  Process    │
        └─────────────┘  └─────────────┘
```

### AI-aware scheduler

Processes carry a `ProcessPriority`:

| Priority | Description |
|----------|-------------|
| `Low` | Background tasks |
| `Normal` | Standard user processes |
| `High` | Interactive / time-sensitive |
| `AiAccelerated` | AI inference / training – always scheduled first |

### Natural-language shell

The shell accepts plain-English commands and maps them to kernel actions:

```
gifaros> open browser
Launching 'browser'…
[kernel] Process 'browser' started (PID 1). Total processes: 1

gifaros> show memory
Fetching system information…
[kernel] Processes running : 1
[kernel] Memory used       : 4194304 bytes
[kernel] Memory free       : 264241152 bytes

gifaros> what is an AI operating system?
You asked: "what is an AI operating system?". I am processing your question with the on-device AI model.

gifaros> exit
Goodbye! Shutting down your session.
```

## Getting started

```bash
# Build everything
cargo build

# Run the shell
cargo run --bin gifaros-shell

# Run all tests
cargo test
```

## Requirements

* Rust 1.70+ (2021 edition)

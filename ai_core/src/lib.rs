//! Gifaros AI Core
//!
//! Provides the intent recognition and assistant engine that powers the
//! natural-language interface of the Gifaros AI-era operating system.

pub mod assistant;
pub mod context;
pub mod intent;

pub use assistant::Assistant;
pub use context::Context;
pub use intent::{Intent, IntentKind, IntentRecognizer};

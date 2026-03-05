//! Intent recognition – maps natural-language input to structured intents.
//!
//! The recognizer uses keyword-based pattern matching. In a production system
//! this layer would call an on-device language model.

/// High-level categories of user intent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntentKind {
    /// Launch or start an application / process.
    Launch,
    /// Stop or kill a running process.
    Terminate,
    /// Query system information (memory, processes, …).
    Query,
    /// Navigate to a location (file system, web, etc.).
    Navigate,
    /// Create a new file, directory, or resource.
    Create,
    /// Delete a file, directory, or resource.
    Delete,
    /// Ask the AI assistant an open-ended question.
    Ask,
    /// Exit the shell.
    Exit,
    /// Input did not match any known intent.
    Unknown,
}

/// A structured representation of what the user wants to do.
#[derive(Debug, Clone)]
pub struct Intent {
    /// The high-level intent category.
    pub kind: IntentKind,
    /// The raw natural-language input that produced this intent.
    pub raw_input: String,
    /// Optional target extracted from the input (application name, path, …).
    pub target: Option<String>,
    /// Confidence score in the range [0.0, 1.0].
    pub confidence: f32,
}

impl Intent {
    /// Convenience constructor.
    pub fn new(kind: IntentKind, raw_input: impl Into<String>, target: Option<String>, confidence: f32) -> Self {
        Intent {
            kind,
            raw_input: raw_input.into(),
            target,
            confidence,
        }
    }

    /// Returns `true` if the intent is actionable (non-unknown, adequate confidence).
    pub fn is_actionable(&self) -> bool {
        self.kind != IntentKind::Unknown && self.confidence >= 0.5
    }
}

/// Recognises user intents from natural-language text.
pub struct IntentRecognizer;

impl IntentRecognizer {
    pub fn new() -> Self {
        IntentRecognizer
    }

    /// Classify `input` into an [`Intent`].
    pub fn recognize(&self, input: &str) -> Intent {
        let lower = input.to_lowercase();
        let words: Vec<&str> = lower.split_whitespace().collect();

        if words.is_empty() {
            return Intent::new(IntentKind::Unknown, input, None, 0.0);
        }

        // Exit
        if matches!(words[0], "exit" | "quit" | "bye" | "logout") {
            return Intent::new(IntentKind::Exit, input, None, 1.0);
        }

        // Launch
        if words.iter().any(|w| matches!(*w, "launch" | "open" | "start" | "run" | "execute")) {
            let target = Self::extract_target(&words, &["launch", "open", "start", "run", "execute"]);
            return Intent::new(IntentKind::Launch, input, target, 0.9);
        }

        // Terminate
        if words.iter().any(|w| matches!(*w, "kill" | "stop" | "terminate" | "close" | "end")) {
            let target = Self::extract_target(&words, &["kill", "stop", "terminate", "close", "end"]);
            return Intent::new(IntentKind::Terminate, input, target, 0.9);
        }

        // Navigate
        if words.iter().any(|w| matches!(*w, "go" | "navigate" | "cd" | "move")) {
            let target = Self::extract_target(&words, &["go", "navigate", "cd", "move", "to"]);
            return Intent::new(IntentKind::Navigate, input, target, 0.85);
        }

        // Create
        if words.iter().any(|w| matches!(*w, "create" | "make" | "new" | "touch" | "mkdir")) {
            let target = Self::extract_target(&words, &["create", "make", "new", "touch", "mkdir", "a", "an"]);
            return Intent::new(IntentKind::Create, input, target, 0.85);
        }

        // Delete
        if words.iter().any(|w| matches!(*w, "delete" | "remove" | "rm" | "erase")) {
            let target = Self::extract_target(&words, &["delete", "remove", "rm", "erase"]);
            return Intent::new(IntentKind::Delete, input, target, 0.85);
        }

        // Query
        if words.iter().any(|w| matches!(*w, "show" | "list" | "status" | "ps" | "info" | "memory" | "mem")) {
            return Intent::new(IntentKind::Query, input, None, 0.8);
        }

        // Open-ended question → Ask
        if words[0].ends_with('?') || matches!(words[0], "what" | "how" | "why" | "when" | "where" | "who") {
            return Intent::new(IntentKind::Ask, input, None, 0.75);
        }
        if words.contains(&"?") || lower.ends_with('?') {
            return Intent::new(IntentKind::Ask, input, None, 0.75);
        }

        Intent::new(IntentKind::Unknown, input, None, 0.0)
    }

    /// Extract the first word after any of the `skip_words`.
    fn extract_target(words: &[&str], skip_words: &[&str]) -> Option<String> {
        for (i, w) in words.iter().enumerate() {
            if skip_words.contains(w) {
                if let Some(next) = words.get(i + 1) {
                    return Some(next.to_string());
                }
            }
        }
        None
    }
}

impl Default for IntentRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recognizer() -> IntentRecognizer {
        IntentRecognizer::new()
    }

    #[test]
    fn recognizes_exit() {
        let i = recognizer().recognize("exit");
        assert_eq!(i.kind, IntentKind::Exit);
        assert!(i.is_actionable());
    }

    #[test]
    fn recognizes_launch_with_target() {
        let i = recognizer().recognize("open browser");
        assert_eq!(i.kind, IntentKind::Launch);
        assert_eq!(i.target.as_deref(), Some("browser"));
        assert!(i.is_actionable());
    }

    #[test]
    fn recognizes_terminate() {
        let i = recognizer().recognize("kill my-app");
        assert_eq!(i.kind, IntentKind::Terminate);
        assert_eq!(i.target.as_deref(), Some("my-app"));
    }

    #[test]
    fn recognizes_query() {
        let i = recognizer().recognize("show memory");
        assert_eq!(i.kind, IntentKind::Query);
        assert!(i.is_actionable());
    }

    #[test]
    fn recognizes_navigate() {
        let i = recognizer().recognize("go to /home");
        assert_eq!(i.kind, IntentKind::Navigate);
    }

    #[test]
    fn recognizes_create() {
        let i = recognizer().recognize("create a new file");
        assert_eq!(i.kind, IntentKind::Create);
    }

    #[test]
    fn recognizes_delete() {
        let i = recognizer().recognize("delete old-log");
        assert_eq!(i.kind, IntentKind::Delete);
        assert_eq!(i.target.as_deref(), Some("old-log"));
    }

    #[test]
    fn recognizes_question() {
        let i = recognizer().recognize("what processes are running?");
        assert_eq!(i.kind, IntentKind::Ask);
    }

    #[test]
    fn unknown_intent() {
        let i = recognizer().recognize("xyzzy plugh");
        assert_eq!(i.kind, IntentKind::Unknown);
        assert!(!i.is_actionable());
    }

    #[test]
    fn empty_input_is_unknown() {
        let i = recognizer().recognize("");
        assert_eq!(i.kind, IntentKind::Unknown);
    }
}

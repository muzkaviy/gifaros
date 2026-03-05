//! The AI assistant – the high-level interface between the user and the OS.
//!
//! [`Assistant`] wires together [`IntentRecognizer`], [`Context`], and
//! the kernel to translate natural-language commands into system actions.

use crate::{
    context::Context,
    intent::{Intent, IntentKind, IntentRecognizer},
};

/// Response produced by the assistant for a single user input.
#[derive(Debug, Clone)]
pub struct Response {
    /// The human-readable reply to display.
    pub text: String,
    /// The structured intent that was interpreted.
    pub intent: Intent,
}

/// The Gifaros AI assistant.
///
/// Call [`Assistant::process`] with each line of user input and display
/// the returned [`Response::text`] to the user.
pub struct Assistant {
    recognizer: IntentRecognizer,
    context: Context,
}

impl Assistant {
    /// Create a new assistant.
    pub fn new() -> Self {
        Assistant {
            recognizer: IntentRecognizer::new(),
            context: Context::new(100),
        }
    }

    /// Process one line of user input and return the assistant's response.
    pub fn process(&mut self, input: &str) -> Response {
        self.context.push_user(input);

        let intent = self.recognizer.recognize(input);
        let text = self.generate_response(&intent);

        self.context.push_system(&text);

        Response { text, intent }
    }

    /// Read-only access to the conversation context.
    pub fn context(&self) -> &Context {
        &self.context
    }

    // ------------------------------------------------------------------
    // Internal response generation
    // ------------------------------------------------------------------

    fn generate_response(&self, intent: &Intent) -> String {
        match &intent.kind {
            IntentKind::Exit => "Goodbye! Shutting down your session.".to_string(),

            IntentKind::Launch => {
                match &intent.target {
                    Some(t) => format!("Launching '{}'…", t),
                    None => "What would you like to launch?".to_string(),
                }
            }

            IntentKind::Terminate => {
                match &intent.target {
                    Some(t) => format!("Terminating '{}'…", t),
                    None => "Which process should I terminate?".to_string(),
                }
            }

            IntentKind::Navigate => {
                match &intent.target {
                    Some(t) => format!("Navigating to '{}'…", t),
                    None => "Where would you like to go?".to_string(),
                }
            }

            IntentKind::Create => {
                match &intent.target {
                    Some(t) => format!("Creating '{}'…", t),
                    None => "What would you like to create?".to_string(),
                }
            }

            IntentKind::Delete => {
                match &intent.target {
                    Some(t) => format!("Deleting '{}'. This action is irreversible – confirm? (yes/no)", t),
                    None => "What would you like to delete?".to_string(),
                }
            }

            IntentKind::Query => {
                "Fetching system information…".to_string()
            }

            IntentKind::Ask => {
                format!(
                    "You asked: \"{}\". I am processing your question with the on-device AI model.",
                    intent.raw_input
                )
            }

            IntentKind::Unknown => {
                format!(
                    "I didn't understand '{}'. Try phrasing that differently, or type 'help'.",
                    intent.raw_input
                )
            }
        }
    }
}

impl Default for Assistant {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_response() {
        let mut a = Assistant::new();
        let r = a.process("exit");
        assert_eq!(r.intent.kind, IntentKind::Exit);
        assert!(r.text.to_lowercase().contains("goodbye"));
    }

    #[test]
    fn launch_with_target() {
        let mut a = Assistant::new();
        let r = a.process("open notes");
        assert_eq!(r.intent.kind, IntentKind::Launch);
        assert!(r.text.contains("notes"));
    }

    #[test]
    fn unknown_input_prompts_help() {
        let mut a = Assistant::new();
        let r = a.process("flibbertigibbet");
        assert_eq!(r.intent.kind, IntentKind::Unknown);
        assert!(r.text.contains("help"));
    }

    #[test]
    fn context_grows_with_conversation() {
        let mut a = Assistant::new();
        a.process("hello");
        a.process("show memory");
        // Each call produces one user turn + one system turn.
        assert_eq!(a.context().len(), 4);
    }

    #[test]
    fn delete_asks_for_confirmation() {
        let mut a = Assistant::new();
        let r = a.process("delete old-logs");
        assert!(r.text.contains("irreversible") || r.text.contains("confirm"));
    }
}

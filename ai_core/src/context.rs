//! Conversational context: tracks the history of a user session.

/// A single turn in the conversation (user or system).
#[derive(Debug, Clone)]
pub struct Turn {
    pub speaker: Speaker,
    pub text: String,
}

/// Who produced a conversation turn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Speaker {
    User,
    System,
}

/// Maintains the history of a conversation session.
pub struct Context {
    turns: Vec<Turn>,
    max_history: usize,
}

impl Context {
    /// Create a context that retains at most `max_history` turns.
    pub fn new(max_history: usize) -> Self {
        Context {
            turns: Vec::new(),
            max_history,
        }
    }

    /// Record a user utterance.
    pub fn push_user(&mut self, text: impl Into<String>) {
        self.push(Turn { speaker: Speaker::User, text: text.into() });
    }

    /// Record a system response.
    pub fn push_system(&mut self, text: impl Into<String>) {
        self.push(Turn { speaker: Speaker::System, text: text.into() });
    }

    /// All recorded turns.
    pub fn turns(&self) -> &[Turn] {
        &self.turns
    }

    /// Number of turns in history.
    pub fn len(&self) -> usize {
        self.turns.len()
    }

    /// Returns `true` when the history is empty.
    pub fn is_empty(&self) -> bool {
        self.turns.is_empty()
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.turns.clear();
    }

    fn push(&mut self, turn: Turn) {
        if self.turns.len() >= self.max_history {
            self.turns.remove(0);
        }
        self.turns.push(turn);
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new(50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_retrieve() {
        let mut ctx = Context::new(10);
        ctx.push_user("hello");
        ctx.push_system("Hi! How can I help?");
        assert_eq!(ctx.len(), 2);
        assert_eq!(ctx.turns()[0].speaker, Speaker::User);
        assert_eq!(ctx.turns()[1].speaker, Speaker::System);
    }

    #[test]
    fn history_is_bounded() {
        let mut ctx = Context::new(3);
        ctx.push_user("a");
        ctx.push_user("b");
        ctx.push_user("c");
        ctx.push_user("d"); // "a" should be evicted
        assert_eq!(ctx.len(), 3);
        assert_eq!(ctx.turns()[0].text, "b");
    }

    #[test]
    fn clear_resets() {
        let mut ctx = Context::default();
        ctx.push_user("hello");
        ctx.clear();
        assert!(ctx.is_empty());
    }
}

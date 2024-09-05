use super::ThreadMessage;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The metrics that the dreamer agent can report to the outside world1.
pub enum Metrics {
    /// The thread message.
    ThreadMessage(ThreadMessage),
}

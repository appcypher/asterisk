use tokio::sync::mpsc;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The channel handles for the agent side. This is, for example, how the agent gets messages from
/// the outside world and send action requests to the outside world.
pub struct AgentSideChannels {
    /// The channel for receiving messages from the outside world.
    pub message_rx: mpsc::UnboundedReceiver<String>,

    /// The channel for sending action requests to the outside world.
    pub action_tx: mpsc::UnboundedSender<String>,
}

/// The channel handles for the external side. This is, for example, how the outside world sends
/// messages to the agent and receive action requests from the agent.
pub struct ExternalSideChannels {
    /// The channel for sending messages to the outside world.
    pub message_tx: mpsc::UnboundedSender<String>,

    /// The channel for receiving action requests from the outside world.
    pub action_rx: mpsc::UnboundedReceiver<String>,
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates a new set of channel handles for the agent and external side.
pub fn create() -> (AgentSideChannels, ExternalSideChannels) {
    let (message_tx, message_rx) = mpsc::unbounded_channel();
    let (action_tx, action_rx) = mpsc::unbounded_channel();

    (
        AgentSideChannels {
            message_rx,
            action_tx,
        },
        ExternalSideChannels {
            message_tx,
            action_rx,
        },
    )
}

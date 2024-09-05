//! Defines the channels used for communication between the agent and the outside world.

use tokio::sync::mpsc;

use super::{ActionMessage, Metrics};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The channel handles for the agent side. This is, for example, how the agent gets messages from
/// the outside world and send action requests to the outside world.
#[derive(Debug)]
pub struct AgentSideChannels {
    /// The channel for receiving messages from the outside world.
    pub message_rx: mpsc::UnboundedReceiver<String>,

    /// The channel for sending action requests to the outside world.
    pub action_tx: mpsc::UnboundedSender<ActionMessage>,

    /// The channel for sending metrics to the outside world.
    pub metrics_tx: mpsc::UnboundedSender<Metrics>,
}

/// The channel handles for the outside world. This is, for example, how the outside world sends
/// messages to the agent and receive action requests from the agent.
#[derive(Debug)]
pub struct ExternalSideChannels {
    /// The channel for sending messages to the outside world.
    pub message_tx: mpsc::UnboundedSender<String>,

    /// The channel for receiving action requests from the outside world.
    pub action_rx: mpsc::UnboundedReceiver<ActionMessage>,

    /// The channel for receiving metrics from the outside world.
    pub metrics_rx: mpsc::UnboundedReceiver<Metrics>,
}

/// The channel handles for the agent and outside world.
pub type Channels = (AgentSideChannels, ExternalSideChannels);

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Creates a new set of channel handles for the agent and outside world.
pub fn create() -> Channels {
    let (message_tx, message_rx) = mpsc::unbounded_channel();
    let (action_tx, action_rx) = mpsc::unbounded_channel();
    let (metrics_tx, metrics_rx) = mpsc::unbounded_channel();
    (
        AgentSideChannels {
            message_rx,
            action_tx,
            metrics_tx,
        },
        ExternalSideChannels {
            message_tx,
            action_rx,
            metrics_rx,
        },
    )
}

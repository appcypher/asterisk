use std::{io::Write, process};

use asterisk_core::agents::dreamer::{
    channels, ActionMessage, Dreamer, Metrics, ThreadMessage, ACTION_TAG, THOUGHT_TAG,
};
use colored::{Color, Colorize};
use crossterm::{
    event::{Event, EventStream, KeyCode, KeyModifiers},
    terminal,
};
use futures_util::StreamExt;
use lazy_static::lazy_static;
use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    sync::mpsc,
};

use crate::CliResult;

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Runs the shell.
pub async fn run() -> CliResult<()> {
    // Create the agent
    let agent = Dreamer::builder().build();

    println!(
        "{}",
        " Dreamer Agent Initialized "
            .bold()
            .color(*SYSTEM_MESSAGE_HEADER_FG_COLOR)
            .on_color(*SYSTEM_MESSAGE_HEADER_BG_COLOR)
    );

    // Create channels for the agent and external communication
    let (agent_channels, mut external_channels) = channels::create();

    // Spawn the agent in a new task
    agent.run(agent_channels);

    // Prompt the user for input
    prompt(&external_channels.message_tx).await?;

    // Handle agent actions, metrics
    tokio::spawn(async move {
        loop {
            tokio::select! {
                metrics = external_channels.metrics_rx.recv() => if let Some(metrics) = metrics {
                    handle_metric_message(metrics)?;
                },
                action = external_channels.action_rx.recv() => if let Some(action) = action {
                    handle_action_message(action)?;
                },
            }
        }

        #[allow(unreachable_code)]
        crate::Ok(())
    });

    // Handle terminal events
    let mut reader = EventStream::new();
    loop {
        terminal::enable_raw_mode()?;
        if let Some(event) = reader.next().await {
            handle_terminal_event(event?, &external_channels.message_tx).await?;
        }
    }
}

fn handle_metric_message(metrics: Metrics) -> CliResult<()> {
    terminal::disable_raw_mode()?;
    match metrics {
        Metrics::ThreadMessage(message) => match message {
            ThreadMessage::Thought(message) => {
                println!(
                    "\n{}{}{}",
                    " Agent Event:"
                        .bold()
                        .color(*SYSTEM_MESSAGE_HEADER_FG_COLOR)
                        .on_color(*SYSTEM_MESSAGE_HEADER_BG_COLOR),
                    THOUGHT_TAG.bold().color(*THOUGHT_TAG_COLOR),
                    message
                        .get_main_content()
                        .italic()
                        .color(*THOUGHT_TAG_COLOR)
                );
            }
            ThreadMessage::Action(message) => {
                println!(
                    "\n{}{}{}",
                    " Agent Event:"
                        .bold()
                        .color(*SYSTEM_MESSAGE_HEADER_FG_COLOR)
                        .on_color(*SYSTEM_MESSAGE_HEADER_BG_COLOR),
                    ACTION_TAG.bold().color(*ACTION_TAG_COLOR),
                    message.get_main_content().italic().color(*ACTION_TAG_COLOR)
                );
            }
            _ => unreachable!(),
        },
    }

    Ok(())
}

fn handle_action_message(_action: ActionMessage) -> CliResult<()> {
    terminal::disable_raw_mode()?;
    Ok(())
}

async fn handle_terminal_event(
    event: Event,
    message_tx: &mpsc::UnboundedSender<String>,
) -> CliResult<()> {
    terminal::disable_raw_mode()?;

    // Ctrl+C should quit the shell
    if let Event::Key(key) = event {
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            process::exit(0);
        }
    }

    prompt(message_tx).await?;

    Ok(())
}

async fn prompt(message_tx: &mpsc::UnboundedSender<String>) -> CliResult<()> {
    // Print the prompt
    print!(
        "\n{}\n{} ",
        " User Message "
            .bold()
            .color(*USER_MESSAGE_HEADER_FG_COLOR)
            .on_color(*USER_MESSAGE_HEADER_BG_COLOR),
        ">>>".bold().color(*USER_MESSAGE_HEADER_FG_COLOR)
    );
    std::io::stdout().flush().unwrap();

    // Read the input from stdin
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();
    let line = lines.next_line().await?;

    // Send the message to the agent
    message_tx.send(line.unwrap_or_default())?;

    Ok(())
}

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

lazy_static! {
    static ref SYSTEM_MESSAGE_HEADER_BG_COLOR: Color = Color::Magenta;
    static ref SYSTEM_MESSAGE_HEADER_FG_COLOR: Color = Color::White;
    static ref USER_MESSAGE_HEADER_BG_COLOR: Color = Color::Green;
    static ref USER_MESSAGE_HEADER_FG_COLOR: Color = Color::White;
    static ref THOUGHT_TAG_COLOR: Color = Color::BrightBlack;
    static ref ACTION_TAG_COLOR: Color = Color::BrightMagenta;
}

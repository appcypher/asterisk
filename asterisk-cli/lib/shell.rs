use std::{env, io::Write, process};

use asterisk_core::{
    agents::dreamer::{channels, ActionMessage, Dreamer, Metrics, ThreadMessage},
    models::{
        openai::{ModelType, OpenAILikeModel, OpenAIModel},
        ModelResult, Prompt, TextModel,
    },
    utils::{self, Env},
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

use crate::{CliError, CliResult};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

#[derive(Clone)]
pub enum Model {
    OpenAIModel(OpenAIModel),
    OpenAILikeModel(OpenAILikeModel),
}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// Runs the shell.
pub async fn run() -> CliResult<()> {
    utils::load_env(Env::Dev);

    // Create the model behind the agent.
    let agent = select_agent()?;

    println!(
        "\n{}",
        " dreamer agent initialized "
            .bold()
            .color(*SYSTEM_MESSAGE_HEADER_FG_COLOR)
            .on_color(*SYSTEM_MESSAGE_HEADER_BG_COLOR)
    );

    // Create channels for the agent and external communication
    let (agent_channels, mut external_channels) = channels::create();

    // Spawn the agent in a new task
    let mut handle = agent.run(agent_channels);

    // Prompt and send the first message
    prompt_and_send(&external_channels.message_tx).await?;

    // Handle agent actions, metrics
    let mut handle = tokio::spawn(async move {
        loop {
            tokio::select! {
                result = &mut handle => {
                    terminal::disable_raw_mode()?;
                    match result {
                        Ok(r) => if let Err(e) = r {
                            return Err(CliError::DreamerError(e));
                        },
                        Err(e) => {
                            return Err(CliError::JoinError(e));
                        }
                    }

                    break;
                }
                metrics = external_channels.metrics_rx.recv() => if let Some(metrics) = metrics {
                    handle_metric_message(metrics)?;
                },
                action = external_channels.action_rx.recv() => if let Some(action) = action {
                    handle_action_message(action)?;
                },
            }
        }

        crate::Ok(())
    });

    // Handle terminal events and agent task exit.
    let mut reader = EventStream::new();
    loop {
        terminal::enable_raw_mode()?;
        tokio::select! {
            event = reader.next() => if let Some(event) = event {
                handle_terminal_event(event?, &external_channels.message_tx).await?;
            },
            result = &mut handle => {
                terminal::disable_raw_mode()?;
                match result {
                    Ok(r) => r?,
                    Err(e) => {
                        return Err(CliError::JoinError(e));
                    }
                }

                break;
            }
        }
    }

    Ok(())
}

fn handle_metric_message(metrics: Metrics) -> CliResult<()> {
    terminal::disable_raw_mode()?;
    match metrics {
        Metrics::ThreadMessage(message) => match message {
            ThreadMessage::Thought(message) => {
                println!(
                    "\n{}\n{}",
                    " agent thought "
                        .italic()
                        .color(*SYSTEM_MESSAGE_HEADER_FG_COLOR)
                        .on_color(*THOUGHT_TAG_COLOR),
                    message
                        .get_main_content()
                        .italic()
                        .color(*THOUGHT_TAG_COLOR)
                );
            }
            ThreadMessage::Action(message) => {
                let pretty_message = jsonxf::pretty_print(message.get_main_content()).unwrap();
                println!(
                    "\n{}\n{}",
                    " agent action "
                        .italic()
                        .color(*SYSTEM_MESSAGE_HEADER_FG_COLOR)
                        .on_color(*ACTION_TAG_COLOR),
                    pretty_message.italic().color(*ACTION_TAG_COLOR)
                );
            }
            ThreadMessage::Notification(message) => {
                println!(
                    "\n{}\n{}",
                    " agent notification "
                        .italic()
                        .color(*SYSTEM_MESSAGE_HEADER_FG_COLOR)
                        .on_color(*NOTIFICATION_TAG_COLOR),
                    message
                        .get_main_content()
                        .italic()
                        .color(*NOTIFICATION_TAG_COLOR)
                );
            }
            ThreadMessage::Observation(message) => {
                println!(
                    "\n{}\n{}",
                    " agent observation "
                        .italic()
                        .color(*SYSTEM_MESSAGE_HEADER_FG_COLOR)
                        .on_color(*OBSERVATION_TAG_COLOR),
                    message
                        .get_main_content()
                        .italic()
                        .color(*OBSERVATION_TAG_COLOR)
                );
            }
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

    // Prompt and send the message
    prompt_and_send(message_tx).await?;

    Ok(())
}

async fn prompt_and_send(message_tx: &mpsc::UnboundedSender<String>) -> CliResult<()> {
    // Print the prompt
    print!(
        "\n{}\n{} ",
        " user message "
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

fn select_agent() -> CliResult<Dreamer<Model>> {
    println!(
        "{}\n",
        " choose a model: "
            .bold()
            .color(*SYSTEM_MESSAGE_HEADER_FG_COLOR)
            .on_color(*SYSTEM_MESSAGE_HEADER_BG_COLOR)
    );
    println!("{} gpt-4o", " 1.".bold().black().on_white());
    println!("{} gpt-4o-mini", " 2.".bold().black().on_white());
    println!(
        "{} llama-3-1-8b (together)",
        " 3.".bold().black().on_white()
    );
    println!(
        "{} llama-3-1-70b (together)",
        " 4.".bold().black().on_white()
    );
    print!(">>> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let model: Model = match input.trim() {
        "1" => Model::OpenAIModel(
            OpenAIModel::builder()
                .seed(0)
                .model(ModelType::Gpt4o_2024_08_06)
                .build(),
        ),
        "" | "2" => Model::OpenAIModel(
            OpenAIModel::builder()
                .seed(0)
                .model(ModelType::Gpt4oMini_2024_07_18)
                .build(),
        ),
        "3" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("TOGETHER_API_KEY").unwrap())
                .base_url(TOGETHER_URL)
                .seed(0)
                .model(TOGETHER_LLAMA_3_1_8B_MODEL)
                .build(),
        ),
        "4" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("TOGETHER_API_KEY").unwrap())
                .base_url(TOGETHER_URL)
                .seed(0)
                .model(TOGETHER_LLAMA_3_1_70B_MODEL)
                .build(),
        ),
        _ => return Err(CliError::InvalidModel(input.trim().to_string())),
    };

    println!(
        "\n{}{}",
        (String::from(" ") + &model.get_model())
            .bold()
            .color(*SYSTEM_MESSAGE_HEADER_FG_COLOR)
            .on_color(*SYSTEM_MESSAGE_HEADER_BG_COLOR),
        " selected "
            .bold()
            .color(*SYSTEM_MESSAGE_HEADER_FG_COLOR)
            .on_color(*SYSTEM_MESSAGE_HEADER_BG_COLOR)
    );

    let agent = Dreamer::builder().model(model).build();

    Ok(agent)
}

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

lazy_static! {
    static ref SYSTEM_MESSAGE_HEADER_BG_COLOR: Color = Color::BrightMagenta;
    static ref SYSTEM_MESSAGE_HEADER_FG_COLOR: Color = Color::Black;
    static ref USER_MESSAGE_HEADER_BG_COLOR: Color = Color::Green;
    static ref USER_MESSAGE_HEADER_FG_COLOR: Color = Color::Black;
    static ref THOUGHT_TAG_COLOR: Color = Color::BrightBlack;
    static ref ACTION_TAG_COLOR: Color = Color::BrightCyan;
    static ref NOTIFICATION_TAG_COLOR: Color = Color::BrightYellow;
    static ref OBSERVATION_TAG_COLOR: Color = Color::BrightGreen;
}

// const FIREWORKS_URL: &str = "https://api.fireworks.ai/v1/chat/completions";
// const FIREWORKS_LLAMA_3_1_8B_MODEL: &str = "accounts/fireworks/models/llama-v3p1-8b-instruct";
// const FIREWORKS_LLAMA_3_1_70B_MODEL: &str = "accounts/fireworks/models/llama-v3p1-70b-instruct";

const TOGETHER_URL: &str = "https://api.together.xyz/v1/chat/completions";
const TOGETHER_LLAMA_3_1_8B_MODEL: &str = "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo";
const TOGETHER_LLAMA_3_1_70B_MODEL: &str = "meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo";

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl Model {
    pub fn get_model(&self) -> String {
        match self {
            Model::OpenAIModel(model) => model.get_config().model.clone(),
            Model::OpenAILikeModel(model) => model.get_config().model.clone(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl TextModel for Model {
    async fn prompt(&self, prompt: impl Into<Prompt> + Send) -> ModelResult<String> {
        match self {
            Model::OpenAIModel(model) => model.prompt(prompt).await,
            Model::OpenAILikeModel(model) => model.prompt(prompt).await,
        }
    }
}

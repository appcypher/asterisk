use std::{env, io::Write};

use asterisk_core::{
    models::{
        openai::{ModelType, OpenAILikeModel, OpenAIModel},
        ModelError, ModelResult, Prompt, PromptMessage, TextStreamModel,
    },
    utils::{self, Env},
};
use colored::Colorize;
use futures::StreamExt;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const TOGETHER_URL: &str = "https://api.together.xyz/v1/chat/completions";
const TOGETHER_LLAMA_3_1_8B_MODEL: &str = "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo";

const FIREWORKS_URL: &str = "https://api.fireworks.ai/inference/v1/chat/completions";
const FIREWORKS_LLAMA_3_1_8B_MODEL: &str = "accounts/fireworks/models/llama-v3p1-8b-instruct";

const GROQ_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const GROQ_LLAMA_3_8B_MODEL: &str = "llama3-8b-8192";

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

enum Model {
    OpenAIModel(OpenAIModel),
    OpenAILikeModel(OpenAILikeModel),
}

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

#[tokio::main(flavor = "current_thread")]
async fn main() -> ModelResult<()> {
    utils::load_env(Env::Dev);
    tracing_subscriber::fmt::init();

    println!("{}\n", " choose a model: ".bold().black().on_bright_cyan());
    println!("{} gpt-4o", " 1.".bold().black().on_white());
    println!("{} gpt-4o-mini", " 2.".bold().black().on_white());
    println!(
        "{} llama-3-1-8b (together)",
        " 3.".bold().black().on_white()
    );
    println!(
        "{} llama-3-1-8b (fireworks)",
        " 4.".bold().black().on_white()
    );
    println!("{} llama-3-8b (groq)", " 5.".bold().black().on_white());
    print!(">>> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let model: Model = match input.trim() {
        "1" => Model::OpenAIModel(
            OpenAIModel::builder()
                .model(ModelType::Gpt4o_2024_08_06)
                .temperature(0.)
                .build(),
        ),
        "" | "2" => Model::OpenAIModel(
            OpenAIModel::builder()
                .model(ModelType::Gpt4oMini_2024_07_18)
                .temperature(0.)
                .build(),
        ),
        "3" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("TOGETHER_API_KEY").unwrap())
                .base_url(TOGETHER_URL)
                .model(TOGETHER_LLAMA_3_1_8B_MODEL)
                .temperature(0.)
                .build(),
        ),
        "4" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("FIREWORKS_API_KEY").unwrap())
                .base_url(FIREWORKS_URL)
                .model(FIREWORKS_LLAMA_3_1_8B_MODEL)
                .temperature(0.)
                .build(),
        ),
        "5" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("GROQ_API_KEY").unwrap())
                .base_url(GROQ_URL)
                .model(GROQ_LLAMA_3_8B_MODEL)
                .temperature(0.)
                .build(),
        ),
        _ => return Err(ModelError::custom(anyhow::anyhow!("invalid model"))),
    };

    println!(
        "\n{}{}",
        model.get_name().italic().dimmed(),
        " selected".italic().dimmed()
    );

    let mut prompt = Prompt::new();
    prompt.add_message(PromptMessage::system("You are a helpful assistant."));

    loop {
        println!("\n{}", " user: ".bold().black().on_bright_green());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        prompt.add_message(PromptMessage::user(input.clone()));
        let mut output = match &model {
            Model::OpenAIModel(model) => model.prompt_stream(prompt.clone()).await?,
            Model::OpenAILikeModel(model) => model.prompt_stream(prompt.clone()).await?,
        };

        println!("\n{}", " assistant: ".bold().black().on_bright_cyan());

        let mut response = String::new();
        while let Some(chunk) = output.next().await {
            let chunk = chunk?;
            response.push_str(&chunk);
            std::io::stdout().write_all(&chunk.as_bytes()).unwrap();
        }

        prompt.add_message(PromptMessage::assistant(response));
        println!();
    }
}

//-------------------------------------------------------------------------------------------------
// Methods
//-------------------------------------------------------------------------------------------------

impl Model {
    fn get_name(&self) -> String {
        match self {
            Model::OpenAIModel(model) => model.get_config().model.to_string(),
            Model::OpenAILikeModel(model) => model.get_config().model.to_string(),
        }
    }
}

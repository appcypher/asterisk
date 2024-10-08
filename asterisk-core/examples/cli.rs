use std::{env, io::Write};

use asterisk_core::{
    models::{
        ollama::{self, OllamaModel},
        openai::{self, OpenAILikeModel, OpenAIModel},
        ModelError, ModelResult, Prompt, PromptMessage, TextStreamModel,
    },
    utils::{self, Env},
};
use colored::Colorize;
use futures::{stream::BoxStream, StreamExt};

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const TOGETHER_URL: &str = "https://api.together.xyz/v1/chat/completions";
const TOGETHER_LLAMA_3_1_8B_MODEL: &str = "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo";

const FIREWORKS_URL: &str = "https://api.fireworks.ai/inference/v1/chat/completions";
const FIREWORKS_LLAMA_3_1_8B_MODEL: &str = "accounts/fireworks/models/llama-v3p1-8b-instruct";
const FIREWORKS_LLAMA_3_1_70B_MODEL: &str = "accounts/fireworks/models/llama-v3p1-70b-instruct";

const GROQ_URL: &str = "https://api.groq.com/openai/v1/chat/completions";
const GROQ_LLAMA_3_8B_MODEL: &str = "llama3-8b-8192";

const SAMBA_NOVA_URL: &str = "https://api.sambanova.ai/v1/chat/completions";
const SAMBA_NOVA_LLAMA_3_1_8B_MODEL: &str = "Meta-Llama-3.1-8B-Instruct";

const CEREBRAS_URL: &str = "https://api.cerebras.ai/v1/chat/completions";
const CEREBRAS_LLAMA_3_1_8B_MODEL: &str = "llama3.1-8b";

//-------------------------------------------------------------------------------------------------
// Types
//-------------------------------------------------------------------------------------------------

enum Model {
    OpenAIModel(OpenAIModel),
    OpenAILikeModel(OpenAILikeModel),
    OllamaModel(OllamaModel),
}

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

#[tokio::main]
async fn main() -> ModelResult<()> {
    utils::load_env(Env::Dev);
    tracing_subscriber::fmt::init();

    println!("{}\n", " choose a model: ".bold().black().on_bright_cyan());
    println!("{} gpt-4o", " 1.".bold().black().on_white());
    println!("{} gpt-4o-mini", " 2.".bold().black().on_white());
    println!(
        "{} llama-3-1-8b (fireworks)",
        " 3.".bold().black().on_white()
    );
    println!(
        "{} llama-3-1-8b (sambanova)",
        " 4.".bold().black().on_white()
    );
    println!(
        "{} llama-3-1-8b (cerebras)",
        " 5.".bold().black().on_white()
    );
    println!(
        "{} llama-3-1-8b (together)",
        " 6.".bold().black().on_white()
    );
    println!("{} llama-3-8b (groq)", " 7.".bold().black().on_white());
    println!("{} llama-3-1-8b (ollama)", " 8.".bold().black().on_white());
    println!(
        "{} llama-3-1-70b (fireworks)",
        " 9.".bold().black().on_white()
    );
    print!(">>> ");
    std::io::stdout().flush().unwrap();

    // Check if there is a --no-history flag
    let args: Vec<String> = env::args().collect();
    let no_history = args.iter().any(|arg| arg == "--no-history");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let model: Model = match input.trim() {
        "1" => Model::OpenAIModel(
            OpenAIModel::builder()
                .model(openai::ModelType::Gpt4o_2024_08_06)
                .temperature(0.)
                .build(),
        ),
        "" | "2" => Model::OpenAIModel(
            OpenAIModel::builder()
                .model(openai::ModelType::Gpt4oMini_2024_07_18)
                .temperature(0.)
                .build(),
        ),
        "3" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("FIREWORKS_API_KEY").unwrap())
                .base_url(FIREWORKS_URL)
                .model(FIREWORKS_LLAMA_3_1_8B_MODEL)
                .temperature(0.)
                .build(),
        ),
        "4" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("SAMBA_NOVA_API_KEY").unwrap())
                .base_url(SAMBA_NOVA_URL)
                .model(SAMBA_NOVA_LLAMA_3_1_8B_MODEL)
                .temperature(0.)
                .build(),
        ),
        "5" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("CEREBRAS_API_KEY").unwrap())
                .base_url(CEREBRAS_URL)
                .model(CEREBRAS_LLAMA_3_1_8B_MODEL)
                .temperature(0.)
                .build(),
        ),
        "6" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("TOGETHER_API_KEY").unwrap())
                .base_url(TOGETHER_URL)
                .model(TOGETHER_LLAMA_3_1_8B_MODEL)
                .temperature(0.)
                .build(),
        ),
        "7" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("GROQ_API_KEY").unwrap())
                .base_url(GROQ_URL)
                .model(GROQ_LLAMA_3_8B_MODEL)
                .temperature(0.)
                .build(),
        ),
        "8" => Model::OllamaModel(
            OllamaModel::builder()
                .model(ollama::ModelType::Llama3_1_8B)
                .temperature(0.)
                .build(),
        ),
        "9" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("FIREWORKS_API_KEY").unwrap())
                .base_url(FIREWORKS_URL)
                .model(FIREWORKS_LLAMA_3_1_70B_MODEL)
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

        prompt.push(PromptMessage::user(input.clone()));
        let mut output = model.prompt_stream(prompt.clone()).await?;

        println!("\n{}", " assistant: ".bold().black().on_bright_cyan());

        let mut response = String::new();
        while let Some(chunk) = output.next().await {
            let chunk = chunk?;
            response.push_str(&chunk);
            std::io::stdout().write_all(&chunk.as_bytes()).unwrap();
        }
        println!();

        if no_history {
            prompt.pop();
        } else {
            prompt.push(PromptMessage::assistant(response));
        }
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
            Model::OllamaModel(model) => model.get_config().model.to_string(),
        }
    }
}

//-------------------------------------------------------------------------------------------------
// Trait Implementations
//-------------------------------------------------------------------------------------------------

impl TextStreamModel for Model {
    async fn prompt_stream(
        &self,
        prompt: impl Into<Prompt> + Send,
    ) -> ModelResult<BoxStream<'static, ModelResult<String>>> {
        match self {
            Model::OpenAIModel(model) => model.prompt_stream(prompt).await,
            Model::OpenAILikeModel(model) => model.prompt_stream(prompt).await,
            Model::OllamaModel(model) => model.prompt_stream(prompt).await,
        }
    }
}

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

const LLAMA_3_1_8B_MODEL: &str = "meta-llama/Meta-Llama-3.1-8B-Instruct-Turbo";
const TOGETHER_URL: &str = "https://api.together.xyz/v1/chat/completions";

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
    println!(" {} {}", "1.".bold().black().on_white(), "gpt-4o");
    println!(" {} {}", "2.".bold().black().on_white(), "gpt-4o-mini");
    println!(" {} {}", "3.".bold().black().on_white(), "llama-3-1-8b");
    print!(" > ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let model: Model = match input.trim() {
        "1" => Model::OpenAIModel(
            OpenAIModel::builder()
                .model(ModelType::Gpt4o_2024_08_06)
                .build(),
        ),
        "2" => Model::OpenAIModel(
            OpenAIModel::builder()
                .model(ModelType::Gpt4oMini_2024_07_18)
                .build(),
        ),
        "3" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("TOGETHER_API_KEY").unwrap())
                .base_url(TOGETHER_URL)
                .model(LLAMA_3_1_8B_MODEL)
                .build(),
        ),
        _ => return Err(ModelError::custom(anyhow::anyhow!("invalid model"))),
    };

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

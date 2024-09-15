use std::{
    env,
    io::{BufRead, BufReader, Write},
};

use asterisk_core::{
    models::{
        openai::{self, OpenAILikeModel, OpenAIModel},
        ModelError, ModelResult, Prompt, PromptMessage, TextStreamModel,
    },
    utils::{self, Env},
};
use colored::Colorize;
use futures::{stream::BoxStream, StreamExt};
use regex::RegexBuilder;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

const FIREWORKS_URL: &str = "https://api.fireworks.ai/inference/v1/chat/completions";
const FIREWORKS_LLAMA_3_1_8B_MODEL: &str = "accounts/fireworks/models/llama-v3p1-8b-instruct";
const FIREWORKS_LLAMA_3_1_70B_MODEL: &str = "accounts/fireworks/models/llama-v3p1-70b-instruct";

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
        "{} llama-3-1-70b (fireworks)",
        " 4.".bold().black().on_white()
    );
    print!(">>> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let model: Model = match input.trim() {
        "1" => Model::OpenAIModel(
            OpenAIModel::builder()
                .model(openai::ModelType::Gpt4o_2024_08_06)
                .temperature(0.)
                .build(),
        ),
        "2" => Model::OpenAIModel(
            OpenAIModel::builder()
                .model(openai::ModelType::Gpt4oMini_2024_07_18)
                .temperature(0.)
                .build(),
        ),
        "" | "3" => Model::OpenAILikeModel(
            OpenAILikeModel::builder()
                .api_key(env::var("FIREWORKS_API_KEY").unwrap())
                .base_url(FIREWORKS_URL)
                .model(FIREWORKS_LLAMA_3_1_8B_MODEL)
                .temperature(0.)
                .build(),
        ),
        "4" => Model::OpenAILikeModel(
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

    loop {
        println!("\n{}", " messages: ".bold().black().on_bright_green());

        let mut input = String::new();

        let stdin = std::io::stdin();
        let mut reader = BufReader::new(stdin).lines();

        let mut count = 0;
        while let Some(line) = reader.next() {
            let line = line.unwrap();
            if line.is_empty() {
                if count < 1 {
                    count += 1;
                    continue;
                }
                break;
            }
            input.push_str(&line);
            input.push('\n');
        }

        let prompt = parse_prompt(&input);
        let mut output = model.prompt_stream(prompt).await?;

        println!("\n{}", " assistant: ".bold().black().on_bright_cyan());

        let mut response = String::new();
        while let Some(chunk) = output.next().await {
            let chunk = chunk?;
            response.push_str(&chunk);
            std::io::stdout().write_all(&chunk.as_bytes()).unwrap();
        }
        println!();
    }
}

//-------------------------------------------------------------------------------------------------
// Functions
//-------------------------------------------------------------------------------------------------

fn parse_prompt(mut prompt: &str) -> Prompt {
    let mut prompt_builder = Prompt::new();

    // Match the first system message
    let system_pattern = RegexBuilder::new(r"^\s*system:\s*((.+?\n*)+?)(user:|assistant:|$)")
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    let system_pattern = system_pattern.captures(prompt);
    if let Some(Some(system_match)) = system_pattern.map(|m| m.get(1)) {
        prompt_builder.push(PromptMessage::system(system_match.as_str().trim()));
        prompt = &prompt[system_match.end()..];
    }

    // Match all user and assistant messages
    let user_pattern = RegexBuilder::new(r"^user:\s*((.+?\n*)+?)(assistant:|user:|$)")
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    let assistant_pattern = RegexBuilder::new(r"^assistant:\s*((.+?\n*)+?)(user:|assistant:|$)")
        .dot_matches_new_line(true)
        .build()
        .unwrap();

    loop {
        if let Some(Some(user_match)) = user_pattern.captures(prompt).map(|m| m.get(1)) {
            prompt_builder.push(PromptMessage::user(user_match.as_str().trim()));
            prompt = &prompt[user_match.end()..];
        } else if let Some(Some(assistant_match)) =
            assistant_pattern.captures(prompt).map(|m| m.get(1))
        {
            prompt_builder.push(PromptMessage::assistant(assistant_match.as_str().trim()));
            prompt = &prompt[assistant_match.end()..];
        } else {
            break;
        }
    }

    prompt_builder
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
        }
    }
}

//-------------------------------------------------------------------------------------------------
// Tests
//-------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_prompt() {
        let prompt = "\nsystem: \nYou are a helpful assistant.\n\nuser:\nHello, how are you?\n\nassistant: \nI am fine, thank you!";
        let mut expected = Prompt::new();
        expected.push(PromptMessage::system("You are a helpful assistant."));
        expected.push(PromptMessage::user("Hello, how are you?"));
        expected.push(PromptMessage::assistant("I am fine, thank you!"));

        let parsed = parse_prompt(prompt);

        assert_eq!(expected, parsed);

        let prompt = r#"
system:
Always break a user's sentence into independent ideas and conjuctions using predicate logic.
List them out as bulletpoints without any other information

user:
Alice has N brothers and she also has M sisters.

assistant:
- Alice has N brothers
- and
- she also has M sisters

user:
How many sisters does Alice’s brother have?"#;

        let mut expected = Prompt::new();
        expected.push(PromptMessage::system("Always break a user's sentence into independent ideas and conjuctions using predicate logic.\nList them out as bulletpoints without any other information"));
        expected.push(PromptMessage::user(
            "Alice has N brothers and she also has M sisters.",
        ));
        expected.push(PromptMessage::assistant(
            "- Alice has N brothers\n- and\n- she also has M sisters",
        ));
        expected.push(PromptMessage::user(
            "How many sisters does Alice’s brother have?",
        ));

        let parsed = parse_prompt(prompt);

        assert_eq!(expected, parsed);
    }
}

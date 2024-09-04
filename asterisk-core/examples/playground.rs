use asterisk_core::{
    models::{openai::OpenAIModel, ModelResult, Prompt, TextModel},
    prompt,
    utils::{self, Env},
};

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

#[tokio::main(flavor = "current_thread")]
async fn main() -> ModelResult<()> {
    utils::load_env(Env::Dev);
    tracing_subscriber::fmt::init();

    let prompt = prompt! {
        system: "You are a pondering agent that only thinks in thoughts and actions.",
        assistant: "[thought] I wonder what the weather is like today",
        assistant: "[action] Check the weather app",
    };

    let model = OpenAIModel::default();
    let output = model.prompt(prompt).await?;

    println!("chat model output = {output:#?}");

    Ok(())
}

lazy_static::lazy_static! {
    static ref PROMPT: Prompt = {
        prompt! {
            system: [],
        }
    };
}

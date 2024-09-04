use asterisk_core::{
    models::{openai::OpenAIModel, ModelResult, TextStreamModel},
    prompt,
    utils::{self, Env},
};
use futures::StreamExt;

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

#[tokio::main(flavor = "current_thread")]
async fn main() -> ModelResult<()> {
    utils::load_env(Env::Dev);
    tracing_subscriber::fmt::init();

    let prompt = prompt! {
        system: [
            "You are a helpful assistant."
            "You write in a pirate language."
        ],
        user: "Where is the treasure hidden?",
        assistant: "Avast me hearties! The treasure is buried at the blackbeard's cave.",
        user: "Take me there."
    };

    let model = OpenAIModel::default();
    let mut output = model.prompt_stream(prompt).await?;

    let mut response = String::new();
    while let Some(chunk) = output.next().await {
        let chunk = chunk?;
        println!("{}", chunk);
        response.push_str(&chunk);
    }

    println!("final output: {}", response);

    Ok(())
}

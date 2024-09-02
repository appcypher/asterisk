use asterisk_core::{
    models::{openai::OpenAIModel, ModelResult, TextModel},
    prompt,
    utils::{self, Env},
};

//-------------------------------------------------------------------------------------------------
// Main
//-------------------------------------------------------------------------------------------------

#[tokio::main(flavor = "current_thread")]
async fn main() -> ModelResult<()> {
    utils::load_env(Env::Prod);
    env_logger::init();

    let prompt = prompt! {
        system: "Classify the text into neutral, negative or positive.",
        user: "I think the vacation is okay.",
        assistant: "neutral",
        user: "I was not happy with the service."
    };

    let model = OpenAIModel::default();
    let output: String = model.prompt(prompt).await?;

    println!("chat model output = {output:#?}");

    Ok(())
}

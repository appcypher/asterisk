use futures::{stream::BoxStream, Future};

use super::{ModelResult, Prompt};

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// A trait for models that can be used to generate text.
pub trait TextModel {
    /// Sends messages to the model and gets a response back.
    fn prompt(&self, prompt: Prompt) -> impl Future<Output = ModelResult<String>>;
}

/// A trait for models that can be used to generate text streams.
pub trait TextStreamModel {
    /// Sends messages to the model and gets back a stream of strings as response.
    fn prompt_stream(
        &self,
        prompt: Prompt,
    ) -> impl Future<Output = ModelResult<BoxStream<'static, String>>>;
}

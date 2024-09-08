use std::borrow::Cow;

use futures::stream::BoxStream;
use tracing::debug;

use crate::models::{
    ollama::{StreamOptions, OLLAMA_API_URL},
    ModelError, ModelResult, Prompt, TextModel, TextStreamModel,
};

use super::{
    Config, ModelBuilder, RequestBody, RequestMessages, ResponseBody, ResponseOk, ResponseStream,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `OllamaModel` is similar to `OpenAIModel` but not enough to be interchangeable.
#[derive(Debug, Clone)]
pub struct OllamaModel {
    pub(crate) config: Cow<'static, Config>,
    pub(crate) base_url: String,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl OllamaModel {
    /// Creates a builder for the model.
    pub fn builder() -> ModelBuilder {
        ModelBuilder::default()
    }

    /// Calls the API with the given request messages.
    pub async fn call(&self, messages: impl Into<RequestMessages>) -> ModelResult<ResponseOk> {
        let config = self.get_config_without_streaming();
        let request = reqwest::Client::new()
            .post(&self.base_url)
            .json(&RequestBody {
                messages: messages.into(),
                config: config.into_owned(),
            });

        let response = request.send().await?;
        let body = response.text().await?;
        debug!("body = {body:#?}");
        let body: ResponseBody = serde_json::from_str(&body)?;
        let ResponseBody::Ok(body) = body else {
            return Err(ModelError::OllamaResponseError(body.unwrap_err()));
        };

        Ok(body)
    }

    /// Calls the API with the given request messages and gets back a stream of response chunks.
    pub fn call_streaming(
        &self,
        messages: impl Into<RequestMessages>,
    ) -> ModelResult<ResponseStream> {
        let config = self.get_config_with_streaming(None);
        debug!("config = {config:#?}");
        let request = reqwest::Client::new()
            .post(&self.base_url)
            .json(&RequestBody {
                messages: messages.into(),
                config: config.into_owned(),
            });

        Ok(ResponseStream::new(request))
    }

    /// Gets the model's configuration with streaming enabled.
    fn get_config_with_streaming(&self, options: Option<StreamOptions>) -> Cow<Config> {
        let mut config = Cow::Borrowed(self.config.as_ref());

        if self.config.stream.is_none() {
            config.to_mut().stream = Some(true);
        }

        if let Some(options) = options {
            config.to_mut().stream_options = Some(options);
        }

        config
    }

    /// Gets the model's configuration without streaming enabled.
    fn get_config_without_streaming(&self) -> Cow<Config> {
        let mut config = Cow::Borrowed(self.config.as_ref());

        if self.config.stream.is_some() {
            config.to_mut().stream = Some(false);
        }

        config
    }

    /// Extract main content from response
    pub(crate) fn extract_content_from_response(response: &ResponseOk) -> String {
        debug!("response = {response:#?}");
        response.message.content.clone().unwrap_or_default()
    }

    /// Extract main content from response chunk
    pub(crate) fn extract_content_from_response_chunk(response: &ResponseOk) -> String {
        response.message.content.clone().unwrap_or_default()
    }

    /// Get the model's configuration
    pub fn get_config(&self) -> &Config {
        &self.config
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl TextModel for OllamaModel {
    async fn prompt(&self, prompt: impl Into<Prompt> + Send) -> ModelResult<String> {
        let response = self.call(prompt.into()).await?;
        let content = Self::extract_content_from_response(&response);
        Ok(content)
    }
}

impl TextStreamModel for OllamaModel {
    async fn prompt_stream(
        &self,
        prompt: impl Into<Prompt> + Send,
    ) -> ModelResult<BoxStream<'static, ModelResult<String>>> {
        let stream = self.call_streaming(prompt.into())?;
        Ok(Box::pin(stream))
    }
}

impl Default for OllamaModel {
    fn default() -> Self {
        Self {
            config: Cow::Owned(Config::default()),
            base_url: OLLAMA_API_URL.to_string(),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{
        models::ollama::ModelType,
        utils::{self, Env},
    };

    use super::*;

    #[test]
    fn test_model_ollama_default() {
        utils::load_env(Env::Dev);
        let model = OllamaModel::default();

        assert_eq!(model.base_url, OLLAMA_API_URL.to_string());
        assert_eq!(model.config.model, ModelType::Llama3_1_8B.to_string());
        assert_eq!(model.config.frequency_penalty, None);
        assert_eq!(model.config.logit_bias, None);
        assert_eq!(model.config.logprobs, None);
        assert_eq!(model.config.top_logprobs, None);
        assert_eq!(model.config.max_tokens, None);
        assert_eq!(model.config.n, None);
        assert_eq!(model.config.presence_penalty, None);
        assert_eq!(model.config.response_format, None);
        assert_eq!(model.config.seed, None);
        assert_eq!(model.config.service_tier, None);
        assert_eq!(model.config.stop, None);
        assert_eq!(model.config.stream, None);
        assert_eq!(model.config.stream_options, None);
        assert_eq!(model.config.temperature, None);
        assert_eq!(model.config.top_p, None);
        assert_eq!(model.config.tools, None);
        assert_eq!(model.config.tool_choice, None);
        assert_eq!(model.config.parallel_tool_calls, None);
        assert_eq!(model.config.user, None);
    }

    #[test]
    fn test_model_ollama_builders() {
        utils::load_env(Env::Dev);
        let model = OllamaModel::builder().build();

        assert_eq!(model.base_url, OLLAMA_API_URL.to_string());
        assert_eq!(model.config.model, ModelType::Llama3_1_8B.to_string());
        assert_eq!(model.config.frequency_penalty, None);
        assert_eq!(model.config.logit_bias, None);
        assert_eq!(model.config.logprobs, None);
        assert_eq!(model.config.top_logprobs, None);
        assert_eq!(model.config.max_tokens, None);
        assert_eq!(model.config.n, None);
        assert_eq!(model.config.presence_penalty, None);
        assert_eq!(model.config.response_format, None);
        assert_eq!(model.config.seed, None);
        assert_eq!(model.config.service_tier, None);
        assert_eq!(model.config.stop, None);
        assert_eq!(model.config.stream, None);
        assert_eq!(model.config.stream_options, None);
        assert_eq!(model.config.temperature, None);
        assert_eq!(model.config.top_p, None);
        assert_eq!(model.config.tools, None);
        assert_eq!(model.config.tool_choice, None);
        assert_eq!(model.config.parallel_tool_calls, None);
        assert_eq!(model.config.user, None);
    }
}

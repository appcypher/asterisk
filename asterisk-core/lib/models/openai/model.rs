use std::{borrow::Cow, ops::Deref};

use futures::stream::BoxStream;
use tracing::debug;

use crate::models::{
    openai::{StreamOptions, OPENAI_API_URL},
    ModelError, ModelResult, Prompt, TextModel, TextStreamModel,
};

use super::{
    Config, ModelBuilder, RequestBody, RequestMessages, ResponseBody, ResponseChunkOk, ResponseOk,
    ResponseStream,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `OpenAIModel` is a type that can prompt and stream responses from models provided by OpenAI.
#[derive(Debug, Clone)]
pub struct OpenAIModel {
    pub(crate) config: Cow<'static, Config>,
    pub(crate) base_url: String,
}

/// `OpenAILikeModel` is a type that can prompt and stream responses from models that are compatible
/// with the OpenAI API.
#[derive(Debug, Clone)]
pub struct OpenAILikeModel(pub(crate) OpenAIModel);

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl OpenAIModel {
    /// Creates a builder for the model.
    pub fn builder() -> ModelBuilder<(), ()> {
        ModelBuilder::default()
    }

    /// Calls the API with the given request messages.
    pub async fn call(&self, messages: impl Into<RequestMessages>) -> ModelResult<ResponseOk> {
        let config = self.get_config_without_streaming();
        debug!("config = {config:#?}");

        let messages = messages.into();
        debug!("messages = {}", serde_json::to_string(&messages).unwrap());

        let request = reqwest::Client::new()
            .post(&self.base_url)
            .timeout(std::time::Duration::from_secs(60))
            .bearer_auth(config.api_key.as_ref().ok_or(ModelError::NoAPIKeyFound)?)
            .json(&RequestBody {
                messages,
                config: config.into_owned(),
            });

        let response = request.send().await?;

        let body = response.text().await?;
        debug!("body = {body:#?}");

        let body: ResponseBody = serde_json::from_str(&body)?;
        let ResponseBody::Ok(body) = body else {
            return Err(ModelError::OpenAIResponseError(body.unwrap_err()));
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

        let messages = messages.into();
        debug!("messages = {}", serde_json::to_string(&messages).unwrap());

        let request = reqwest::Client::new()
            .post(&self.base_url)
            .bearer_auth(config.api_key.as_ref().ok_or(ModelError::NoAPIKeyFound)?)
            .json(&RequestBody {
                messages,
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
        response.choices[0]
            .message
            .content
            .clone()
            .unwrap_or_default()
    }

    /// Extract main content from response chunk
    pub(crate) fn extract_content_from_response_chunk(response: &ResponseChunkOk) -> String {
        response.choices[0]
            .delta
            .content
            .clone()
            .unwrap_or_default()
    }

    /// Get the model's configuration
    pub fn get_config(&self) -> &Config {
        &self.config
    }
}

impl OpenAILikeModel {
    /// Creates a builder for the model.
    pub fn builder() -> ModelBuilder<(), ()> {
        ModelBuilder::default()
    }

    /// Creates a new `OpenAILikeModel` with the given base URL and default config.
    pub fn new(base_url: String) -> Self {
        Self(OpenAIModel {
            config: Cow::Owned(Config::default()),
            base_url,
        })
    }

    /// Get the model's configuration
    pub fn get_config(&self) -> &Config {
        &self.0.config
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl TextModel for OpenAIModel {
    async fn prompt(&self, prompt: impl Into<Prompt> + Send) -> ModelResult<String> {
        let response = self.call(prompt.into()).await?;
        let content = Self::extract_content_from_response(&response);
        Ok(content)
    }
}

impl TextStreamModel for OpenAIModel {
    async fn prompt_stream(
        &self,
        prompt: impl Into<Prompt> + Send,
    ) -> ModelResult<BoxStream<'static, ModelResult<String>>> {
        let stream = self.call_streaming(prompt.into())?;
        Ok(Box::pin(stream))
    }
}

impl TextModel for OpenAILikeModel {
    async fn prompt(&self, prompt: impl Into<Prompt> + Send) -> ModelResult<String> {
        self.0.prompt(prompt).await
    }
}

impl TextStreamModel for OpenAILikeModel {
    async fn prompt_stream(
        &self,
        prompt: impl Into<Prompt> + Send,
    ) -> ModelResult<BoxStream<'static, ModelResult<String>>> {
        self.0.prompt_stream(prompt).await
    }
}

impl Default for OpenAIModel {
    fn default() -> Self {
        Self {
            config: Cow::Owned(Config::default()),
            base_url: OPENAI_API_URL.to_string(),
        }
    }
}

impl Deref for OpenAILikeModel {
    type Target = OpenAIModel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//--------------------------------------------------------------------------------------------------
// Tests
//--------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::{
        models::openai::ModelType,
        utils::{self, Env},
    };

    use super::*;

    #[test]
    fn test_model_openai_default() {
        utils::load_env(Env::Test);
        let model = OpenAIModel::default();

        assert_eq!(model.base_url, OPENAI_API_URL.to_string());
        assert!(model.config.api_key.as_ref().unwrap().starts_with("sk-"));
        assert_eq!(model.config.model, ModelType::Gpt4oMini.to_string());
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
    fn test_model_openai_builders() {
        utils::load_env(Env::Test);
        let model = OpenAIModel::builder().build();

        assert_eq!(model.base_url, OPENAI_API_URL.to_string());
        assert!(model.config.api_key.as_ref().unwrap().starts_with("sk-"));
        assert_eq!(model.config.model, ModelType::Gpt4oMini.to_string());
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

        let url = "https://api.closedai.com/v1/chat/completions".to_string();
        let model = OpenAILikeModel::builder()
            .base_url(url.clone())
            .model("llama-3.1-405b")
            .build();

        assert_eq!(model.base_url, url);
        assert!(model.config.api_key.as_ref().unwrap().starts_with("sk-"));
        assert_eq!(model.config.model, "llama-3.1-405b".to_string());
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

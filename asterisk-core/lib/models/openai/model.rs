use std::borrow::Cow;

use futures::stream::BoxStream;

use crate::models::{
    openai::{RequestBody, StreamOptions, OPENAI_API_URL},
    ModelError, ModelResult, Prompt, TextModel, TextStreamModel,
};

use super::{Config, ResponseBody, ResponseOk};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// `OpenAIModel` is a type that can prompt and stream responses from models provided by OpenAI.
pub struct OpenAIModel {
    pub(crate) config: Cow<'static, Config>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl OpenAIModel {
    /// Creates a new `OpenAIModel` with the given configuration.
    pub fn with_config(config: Config) -> Self {
        Self {
            config: Cow::Owned(config),
        }
    }

    /// Calls the OpenAI API with the given request.
    pub async fn call(&self, request: RequestBody) -> ModelResult<ResponseOk> {
        let config = self.get_config_without_streaming();
        let request = reqwest::Client::new()
            .post(OPENAI_API_URL)
            .bearer_auth(config.api_key.as_ref().unwrap())
            .json(&request);

        let response = request.send().await?;
        let body: ResponseBody = response.json().await?;
        let ResponseBody::Ok(body) = body else {
            return Err(ModelError::OpenAIError(body.unwrap_err()));
        };

        Ok(body)
    }

    /// Gets the model's configuration with streaming enabled.
    pub fn get_config_with_streaming(&self, options: Option<StreamOptions>) -> Cow<Config> {
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
    pub fn get_config_without_streaming(&self) -> Cow<Config> {
        let mut config = Cow::Borrowed(self.config.as_ref());

        if self.config.stream.is_some() {
            config.to_mut().stream = None;
        }

        config
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl TextModel for OpenAIModel {
    async fn prompt(&self, prompt: Prompt) -> ModelResult<String> {
        let request = RequestBody {
            messages: prompt.into(),
            config: self.config.as_ref().clone(),
        };

        let response = self.call(request).await?;
        let content = response.choices[0]
            .message
            .content
            .clone()
            .unwrap_or_default();

        Ok(content)
    }
}

impl TextStreamModel for OpenAIModel {
    async fn prompt_stream(&self, _prompt: Prompt) -> ModelResult<BoxStream<'static, String>> {
        let _config = self.get_config_with_streaming(None);

        todo!()
    }
}

impl Default for OpenAIModel {
    fn default() -> Self {
        Self {
            config: Cow::Owned(Config::default()),
        }
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
        utils::load_env(Env::Dev);
        let model = OpenAIModel::default();

        assert!(model.config.api_key.as_ref().unwrap().starts_with("sk-"));
        assert_eq!(model.config.model, ModelType::Gpt4o);
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

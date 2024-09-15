use std::{borrow::Cow, collections::HashMap, env};

use super::{
    Config, ModelType, OpenAILikeModel, OpenAIModel, ResponseFormat, ServiceTier, StreamOptions,
    Tool, ToolChoice, OPENAI_API_KEY, OPENAI_API_URL,
};

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// A builder for an OpenAI model.
pub struct ModelBuilder<U, M> {
    model: M,
    base_url: U,
    api_key: Option<String>,
    frequency_penalty: Option<f32>,
    logit_bias: Option<HashMap<u64, i8>>,
    logprobs: Option<u8>,
    top_logprobs: Option<u8>,
    max_tokens: Option<u16>,
    n: Option<u8>,
    presence_penalty: Option<f32>,
    response_format: Option<ResponseFormat>,
    seed: Option<u64>,
    service_tier: Option<ServiceTier>,
    stop: Option<Vec<String>>,
    stream: Option<bool>,
    stream_options: Option<StreamOptions>,
    temperature: Option<f32>,
    top_p: Option<f32>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<ToolChoice>,
    parallel_tool_calls: Option<bool>,
    user: Option<String>,
}

/// A builder for an OpenAI model.
pub type OpenAIModelBuilder<M> = ModelBuilder<(), M>;

/// A builder for an OpenAI-like model.
pub type OpenAILikeModelBuilder<M> = ModelBuilder<String, M>;

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl<U, M> ModelBuilder<U, M> {
    /// The ID of the model to use.
    ///
    /// Defaults to `ModelType::Gpt4oMini`.
    pub fn model(self, model: impl Into<String>) -> ModelBuilder<U, String> {
        ModelBuilder {
            model: model.into(),
            base_url: self.base_url,
            api_key: self.api_key,
            frequency_penalty: self.frequency_penalty,
            logit_bias: self.logit_bias,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
            max_tokens: self.max_tokens,
            n: self.n,
            presence_penalty: self.presence_penalty,
            response_format: self.response_format,
            seed: self.seed,
            service_tier: self.service_tier,
            stop: self.stop,
            stream: self.stream,
            stream_options: self.stream_options,
            temperature: self.temperature,
            top_p: self.top_p,
            tools: self.tools,
            tool_choice: self.tool_choice,
            parallel_tool_calls: self.parallel_tool_calls,
            user: self.user,
        }
    }

    /// The base URL for making requests to the OpenAI-like API.
    pub fn base_url(self, base_url: impl Into<String>) -> OpenAILikeModelBuilder<M> {
        ModelBuilder {
            base_url: base_url.into(),
            model: self.model,
            api_key: self.api_key,
            frequency_penalty: self.frequency_penalty,
            logit_bias: self.logit_bias,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
            max_tokens: self.max_tokens,
            n: self.n,
            presence_penalty: self.presence_penalty,
            response_format: self.response_format,
            seed: self.seed,
            service_tier: self.service_tier,
            stop: self.stop,
            stream: self.stream,
            stream_options: self.stream_options,
            temperature: self.temperature,
            top_p: self.top_p,
            tools: self.tools,
            tool_choice: self.tool_choice,
            parallel_tool_calls: self.parallel_tool_calls,
            user: self.user,
        }
    }

    /// The API key for making requests to the OpenAI API.
    ///
    /// Defaults to the value of the `OPENAI_API_KEY` environment variable if set.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// A number between -2.0 and 2.0. Positive values penalize new tokens based on their existing
    /// frequency in the text so far, decreasing the model's likelihood to repeat the same line
    /// verbatim.
    pub fn frequency_penalty(mut self, frequency_penalty: f32) -> Self {
        self.frequency_penalty = Some(frequency_penalty);
        self
    }

    /// A number between -100 and 100. Positive values increase the likelihood of the specified
    /// tokens appearing in the completion.
    pub fn logit_bias(mut self, logit_bias: impl IntoIterator<Item = (u64, i8)>) -> Self {
        self.logit_bias = Some(logit_bias.into_iter().collect());
        self
    }

    /// A number between 0 and 20. Whether to return log probabilities of the output tokens.
    pub fn logprobs(mut self, logprobs: u8) -> Self {
        self.logprobs = Some(logprobs);
        self
    }

    /// A number between 0 and 20. The number of most likely tokens to return at each token
    /// position.
    pub fn top_logprobs(mut self, top_logprobs: u8) -> Self {
        self.top_logprobs = Some(top_logprobs);
        self
    }

    /// The maximum number of tokens that can be generated in the chat completion.
    pub fn max_tokens(mut self, max_tokens: u16) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// The number of chat completion choices to generate for each input message.
    ///
    /// Defaults to `1`.
    pub fn n(mut self, n: u8) -> Self {
        self.n = Some(n);
        self
    }

    /// A number between -2.0 and 2.0. Positive values penalize new tokens based on whether they
    /// appear in the text so far, increasing the model's likelihood to talk about new topics.
    ///
    /// Defaults to `0`.
    pub fn presence_penalty(mut self, presence_penalty: f32) -> Self {
        self.presence_penalty = Some(presence_penalty);
        self
    }

    /// An object specifying the format that the model must output.
    pub fn response_format(mut self, response_format: ResponseFormat) -> Self {
        self.response_format = Some(response_format);
        self
    }

    /// A best effort to sample deterministically.
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// The latency tier to use for processing the request.
    pub fn service_tier(mut self, service_tier: ServiceTier) -> Self {
        self.service_tier = Some(service_tier);
        self
    }

    /// Up to 4 sequences where the API will stop generating further tokens.
    pub fn stop(mut self, stop: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.stop = Some(stop.into_iter().map(|s| s.into()).collect());
        self
    }

    /// Whether to stream the response.
    pub fn stream(mut self, stream: bool) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Options for streaming response.
    pub fn stream_options(mut self, stream_options: StreamOptions) -> Self {
        self.stream_options = Some(stream_options);
        self
    }

    /// The sampling temperature to use.
    ///
    /// Defaults to `1`.
    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// An alternative to sampling with temperature, called nucleus sampling, where the model
    /// considers the results of the tokens with top_p probability mass.
    ///
    /// Defaults to `1`.
    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// A list of tools the model may call.
    pub fn tools(mut self, tools: Vec<Tool>) -> Self {
        self.tools = Some(tools);
        self
    }

    /// Controls which (if any) tool is called by the model.
    pub fn tool_choice(mut self, tool_choice: ToolChoice) -> Self {
        self.tool_choice = Some(tool_choice);
        self
    }

    /// Whether to enable parallel tool calls.
    ///
    /// Defaults to `true`.
    pub fn parallel_tool_calls(mut self, parallel_tool_calls: bool) -> Self {
        self.parallel_tool_calls = Some(parallel_tool_calls);
        self
    }

    /// A unique identifier representing your end-user.
    pub fn user(mut self, user: String) -> Self {
        self.user = Some(user);
        self
    }
}

impl OpenAIModelBuilder<()> {
    /// Builds the OpenAI model.
    pub fn build(self) -> OpenAIModel {
        let config = Config {
            model: ModelType::Gpt4oMini.to_string(),
            api_key: self.api_key,
            frequency_penalty: self.frequency_penalty,
            logit_bias: self.logit_bias,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
            max_tokens: self.max_tokens,
            n: self.n,
            presence_penalty: self.presence_penalty,
            response_format: self.response_format,
            seed: self.seed,
            service_tier: self.service_tier,
            stop: self.stop,
            stream: self.stream,
            stream_options: self.stream_options,
            temperature: self.temperature,
            top_p: self.top_p,
            tools: self.tools,
            tool_choice: self.tool_choice,
            parallel_tool_calls: self.parallel_tool_calls,
            user: self.user,
        };

        OpenAIModel {
            config: Cow::Owned(config),
            base_url: OPENAI_API_URL.to_string(),
        }
    }
}

impl OpenAIModelBuilder<String> {
    /// Builds the OpenAI model.
    pub fn build(self) -> OpenAIModel {
        let config = Config {
            model: self.model,
            api_key: self.api_key,
            frequency_penalty: self.frequency_penalty,
            logit_bias: self.logit_bias,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
            max_tokens: self.max_tokens,
            n: self.n,
            presence_penalty: self.presence_penalty,
            response_format: self.response_format,
            seed: self.seed,
            service_tier: self.service_tier,
            stop: self.stop,
            stream: self.stream,
            stream_options: self.stream_options,
            temperature: self.temperature,
            top_p: self.top_p,
            tools: self.tools,
            tool_choice: self.tool_choice,
            parallel_tool_calls: self.parallel_tool_calls,
            user: self.user,
        };

        OpenAIModel {
            config: Cow::Owned(config),
            base_url: OPENAI_API_URL.to_string(),
        }
    }
}

impl OpenAILikeModelBuilder<String> {
    /// Builds the OpenAI model.
    pub fn build(self) -> OpenAILikeModel {
        let config = Config {
            model: self.model,
            api_key: self.api_key,
            frequency_penalty: self.frequency_penalty,
            logit_bias: self.logit_bias,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
            max_tokens: self.max_tokens,
            n: self.n,
            presence_penalty: self.presence_penalty,
            response_format: self.response_format,
            seed: self.seed,
            service_tier: self.service_tier,
            stop: self.stop,
            stream: self.stream,
            stream_options: self.stream_options,
            temperature: self.temperature,
            top_p: self.top_p,
            tools: self.tools,
            tool_choice: self.tool_choice,
            parallel_tool_calls: self.parallel_tool_calls,
            user: self.user,
        };

        OpenAILikeModel(OpenAIModel {
            config: Cow::Owned(config),
            base_url: self.base_url,
        })
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for ModelBuilder<(), ()> {
    fn default() -> Self {
        Self {
            model: (),
            base_url: (),
            api_key: env::var(OPENAI_API_KEY).ok(),
            frequency_penalty: None,
            logit_bias: None,
            logprobs: None,
            top_logprobs: None,
            max_tokens: None,
            n: None,
            presence_penalty: None,
            response_format: None,
            seed: None,
            service_tier: None,
            stop: None,
            stream: None,
            stream_options: None,
            temperature: None,
            top_p: None,
            tools: None,
            tool_choice: None,
            parallel_tool_calls: None,
            user: None,
        }
    }
}

use std::{collections::HashMap, env};

use serde::{Deserialize, Serialize};
use strum_macros::Display;

//--------------------------------------------------------------------------------------------------
// Constants
//--------------------------------------------------------------------------------------------------

/// The environment variable for the OpenAI API key.
pub const OPENAI_API_KEY: &str = "OPENAI_API_KEY";

/// The URL for the OpenAI API.
pub const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// The configuration for the OpenAI model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// The ID of the model to use.
    pub model: String,

    /// The API key for making requests to the OpenAI API.
    #[serde(skip)]
    pub api_key: Option<String>,

    /// A number between -2.0 and 2.0. Positive values penalize new tokens based on their existing
    /// frequency in the text so far, decreasing the model's likelihood to repeat the same line
    /// verbatim.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Modifies the likelihood of specified tokens appearing in the completion.
    ///
    /// The associated value is a number between -100 and 100. Positive values increase the
    /// likelihood of the specified tokens appearing in the completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<u64, i8>>,

    /// Whether to return log probabilities of the output tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u8>,

    /// A number between 0 and 20 specifying the number of most likely tokens to return at each
    /// token position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u8>,

    /// The maximum number of tokens that can be generated in the chat completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u16>,

    /// The number of chat completion choices to generate for each input message.``
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,

    /// A number between -2.0 and 2.0. Positive values penalize new tokens based on whether they
    /// appear in the text so far, increasing the model's likelihood to talk about new topics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// An object specifying the format that the model must output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// A best effort to sample deterministically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,

    /// The latency tier to use for processing the request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTier>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<String>,

    /// Whether to response should be sent as data-only server-sent events as they become available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Options for streaming response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// The sampling temperature to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling, where the model
    /// considers the results of the tokens with top_p probability mass.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// A list of tools the model may call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<Tool>>,

    /// Controls which (if any) tool is called by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Whether to allow parallel tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// A unique identifier representing your end-user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// The model type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Display)]
pub enum ModelType {
    /// The GPT-4o mini model.
    #[serde(rename = "gpt-4o-mini")]
    #[strum(to_string = "gpt-4o-mini")]
    Gpt4oMini,

    /// The GPT-4o model.
    #[serde(rename = "gpt-4o")]
    #[strum(to_string = "gpt-4o")]
    Gpt4o,

    /// The GPT-4 turbo model.
    #[serde(rename = "gpt-4-turbo")]
    #[strum(to_string = "gpt-4-turbo")]
    Gpt4Turbo,

    /// The GPT-4 model.
    #[serde(rename = "gpt-4")]
    #[strum(to_string = "gpt-4")]
    Gpt4,

    /// The GPT-3.5 turbo model.
    #[serde(rename = "gpt-3.5-turbo")]
    #[strum(to_string = "gpt-3.5-turbo")]
    Gpt3_5Turbo,

    /// The GPT-3.5 turbo 16k model.
    #[serde(rename = "gpt-3.5-turbo-16k")]
    #[strum(to_string = "gpt-3.5-turbo-16k")]
    Gpt3_5Turbo16k,
}

/// Specifies the latency tier to use for processing the request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceTier {
    /// The system will utilize scale tier credits until they are exhausted.
    #[serde(rename = "auto")]
    Auto,

    /// The request will be processed using the default service tier with a lower uptime SLA and
    /// no latency guarentee.
    #[serde(rename = "default")]
    Default,
}

/// An object specifying the format that the model must output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum ResponseFormat {
    /// Enables Structured Outputs which ensures the model will match your supplied JSON schema
    #[serde(rename = "json_schema")]
    JsonSchema {
        /// The JSON schema that the model must match.
        json_schema: serde_json::Value,
    },

    /// Enables JSON mode, which ensures the message the model generates is valid JSON.
    #[serde(rename = "json_object")]
    JsonObject,
}

/// Options for streaming response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct StreamOptions {
    /// Whether to include the usage information in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

/// A tool that the model may call.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tool {
    /// The type of tool.
    pub r#type: ToolType,

    /// The function that the tool represents.
    pub function: Function,
}

/// Controls which (if any) tool is called by the model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum ToolChoice {
    /// The level of requirement for the tool.
    Requirement(Requirement),

    /// The tool that the model must call.
    RequiredTool(RequiredTool),
}

/// The type of tool.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum ToolType {
    /// Currently the only supported tool type.
    #[default]
    #[serde(rename = "function")]
    Function,
}

/// A function that the model may call.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct Function {
    /// The name of the function.
    pub name: String,

    /// A description of what the function does, used by the model to choose when and how to call
    /// the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The parameters that the function accepts, described as a JSON schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,

    /// Whether to enable strict schema adherence when generating the function call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// The level of requirement for the tool.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Requirement {
    /// The model will not call any tools.
    None,

    /// The model can pick between generating a message or calling one or more tools
    Auto,

    /// The model must call one or more tools
    Required,
}

/// Specifies a tool the model should use. Use to force the model to call a specific function.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequiredTool {
    /// The type of tool.
    pub r#type: ToolType,

    /// The function that the tool represents.
    pub function: Function,
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: env::var(OPENAI_API_KEY).ok(),
            model: ModelType::Gpt4o.to_string(),
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

impl From<ModelType> for String {
    fn from(value: ModelType) -> Self {
        value.to_string()
    }
}

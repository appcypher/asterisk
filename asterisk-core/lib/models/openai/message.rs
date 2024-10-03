use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::models::{AssistantMessage, Prompt, PromptMessage, SystemMessage, UserMessage};

use super::{Config, ToolType};

//--------------------------------------------------------------------------------------------------
// Types: Request
//--------------------------------------------------------------------------------------------------

/// The body of a request to the OpenAI API.
#[derive(Debug, Serialize)]
pub struct RequestBody {
    /// The messages to send to the model.
    pub messages: RequestMessages,

    /// The model's configuration.
    #[serde(flatten)]
    pub config: Config,
}

#[derive(Debug, Serialize)]
/// A collection of messages in a chat conversation with the model.
pub struct RequestMessages(pub Vec<RequestMessage>);

/// A message in a chat conversation with the model.
#[derive(Debug, Serialize)]
#[serde(tag = "role")]
pub enum RequestMessage {
    /// A message that sets the context for the conversation.
    #[serde(rename = "system")]
    System {
        /// The name of the system.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,

        /// The content of the message.
        content: String,
    },

    /// A message that assumes the role of the user.
    #[serde(rename = "user")]
    User {
        /// The name of the user.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,

        /// The content of the message.
        content: String,
    },

    /// A message that assumes the role of the assistant.
    #[serde(rename = "assistant")]
    Assistant {
        /// The name of the user.
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,

        /// The content of the message.
        content: String,

        /// Refusal message
        #[serde(skip_serializing_if = "Option::is_none")]
        refusal: Option<String>,

        /// The tool calls made by the assistant.
        #[serde(skip_serializing_if = "Option::is_none")]
        tool_calls: Option<Vec<ToolCall>>,
    },

    /// A message for a tool call.
    #[serde(rename = "tool")]
    Tool {
        /// The id of the tool call.
        #[serde(rename = "tool_call_id")]
        id: String,

        /// The content of the message.
        content: String,
    },
}

/// A tool call made by the assistant.
#[derive(Debug, Serialize, Deserialize)]
pub struct ToolCall {
    /// The id of the tool call.
    #[serde(rename = "tool_call_id")]
    id: String,

    /// The type of the tool.
    r#type: ToolType,

    /// The function call made by the tool.
    function: FunctionCall,
}

/// A function call made by the tool.
#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionCall {
    /// The name of the function.
    name: String,

    /// The arguments of the function.
    arguments: Vec<serde_json::Value>,
}

//--------------------------------------------------------------------------------------------------
// Types: Response
//--------------------------------------------------------------------------------------------------

/// Response body.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseBody {
    /// A successful response.
    Ok(Box<ResponseOk>),

    /// An error response.
    Error(Box<ResponseError>),
}

/// Represents an error response returned by the OpenAI API.
///
/// See [here](https://github.com/openai/openai-python/blob/9850c169c4126fd04dc6796e4685f1b9e4924aa4/src/openai/types/shared/error_object.py#L10) for more
#[derive(Debug, Deserialize, Error)]
#[error(transparent)]
pub struct ResponseError {
    /// The error information.
    pub error: ErrorInfo,
}

/// Represents a successful chat completion response returned by model, based on the provided input.
#[derive(Debug, Deserialize)]
pub struct ResponseOk {
    /// A unique identifier for the chat completion.
    pub id: String,

    /// A list of chat completion choices. Can be more than one if `n` is greater
    /// than 1.
    pub choices: Vec<Choice>,

    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: u64,

    /// The model to generate the completion.
    pub model: String,

    /// The service tier used for processing the request. This field is only included if the
    /// service_tier parameter is specified in the request.
    pub service_tier: Option<String>,

    /// This fingerprint represents the backend configuration that the model runs with.
    /// Can be used in conjunction with the seed request parameter to understand when backend
    /// changes have been made that might impact determinism.
    ///
    /// Optional because this is not available in some OpenAI compatible APIs like
    /// TogetherAI's.
    pub system_fingerprint: Option<String>,

    /// The object type, which is always `chat.completion`.
    pub object: String,

    /// Usage statistics for the completion request.
    pub usage: Option<Usage>,
}

/// A chunk of a chat completion response.
#[derive(Debug, Deserialize)]
pub struct ResponseChunkOk {
    /// A unique identifier for the chat completion.
    pub id: String,

    /// A list of chat completion choices. Can be more than one if `n` is greater
    /// than 1.
    pub choices: Vec<ResponseChunkChoice>,

    /// The Unix timestamp (in seconds) of when the chat completion was created.
    pub created: u64,

    /// The model to generate the completion.
    pub model: String,

    /// The service tier used for processing the request. This field is only included if the
    /// service_tier parameter is specified in the request.
    pub service_tier: Option<String>,

    /// This fingerprint represents the backend configuration that the model runs with.
    /// Can be used in conjunction with the seed request parameter to understand when backend
    /// changes have been made that might impact determinism.
    ///
    /// Optional because this is not available in some OpenAI compatible APIs like
    /// TogetherAI's.
    pub system_fingerprint: Option<String>,

    /// The object type, which is always `chat.completion`.
    pub object: String,

    /// Usage statistics for the completion request.
    pub usage: Option<Usage>,
}

/// The error information.
#[derive(Debug, Deserialize, Error)]
pub struct ErrorInfo {
    /// The error code.
    ///
    /// Optional because this is not available in some OpenAI compatible APIs like
    /// TogetherAI's.
    pub code: Option<String>,

    /// The error message.
    pub message: String,

    /// The parameter that caused the error.
    pub param: Option<String>,

    /// The type of error.
    pub r#type: String,
}

/// Usage statistics for the completion request.
#[derive(Debug, Deserialize)]
pub struct Usage {
    /// Number of tokens in the generated completion.
    pub completion_tokens: u64,

    /// Number of tokens in the prompt.
    pub prompt_tokens: u64,

    /// Total number of tokens used in the request (prompt + completion).
    pub total_tokens: u64,

    /// The acceptance rate of the generated tokens (between 0 and 1).
    ///
    /// Generated by the SambaNovaAPI.
    pub acceptance_rate: Option<f64>,

    /// The completion tokens per second.
    ///
    /// Generated by the SambaNovaAPI.
    pub completion_tokens_per_sec: Option<f64>,

    /// The completion tokens after the first per second.
    ///
    /// Generated by the SambaNovaAPI.
    pub completion_tokens_after_first_per_sec: Option<f64>,

    /// The completion tokens after the first per second in the first ten seconds.
    ///
    /// Generated by the SambaNovaAPI.
    pub completion_tokens_after_first_per_sec_first_ten: Option<f64>,

    /// The total latency of the request.
    ///
    /// Generated by the SambaNovaAPI.
    pub total_latency: Option<f64>,

    /// The time to first token of the request.
    ///
    /// Generated by the SambaNovaAPI.
    pub time_to_first_token: Option<f64>,

    /// The total tokens per second.
    ///
    /// Generated by the SambaNovaAPI.
    pub total_tokens_per_sec: Option<f64>,

    /// The start time of the request.
    ///
    /// Generated by the SambaNovaAPI.
    pub start_time: Option<f64>,

    /// The end time of the request.
    ///
    /// Generated by the SambaNovaAPI.
    pub end_time: Option<f64>,

    /// The is last response of the request.
    ///
    /// Generated by the SambaNovaAPI.
    pub is_last_response: Option<bool>,
}

/// A chat completion choice.
#[derive(Debug, Deserialize)]
pub struct Choice {
    /// The reason the model stopped generating tokens. This will be stop if the model hit a natural
    /// stop point or a provided stop sequence, length if the maximum number of tokens specified in
    /// the request was reached, content_filter if content was omitted due to a flag from our
    /// content filters, tool_calls if the model called a tool, or function_call (deprecated) if the
    /// model called a function.
    pub finish_reason: Option<String>,

    /// The index of the choice in the list of choices.
    pub index: u64,

    /// A chat completion message generated by the model.
    pub message: ChoiceMessage,

    /// Log probability information for the choice.
    pub logprobs: Option<ChoiceLogprobs>,
}

/// A chat completion choice.
#[derive(Debug, Deserialize)]
pub struct ResponseChunkChoice {
    /// A chat completion delta generated by streamed model responses.
    ///
    /// This is only returned when the model is streaming.
    pub delta: ChoiceDelta,

    /// The reason the model stopped generating tokens. This will be stop if the model hit a natural
    /// stop point or a provided stop sequence, length if the maximum number of tokens specified in
    /// the request was reached, content_filter if content was omitted due to a flag from our
    /// content filters, tool_calls if the model called a tool, or function_call (deprecated) if the
    /// model called a function.
    pub finish_reason: Option<String>,

    /// The index of the choice in the list of choices.
    pub index: u64,

    /// Log probability information for the choice.
    pub logprobs: Option<ChoiceLogprobs>,
}

/// A chat completion delta generated by streamed model responses.
#[derive(Debug, Deserialize)]
pub struct ChoiceDelta {
    /// The content of the message.
    pub content: Option<String>,

    /// Refusal message
    pub refusal: Option<String>,

    /// The tool calls made by the assistant.
    pub tool_calls: Option<Vec<ToolCall>>,

    /// The role of the message.
    ///
    /// Optional because it can be missing in some OpenAI compatible APIs like
    /// SambaNova API.
    pub role: Option<String>,
}

/// A chat completion message generated by the model.
#[derive(Debug, Deserialize)]
pub struct ChoiceMessage {
    /// The content of the message.
    pub content: Option<String>,

    /// Refusal message
    pub refusal: Option<String>,

    /// The tool calls made by the assistant.
    pub tool_calls: Option<Vec<ToolCall>>,

    /// The role of the message.
    ///
    ///
    pub role: Option<String>,
}

/// Log probability information for the choice.
#[derive(Debug, Deserialize)]
pub struct ChoiceLogprobs {
    /// The content of the log probability information.
    pub content: Option<Vec<LogProbsContent>>,

    /// A list of message refusal tokens with log probability information.
    pub refusal: Option<Vec<LogProbsToken>>,
}

/// Log probability information for the choice.
#[derive(Debug, Deserialize)]
pub struct LogProbsContent {
    /// The token.
    #[serde(flatten)]
    pub token: LogProbsToken,
}

/// A token with log probability information.
#[derive(Debug, Deserialize)]
pub struct LogProbsToken {
    /// The token.
    pub token: String,

    /// The log probability of this token, if it is within the top 20 most likely tokens. Otherwise,
    /// the value -9999.0 is used to signify that the token is very unlikely.
    pub logprob: f64,

    /// A list of integers representing the UTF-8 bytes representation of the token. Useful in
    /// instances where characters are represented by multiple tokens and their byte
    /// representations must be combined to generate the correct text representation. Can be null
    /// if there is no bytes representation for the token.
    pub bytes: Option<Vec<u8>>,

    /// List of the most likely tokens and their log probability, at this token position. In rare
    /// cases, there may be fewer than the number of requested top_logprobs returned.
    pub top_logprobs: Option<Vec<TopLogprobs>>,
}

/// The most likely tokens and their log probability, at this token position. In rare cases, there
/// may be fewer than the number of requested top_logprobs returned.
#[derive(Debug, Deserialize)]
pub struct TopLogprobs {
    /// The token.
    pub token: String,

    /// The log probability of this token, if it is within the top 20 most likely tokens. Otherwise,
    /// the value -9999.0 is used to signify that the token is very unlikely.
    pub logprob: f64,

    /// A list of integers representing the UTF-8 bytes representation of the token. Useful in
    /// instances where characters are represented by multiple tokens and their byte
    /// representations must be combined to generate the correct text representation. Can be null
    /// if there is no bytes representation for the token.
    pub bytes: Option<Vec<u8>>,
}

//--------------------------------------------------------------------------------------------------
// Methods
//--------------------------------------------------------------------------------------------------

impl ResponseBody {
    /// Gets the error variant or panics.
    pub fn unwrap_err(self) -> ResponseError {
        match self {
            ResponseBody::Error(error) => *error,
            ResponseBody::Ok(_) => panic!("Called `unwrap_err()` on a `ResponseBody::Ok` value"),
        }
    }
}

//--------------------------------------------------------------------------------------------------
// Trait Implementations
//--------------------------------------------------------------------------------------------------

impl Display for ErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<Prompt> for RequestMessages {
    fn from(prompt: Prompt) -> Self {
        let request_messages = prompt
            .into_iter()
            .map(|m| match m {
                PromptMessage::System(SystemMessage { content }) => RequestMessage::System {
                    content,
                    name: None,
                },
                PromptMessage::User(UserMessage { content }) => RequestMessage::User {
                    content,
                    name: None,
                },
                PromptMessage::Assistant(AssistantMessage { content }) => {
                    RequestMessage::Assistant {
                        content,
                        name: None,
                        refusal: None,
                        tool_calls: None,
                    }
                }
            })
            .collect();

        Self(request_messages)
    }
}

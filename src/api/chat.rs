use serde::{Deserialize, Serialize};

use super::GenerateModel;

#[derive(Serialize, Default, Debug)]
pub struct ChatRequest<'input> {
    /// The chat message from the user to the model.
    pub message: &'input str,
    /// The model to use for text generation. Custom models can also be supplied with their full ID. Defaults to `command-r-plus`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<GenerateModel>,
    // optional - When specified, the default Cohere preamble will be replaced with the provided one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preamble: Option<String>,
    /// optional - A list of previous messages between the user and the model,
    /// meant to give the model conversational context for responding to the user's message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_history: Option<&'input Vec<ChatMessage>>,
    /// optional - Previous conversations can be stored and resumed by providing the conversation's identifier.
    /// If a conversation with this id does not already exist, a new conversation will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// optional - Dictates how the prompt will be constructed. When set to 'AUTO' some parts of chat history and documents will be dropped
    /// to construct a prompt that fits within the model's context length limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_truncation: Option<PromptTruncation>,
    /// optional - A non-negative float that tunes the degree of randomness in generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    // optional - The maximum number of tokens the model will generate as part of the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    // optional - Ensures only the top k most likely tokens are considered for generation at each step
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "role")]
pub enum ChatMessage {
    #[serde(rename = "CHATBOT")]
    Chatbot { message: String },
    #[serde(rename = "USER")]
    User { message: String },
}

#[derive(Serialize, Debug)]
pub struct ChatStreamRequest<'input> {
    #[serde(flatten)]
    pub request: ChatRequest<'input>,
    stream: bool,
}

impl<'input> From<ChatRequest<'input>> for ChatStreamRequest<'input> {
    fn from(request: ChatRequest<'input>) -> Self {
        Self {
            request,
            stream: true,
        }
    }
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum PromptTruncation {
    #[strum(serialize = "AUTO")]
    #[serde(rename = "AUTO")]
    Auto,
    #[strum(serialize = "OFF")]
    #[serde(rename = "OFF")]
    Off,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum CitationQuality {
    #[strum(serialize = "accurate")]
    #[serde(rename = "accurate")]
    Accurate,
    #[strum(serialize = "fast")]
    #[serde(rename = "fast")]
    Fast,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ChatResponse {
    pub generation_id: String,
    pub response_id: String,
    pub text: String,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "event_type")]
pub enum StreamEvent {
    #[serde(rename = "stream-start")]
    Start {
        generation_id: String,
        is_finished: bool,
    },
    #[serde(rename = "text-generation")]
    TextGeneration { is_finished: bool, text: String },
    #[serde(rename = "stream-end")]
    End {
        finish_reason: String,
        is_finished: bool,
        response: ChatResponse,
    },
}

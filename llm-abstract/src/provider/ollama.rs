use crate::llm_error::LlmError;
use crate::{LlmProvider, LlmRequest, LlmResonse};

// use reqwest::Client;
use http_handler::HttpHandler;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) static OLLAMA_CHAT_ENDPOINT: &str = "127.0.0.1:11434/api/chat";
pub(crate) static OLLAMA_SYS_PROMPT: &str =
    "You are an expert software coding engineer. Keep your answers concise and well structured.";

pub struct OllamaLlm {
    handler: HttpHandler,
    history: OllamaRequest,
}

impl OllamaLlm {
    pub fn new(handler: HttpHandler, model: String) -> Self {
        let history = OllamaRequest::new(model);
        Self { handler, history }
    }
}

#[async_trait::async_trait]
impl LlmProvider for OllamaLlm {
    type ChatReq = OllamaMessage;
    type ChatRes = OllamaResponse;

    async fn chat(&mut self, request: Self::ChatReq) -> Result<Self::ChatRes, LlmError> {
        self.history.push_msg(request);

        let res = self
            .handler
            .post::<OllamaRequest, OllamaResponse>(OLLAMA_CHAT_ENDPOINT, &self.history)
            .await?;

        Ok(res)
    }
}

#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}
impl OllamaRequest {
    pub fn new(model: String) -> Self {
        let messages = vec![OllamaMessage::system_prompt()];
        Self {
            model,
            messages,
            stream: false,
        }
    }
    fn push_msg(&mut self, msg: OllamaMessage) -> () {
        self.messages.push(msg);
    }
}

#[derive(Debug, Deserialize)]
pub struct OllamaResponse {
    message: OllamaMessage,
    done: bool,
}

impl LlmResonse for OllamaResponse {
    fn get_message(&self) -> &str {
        self.message.content()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaToolCall {
    function: OllamaFunctionCall,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaFunctionCall {
    name: String,
    arguments: Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaMessage {
    role: MessageRole,
    content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OllamaToolCall>>,
}

impl OllamaMessage {
    fn system_prompt() -> Self {
        Self {
            role: MessageRole::System,
            content: String::from(OLLAMA_SYS_PROMPT),
            tool_calls: None,
        }
    }
    fn content(&self) -> &str {
        &self.content
    }
}

impl LlmRequest for OllamaMessage {
    fn new_user_msg(content: String) -> Self {
        Self {
            role: MessageRole::User,
            content,
            tool_calls: None,
        }
    }
}

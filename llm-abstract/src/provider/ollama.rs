use crate::llm_error::LlmError;
use crate::{LlmProvider, LlmRequest, LlmResonse, RegisteredTool, Tool, ToolRegistry};

// use reqwest::Client;
use http_handler::HttpHandler;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub(crate) static OLLAMA_CHAT_ENDPOINT: &str = "http://localhost:11434/api/chat";
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
    type ProviderTool = OllamaTool;

    async fn chat(&mut self, request: Self::ChatReq) -> Result<Self::ChatRes, LlmError> {
        self.history.push_msg(request);

        let res = self
            .handler
            .post::<OllamaRequest, OllamaResponse>(OLLAMA_CHAT_ENDPOINT, &self.history)
            .await?;

        Ok(res)
    }

    fn register_tools(&mut self, tools: ToolRegistry) -> () {
        let ollama_tools = tools
            .registered_tools
            .into_iter()
            .map(|t| OllamaTool::from(t))
            .collect::<Vec<_>>();

        self.history.add_tools(ollama_tools);
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OllamaToolType {
    Function,
}

#[derive(Debug, Serialize)]
pub struct OllamaFunction {
    name: String,
    parameters: Value,
    description: String,
}

impl OllamaFunction {
    fn new_func(name: String, parameters: Value, description: String) -> Self {
        Self {
            name,
            parameters,
            description,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OllamaTool {
    #[serde(rename = "type")]
    tool_type: OllamaToolType,
    function: OllamaFunction,
}

impl Tool for OllamaTool {}

impl From<RegisteredTool> for OllamaTool {
    fn from(value: RegisteredTool) -> Self {
        Self {
            tool_type: OllamaToolType::Function,
            function: OllamaFunction::new_func(
                value.func_name,
                value.func_parameters,
                value.func_desc,
            ),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OllamaTool>>,
    stream: bool,
}
impl OllamaRequest {
    pub fn new(model: String) -> Self {
        let messages = vec![OllamaMessage::system_prompt()];
        Self {
            model,
            messages,
            tools: None,
            stream: false,
        }
    }
    fn push_msg(&mut self, msg: OllamaMessage) -> () {
        self.messages.push(msg);
    }
    fn add_tools(&mut self, tools: Vec<OllamaTool>) -> () {
        self.tools = Some(tools);
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

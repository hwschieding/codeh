use llm_error::LlmError;
use serde_json::Value;

pub(crate) mod llm_error;
pub mod provider;

#[cfg(test)]
mod tests;

#[async_trait::async_trait]
pub trait LlmProvider {
    type ChatReq: LlmRequest; // request type
    type ChatRes: LlmResonse; // response type
    type ProviderTool: Tool;
    type ProviderToolCall: ToolCall;

    async fn chat(&mut self, request: Self::ChatReq) -> Result<Self::ChatRes, LlmError>;
    fn new_user_msg(content: String) -> Self::ChatReq {
        Self::ChatReq::new_user_msg(content)
    }

    fn add_tools(&mut self, tools: ToolRegistry) -> ();
}

pub struct RegisteredTool {
    func_name: String,
    func_parameters: Value,
    func_desc: String,
}

pub struct ToolRegistry {
    registered_tools: Vec<RegisteredTool>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let registered_tools = Vec::new();
        Self { registered_tools }
    }
    pub fn register_tool(&mut self, tool: impl Into<RegisteredTool>) -> () {
        self.registered_tools.push(tool.into());
    }
}

pub trait LlmRequest {
    fn new_user_msg(content: String) -> Self;
}
pub trait LlmResonse {
    type ProviderToolCall: ToolCall;

    fn get_message_content(&self) -> &str;
    fn get_message_tools(&self) -> Option<&Vec<Self::ProviderToolCall>>;
}
pub trait Tool: From<RegisteredTool> {}

pub trait ToolCall {}

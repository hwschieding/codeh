use llm_error::LlmError;

mod llm_error;
mod provider;

#[cfg(test)]
mod tests;

#[async_trait::async_trait]
pub trait LlmProvider {
    type ChatReq: LlmRequest; // request type
    type ChatRes: LlmResonse; // response type

    async fn chat(&mut self, request: Self::ChatReq) -> Result<Self::ChatRes, LlmError>;
    fn new_user_msg(content: String) -> Self::ChatReq {
        Self::ChatReq::new_user_msg(content)
    }
}

pub trait LlmRequest {
    fn new_user_msg(content: String) -> Self;
}
pub trait LlmResonse {
    fn get_message(&self) -> &str;
}

// TODO:
// Implement Ollama API with these traits, bare HTTPS no reqwest.

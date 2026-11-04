mod llm_error;
mod provider;

#[cfg(test)]
mod tests;

#[async_trait::async_trait]
pub trait LlmProvider {
    type ChatReq: LlmRequest; // request type
    type ChatRes: LlmResonse; // response type

    async fn chat(&mut self, request: Self::ChatReq) -> Self::ChatRes;
}

pub trait LlmRequest {}
pub trait LlmResonse {}

// TODO:
// Implement Ollama API with these traits, bare HTTPS no reqwest.

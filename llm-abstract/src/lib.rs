use async_trait::async_trait;

#[cfg(test)]
mod tests;

#[async_trait]
pub trait LlmProvider {
    type ChatReq; // request type
    type ChatRes; // response type

    async fn chat(&self, request: Self::ChatReq) -> Self::ChatRes;
}

// TODO:
// Add request and response traits.
// Implement Ollama API with these traits, bare HTTPS no reqwest.

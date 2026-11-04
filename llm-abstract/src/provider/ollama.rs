use crate::llm_error::LlmError;
use crate::{LlmProvider, LlmRequest, LlmResonse};

use serde::{Deserialize, Serialize};
use std::net::TcpStream;

pub(crate) static OLLAMA_SERVER: &str = "127.0.0.1:11434";

pub struct OllamaLlm {
    stream: TcpStream,
}

impl OllamaLlm {
    pub fn new() -> Result<Self, LlmError> {
        let stream = TcpStream::connect(OLLAMA_SERVER)?;

        Ok(Self { stream })
    }
}

#[async_trait::async_trait]
impl LlmProvider for OllamaLlm {
    type ChatReq = OllamaRequest;
    type ChatRes = OllamaResponse;

    async fn chat(&mut self, request: Self::ChatReq) -> Self::ChatRes {
        todo!()
    }
}

#[derive(Debug, Serialize)]
pub struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

impl LlmRequest for OllamaRequest {}

#[derive(Debug, Serialize)]
pub struct OllamaResponse {
    response: String,
    done: bool,
}

impl LlmResonse for OllamaResponse {}

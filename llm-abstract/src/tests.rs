use super::*;
use crate::provider::ollama::OllamaLlm;

#[tokio::test]
async fn ollama_serialization() {
    let msg = OllamaLlm::new_user_msg(String::from("User message"));
    let serialized = serde_json::to_string(&msg).expect("Serialization should succeed");
    println!("{serialized}");
    assert_eq!(
        serialized,
        "{\"role\":\"user\",\"content\":\"User message\"}"
    );
}

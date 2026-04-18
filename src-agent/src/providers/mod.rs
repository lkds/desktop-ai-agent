pub mod trait;
pub mod openai;
pub mod claude;
pub mod ollama;

pub use trait::{Provider, ProviderConfig, ProviderKind, ProviderError, Message, Role, ToolCall, GenerateRequest, GenerateResponse, StreamChunk};
pub use openai::OpenAIProvider;
pub use claude::ClaudeProvider;
pub use ollama::OllamaProvider;
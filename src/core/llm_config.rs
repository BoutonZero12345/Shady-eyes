use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LlmProvider {
    OpenAI,
    Gemini,
    Claude,
    Unknown,
}

impl LlmProvider {
    pub fn detect(key: &str) -> Self {
        if key.starts_with("sk-ant-") {
            LlmProvider::Claude
        } else if key.starts_with("sk-") {
            LlmProvider::OpenAI
        } else if key.starts_with("AIza") {
            LlmProvider::Gemini
        } else {
            LlmProvider::Unknown
        }
    }

    pub fn get_api_url(&self, model: &str, key: &str) -> String {
        match self {
            LlmProvider::OpenAI => "https://api.openai.com/v1/chat/completions".to_string(),
            LlmProvider::Gemini => format!(
                "https://generativelanguage.googleapis.com/v1beta/{}:generateContent?key={}",
                model, key
            ),
            LlmProvider::Claude => "https://api.anthropic.com/v1/messages".to_string(),
            LlmProvider::Unknown => "".to_string(),
        }
    }
}

pub const SYSTEM_PROMPT: &str = "Tu t'appelles Sum Sum ou Summer. N'utilise JAMAIS d'emojis. Tes réponses doivent être un peu concises, sauf si l'utilisateur te demande explicitement de développer.";
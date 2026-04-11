// src/app/types.rs
use crate::core::llm_config::LlmProvider;

#[derive(PartialEq)]
pub enum SetupStep {
    InputKey,
    SelectModel,
}

pub enum ApiPayload {
    FetchModels(String, LlmProvider),
    Chat(String, LlmProvider, String, Vec<(String, String)>),
}

pub enum ApiResponse {
    ModelsFetched(Vec<String>),
    ChatResponse(String),
    Error(String),
}
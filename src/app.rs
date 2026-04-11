// src/app.rs
use eframe::egui;
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::core::llm_config::LlmProvider;
use crate::api::client::ApiClient;

// Déclaration de nos nouveaux sous-fichiers
pub mod types;
pub mod logic;
pub mod render;

use types::{ApiPayload, ApiResponse, SetupStep};

pub struct ShadyApp {
    // Les champs sont maintenant 'pub' pour que nos sous-fichiers puissent y accéder
    pub api_key: String,
    pub provider: LlmProvider,
    pub model: String,
    pub available_models: Vec<String>,
    pub is_setup: bool,
    pub setup_step: SetupStep,
    
    pub history: Vec<(String, String)>,
    pub user_input: String,
    
    pub current_eye_offset: egui::Vec2,
    pub eye_y_scale: f32,
    pub blink_timer: f32,
    pub next_blink: f32,

    pub tx: Sender<ApiPayload>,
    pub rx: Receiver<ApiResponse>,
    pub is_waiting: bool,
    pub status_message: String,
}

impl ShadyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (tx_api, rx_main) = channel::<ApiResponse>();
        let (tx_main, rx_api) = channel::<ApiPayload>();
        
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let client = ApiClient::new();
            while let Ok(payload) = rx_api.recv() {
                match payload {
                    ApiPayload::FetchModels(key, prov) => {
                        match rt.block_on(client.fetch_models(&key, prov)) {
                            Ok(m) => { let _ = tx_api.send(ApiResponse::ModelsFetched(m)); },
                            Err(e) => { let _ = tx_api.send(ApiResponse::Error(e)); }
                        }
                    }
                    ApiPayload::Chat(key, prov, model, hist) => {
                        match rt.block_on(client.send_chat(&key, prov, &model, hist)) {
                            Ok(text) => { let _ = tx_api.send(ApiResponse::ChatResponse(text)); },
                            Err(e) => { let _ = tx_api.send(ApiResponse::Error(e)); }
                        }
                    }
                }
            }
        });

        let _ = dotenvy::dotenv();
        let api_key = std::env::var("API_KEY").unwrap_or_default();
        let model = std::env::var("MODEL").unwrap_or_default();
        let is_setup = api_key.is_empty();

        Self {
            api_key,
            provider: LlmProvider::Unknown,
            model,
            available_models: Vec::new(),
            is_setup,
            setup_step: SetupStep::InputKey,
            history: Vec::new(),
            user_input: String::new(),
            current_eye_offset: egui::Vec2::ZERO,
            eye_y_scale: 1.0,
            blink_timer: 0.0,
            next_blink: 4.0,
            tx: tx_main,
            rx: rx_main,
            is_waiting: false,
            status_message: "SYSTEM READY".to_string(),
        }
    }

    pub fn save_env(&self) {
        let content = format!("API_KEY={}\nMODEL={}\n", self.api_key, self.model);
        std::fs::write(".env", content).expect("Impossible d'écrire .env");
    }
}

// La boucle d'affichage devient ultra minimaliste
impl eframe::App for ShadyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. On met à jour les maths et l'état
        logic::update_state(self);
        
        // 2. On dessine l'interface
        render::draw_ui(self, ctx);
        
        ctx.request_repaint();
    }
}
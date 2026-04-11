use eframe::egui::{self, Key, Pos2, Rect, Vec2, Color32};
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::core::config::*;
use crate::core::llm_config::LlmProvider;
use crate::api::client::ApiClient;
use crate::ui::eyes::draw_eyes;

#[derive(PartialEq)]
enum SetupStep {
    InputKey,
    SelectModel,
}

pub struct ShadyApp {
    // Config
    api_key: String,
    provider: LlmProvider,
    model: String,
    available_models: Vec<String>,
    is_setup: bool,
    setup_step: SetupStep,
    
    // Chat
    history: Vec<(String, String)>,
    user_input: String,
    
    // Visuel
    current_eye_offset: Vec2,
    eye_y_scale: f32,
    blink_timer: f32,
    next_blink: f32,

    // Async communication
    tx: Sender<ApiPayload>,
    rx: Receiver<ApiResponse>,
    is_waiting: bool,
    status_message: String,
}

enum ApiPayload {
    FetchModels(String, LlmProvider),
    Chat(String, LlmProvider, String, Vec<(String, String)>),
}

enum ApiResponse {
    ModelsFetched(Vec<String>),
    ChatResponse(String),
    Error(String),
}

impl ShadyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (tx_api, rx_main) = channel::<ApiResponse>();
        let (tx_main, rx_api) = channel::<ApiPayload>();
        
        // Thread de travail pour les appels API
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
            current_eye_offset: Vec2::ZERO,
            eye_y_scale: 1.0,
            blink_timer: 0.0,
            next_blink: 4.0,
            tx: tx_main,
            rx: rx_main,
            is_waiting: false,
            status_message: "SYSTEM READY".to_string(),
        }
    }

    fn save_env(&self) {
        let content = format!("API_KEY={}\nMODEL={}\n", self.api_key, self.model);
        std::fs::write(".env", content).expect("Impossible d'écrire .env");
    }
}

impl eframe::App for ShadyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- GESTION DU TEMPS (BLINK) ---
        let dt = 0.016; 
        self.blink_timer += dt;
        if self.blink_timer > self.next_blink {
            let blink_progress = (self.blink_timer - self.next_blink) * 15.0;
            if blink_progress > std::f32::consts::PI {
                self.eye_y_scale = 1.0;
                self.blink_timer = 0.0;
                self.next_blink = rand::Rng::gen_range(&mut rand::thread_rng(), 2.0..6.0);
            } else {
                self.eye_y_scale = 1.0 - (blink_progress.sin() * 0.95);
            }
        }

        // --- RÉCEPTION DES RÉPONSES API ---
        if let Ok(res) = self.rx.try_recv() {
            self.is_waiting = false;
            match res {
                ApiResponse::ModelsFetched(m) => {
                    self.available_models = m;
                    self.setup_step = SetupStep::SelectModel;
                    self.status_message = "SELECT MODEL [1, 2, 3...]".to_string();
                }
                ApiResponse::ChatResponse(t) => {
                    self.history.push(("Sum Sum".to_string(), t));
                    self.status_message = "ONLINE".to_string();
                }
                ApiResponse::Error(e) => {
                    self.status_message = format!("ERROR: {}", e);
                }
            }
        }

        // --- DESSIN DE L'INTERFACE ---
        egui::CentralPanel::default().frame(egui::Frame::none().fill(BG_COLOR)).show(ctx, |ui| {
            let rect = ui.available_rect_before_wrap();
            
            // 1. DESSIN DES YEUX (SUM SUM)
            self.current_eye_offset = draw_eyes(ctx, rect.center().x, rect.center().y - 20.0, self.current_eye_offset, self.eye_y_scale);

            // 2. ZONE DE TEXTE (TERMINAL)
            let terminal_area = Rect::from_min_max(
                Pos2::new(15.0, 15.0), 
                Pos2::new(rect.max.x - 15.0, rect.max.y - 70.0)
            );

            ui.allocate_ui_at_rect(terminal_area, |ui| {
                ui.vertical(|ui| {
                    // On masque le statut s'il s'agit juste de "ONLINE" ou "SYSTEM ONLINE"
                    if !self.status_message.contains("ONLINE") {
                        ui.colored_label(Color32::from_rgb(0, 255, 0), &self.status_message);
                        ui.add_space(10.0);
                    }

                    if self.is_setup && self.setup_step == SetupStep::SelectModel {
                        // AFFICHAGE DE LA LISTE DES MODÈLES
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (i, m) in self.available_models.iter().enumerate() {
                                ui.label(format!("[{}] {}", i + 1, m));
                            }
                        });
                    } else {
                        // AFFICHAGE DU CHAT CLASSIQUE
                        egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                            for (role, msg) in &self.history {
                                let color = if role == "User" || role == "user" {
                                    TEXT_USER_COLOR
                                } else {
                                    TEXT_SYSTEM_COLOR
                                };
                                // On a enlevé le nom, on met juste ">"
                                ui.colored_label(color, format!("> {}", msg));
                                ui.add_space(4.0);
                            }
                            if self.is_waiting {
                                ui.label("> ...");
                            }
                        });
                    }
                });
            });

            




            // 3. BARRE DE SAISIE (BAS)
            let input_area = Rect::from_min_max(
                Pos2::new(10.0, rect.max.y - 80.0), 
                Pos2::new(rect.max.x - 10.0, rect.max.y - 10.0)
            );

            ui.allocate_ui_at_rect(input_area, |ui| {
                ui.horizontal_top(|ui| {
                    let prompt = if self.is_setup {
                        if self.setup_step == SetupStep::InputKey { "SET API KEY > " } else { "SELECT # > " }
                    } else {
                        "CHAT > "
                    };
                    
                    ui.label(prompt);

                    // --- EXPLICATION DE LA MODIFICATION ---
                    // On crée la fameuse "cage" (ScrollArea). On lui donne un ID unique 
                    // pour qu'elle ne confonde pas son scroll avec celui du terminal au-dessus.
                    // On lui impose une hauteur MAxIMALE stricte de 55 pixels (~ 3 lignes).
                    egui::ScrollArea::vertical()
                        .id_source("input_scroll_box")
                        .max_height(55.0) 
                        .show(ui, |ui| {
                            
                            // Le TextEdit est maintenant prisonnier de la cage.
                            // S'il dépasse 55 pixels, la cage bloque sa croissance et active le scroll.
                            let output = egui::TextEdit::multiline(&mut self.user_input)
                                .desired_width(ui.available_width()) 
                                .desired_rows(3) 
                                .lock_focus(true)
                                .font(egui::FontId::monospace(14.0))
                                .show(ui);
                            
                            let edit = output.response;

                            // --- LOGIQUE DU LASER VERT ---
                            if edit.has_focus() {
                                if let Some(cursor_range) = output.cursor_range {
                                    let cursor_rect = output.galley.pos_from_cursor(&cursor_range.primary);
                                    let screen_pos = edit.rect.min + cursor_rect.center().to_vec2();
                                    ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new("cursor_pos"), screen_pos));
                                }
                            } else {
                                ui.ctx().data_mut(|d| d.remove::<Pos2>(egui::Id::new("cursor_pos")));
                            }
                            
                            // Validation avec la touche Entrée (sans Shift)
                            if edit.has_focus() && ui.input(|i| i.key_pressed(Key::Enter) && !i.modifiers.shift) {
                                let input = self.user_input.trim().to_string();
                                if !input.is_empty() {
                                    if self.is_setup {
                                        match self.setup_step {
                                            SetupStep::InputKey => {
                                                self.api_key = input;
                                                self.provider = LlmProvider::detect(&self.api_key);
                                                self.status_message = "FETCHING MODELS...".to_string();
                                                let _ = self.tx.send(ApiPayload::FetchModels(self.api_key.clone(), self.provider.clone()));
                                                self.is_waiting = true;
                                            }
                                            SetupStep::SelectModel => {
                                                if let Ok(idx) = input.parse::<usize>() {
                                                    if idx > 0 && idx <= self.available_models.len() {
                                                        self.model = self.available_models[idx-1].clone();
                                                        self.save_env();
                                                        self.is_setup = false;
                                                        self.status_message = "SYSTEM ONLINE".to_string();
                                                        let _ = self.tx.send(ApiPayload::Chat(
                                                            self.api_key.clone(), 
                                                            self.provider.clone(), 
                                                            self.model.clone(), 
                                                            vec![("user".into(), "Bonjour".into())]
                                                        ));
                                                        self.is_waiting = true;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self.history.push(("User".to_string(), input.clone()));
                                        let _ = self.tx.send(ApiPayload::Chat(
                                            self.api_key.clone(), 
                                            self.provider.clone(), 
                                            self.model.clone(), 
                                            self.history.clone()
                                        ));
                                        self.is_waiting = true;
                                    }
                                }
                                self.user_input.clear();
                                edit.request_focus();
                            }
                        }); // Fin de la cage ScrollArea
                });
            });






               });
        ctx.request_repaint();
    }
}
use eframe::egui::{self, Pos2, Rect, Vec2};
use serde::{Deserialize, Serialize};

// On importe nos modules proprement découpés
use crate::core::config::{BG_COLOR, TARGET_FPS};
use crate::core::types::{Message, Role};
use crate::ui::eyes::draw_eyes;
use crate::ui::login::show_login_screen;
use crate::ui::terminal::show_terminal_screen;

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct ShadyApp {
    api_key: String,
    is_unlocked: bool,
    history: Vec<Message>,
    #[serde(skip)]
    user_input: String,
}

impl Default for ShadyApp {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            is_unlocked: false,
            history: vec![Message {
                role: Role::System,
                content: "HOW CAN I HELP YOU TODAY?".to_string(),
            }],
            user_input: String::new(),
        }
    }
}

impl ShadyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Self::default()
    }
}

impl eframe::App for ShadyApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Limitation dynamique des FPS depuis notre fichier de configuration
        ctx.request_repaint_after(std::time::Duration::from_millis(1000 / TARGET_FPS));

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(BG_COLOR))
            .show(ctx, |ui| {
                let available_rect = ui.available_rect_before_wrap();
                let center_x = available_rect.center().x;
                let center_y = available_rect.center().y - 120.0;

                // 1. On dessine les yeux en arrière-plan (géré par le module eyes.rs)
                draw_eyes(ctx, center_x, center_y, &self.user_input);

                // 2. On définit la zone allouée pour l'interface texte (en dessous des yeux)
                let terminal_rect = Rect::from_min_size(
                    Pos2::new(40.0, center_y + 120.0),
                    Vec2::new(
                        available_rect.width() - 80.0,
                        available_rect.height() - (center_y + 120.0) - 20.0,
                    ),
                );

                // 3. On affiche le bon écran selon l'état de l'application
                ui.allocate_ui_at_rect(terminal_rect, |ui| {
                    ui.vertical(|ui| {
                        // On force la police Monospace pour tout ce qui est dans cette zone
                        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

                        if !self.is_unlocked {
                            // Écran 1 : Appel du module de connexion
                            show_login_screen(ui, &mut self.api_key, &mut self.is_unlocked);
                        } else {
                            // Écran 2 : Appel du module de chat
                            show_terminal_screen(ui, &mut self.history, &mut self.user_input);
                        }
                    });
                });
            });
    }
}
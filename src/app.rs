use eframe::egui::{self, Pos2, Rect, Vec2};
use serde::{Deserialize, Serialize};
use rand::Rng; // Import pour l'aléatoire
use crate::core::config::*;
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
    #[serde(skip)]
    current_eye_offset: Vec2,
    #[serde(skip)]
    blink_timer: f32,       // Temps écoulé depuis le dernier clignement
    #[serde(skip)]
    next_blink: f32,        // Quand le prochain clignement doit arriver
    #[serde(skip)]
    eye_y_scale: f32,       // Facteur d'écrasement vertical actuel
}

impl Default for ShadyApp {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            api_key: String::new(),
            is_unlocked: false,
            history: vec![Message { role: Role::System, content: STR_WELCOME.to_string() }],
            user_input: String::new(),
            current_eye_offset: Vec2::ZERO,
            blink_timer: 0.0,
            next_blink: rng.gen_range((BLINK_INTERVAL_MEAN - BLINK_INTERVAL_VAR)..(BLINK_INTERVAL_MEAN + BLINK_INTERVAL_VAR)),
            eye_y_scale: 1.0,
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

    fn update_blink(&mut self, dt: f32) {
        self.blink_timer += dt;

        if self.blink_timer > self.next_blink {
            // Animation de clignement (Sinusoidal pour un aller-retour fluide)
            let blink_progress = (self.blink_timer - self.next_blink) * BLINK_SPEED;
            if blink_progress > std::f32::consts::PI {
                // Fin du clignement
                self.eye_y_scale = 1.0;
                self.blink_timer = 0.0;
                let mut rng = rand::thread_rng();
                self.next_blink = rng.gen_range((BLINK_INTERVAL_MEAN - BLINK_INTERVAL_VAR)..(BLINK_INTERVAL_MEAN + BLINK_INTERVAL_VAR));
            } else {
                // On écrase l'œil selon la courbe du sinus
                let wave = blink_progress.sin(); 
                self.eye_y_scale = 1.0 - (wave * (1.0 - BLINK_MIN_Y_SCALE));
            }
        }
    }
}

impl eframe::App for ShadyApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let dt = 1.0 / TARGET_FPS as f32;
        self.update_blink(dt);
        
        ctx.request_repaint_after(std::time::Duration::from_millis(1000 / TARGET_FPS));

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(BG_COLOR))
            .show(ctx, |ui| {
                let available_rect = ui.available_rect_before_wrap();
                let center_x = available_rect.center().x;
                let center_y = available_rect.center().y - 120.0;

                // On envoie le eye_y_scale au dessinateur
                self.current_eye_offset = draw_eyes(ctx, center_x, center_y, self.current_eye_offset, self.eye_y_scale);

                let terminal_rect = Rect::from_min_size(
                    Pos2::new(40.0, center_y + 120.0),
                    Vec2::new(available_rect.width() - 80.0, available_rect.height() - (center_y + 120.0) - 20.0),
                );

                ui.allocate_ui_at_rect(terminal_rect, |ui| {
                    ui.vertical(|ui| {
                        if !self.is_unlocked {
                            show_login_screen(ui, &mut self.api_key, &mut self.is_unlocked);
                        } else {
                            show_terminal_screen(ui, &mut self.history, &mut self.user_input);
                        }
                    });
                });
            });
    }
}
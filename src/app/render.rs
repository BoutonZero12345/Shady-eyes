// src/app/render.rs
use eframe::egui::{self, Color32, Key, Pos2, Rect};
use crate::app::ShadyApp;
use crate::app::types::{ApiPayload, SetupStep};
use crate::core::config::{BG_COLOR, TEXT_USER_COLOR, TEXT_SYSTEM_COLOR};
use crate::core::llm_config::LlmProvider;
use crate::ui::eyes::draw_eyes;

pub fn draw_ui(app: &mut ShadyApp, ctx: &egui::Context) {
    egui::CentralPanel::default().frame(egui::Frame::none().fill(BG_COLOR)).show(ctx, |ui| {
        let rect = ui.available_rect_before_wrap();
        
        // 1. DESSIN DES YEUX
        app.current_eye_offset = draw_eyes(ctx, rect.center().x, rect.center().y - 20.0, app.current_eye_offset, app.eye_y_scale);

        // 2. ZONE DE TEXTE (TERMINAL)
        let terminal_area = Rect::from_min_max(
            Pos2::new(15.0, 15.0), 
            Pos2::new(rect.max.x - 15.0, rect.max.y - 70.0)
        );

        ui.allocate_ui_at_rect(terminal_area, |ui| {
            ui.vertical(|ui| {
                if !app.status_message.contains("ONLINE") {
                    ui.colored_label(Color32::from_rgb(0, 255, 0), &app.status_message);
                    ui.add_space(10.0);
                }

                if app.is_setup && app.setup_step == SetupStep::SelectModel {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (i, m) in app.available_models.iter().enumerate() {
                            ui.label(format!("[{}] {}", i + 1, m));
                        }
                    });
                } else {
                    egui::ScrollArea::vertical().stick_to_bottom(true).show(ui, |ui| {
                        for (role, msg) in &app.history {
                            let color = if role == "User" || role == "user" {
                                TEXT_USER_COLOR
                            } else {
                                TEXT_SYSTEM_COLOR
                            };
                            ui.colored_label(color, format!("> {}", msg));
                            ui.add_space(4.0);
                        }
                        if app.is_waiting {
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
                let prompt = if app.is_setup {
                    if app.setup_step == SetupStep::InputKey { "SET API KEY > " } else { "SELECT # > " }
                } else {
                    "CHAT > "
                };
                
                ui.label(prompt);

                egui::ScrollArea::vertical()
                    .id_source("input_scroll_box")
                    .max_height(55.0) 
                    .show(ui, |ui| {
                        let output = egui::TextEdit::multiline(&mut app.user_input)
                            .desired_width(ui.available_width()) 
                            .desired_rows(3) 
                            .lock_focus(true)
                            .font(egui::FontId::monospace(14.0))
                            .show(ui);
                        
                        let edit = output.response;

                        if edit.has_focus() {
                            if let Some(cursor_range) = output.cursor_range {
                                let cursor_rect = output.galley.pos_from_cursor(&cursor_range.primary);
                                let screen_pos = edit.rect.min + cursor_rect.center().to_vec2();
                                ui.ctx().data_mut(|d| d.insert_temp(egui::Id::new("cursor_pos"), screen_pos));
                            }
                        } else {
                            ui.ctx().data_mut(|d| d.remove::<Pos2>(egui::Id::new("cursor_pos")));
                        }
                        
                        if edit.has_focus() && ui.input(|i| i.key_pressed(Key::Enter) && !i.modifiers.shift) {
                            let input = app.user_input.trim().to_string();
                            if !input.is_empty() {
                                if app.is_setup {
                                    match app.setup_step {
                                        SetupStep::InputKey => {
                                            app.api_key = input;
                                            app.provider = LlmProvider::detect(&app.api_key);
                                            app.status_message = "FETCHING MODELS...".to_string();
                                            let _ = app.tx.send(ApiPayload::FetchModels(app.api_key.clone(), app.provider.clone()));
                                            app.is_waiting = true;
                                        }
                                        SetupStep::SelectModel => {
                                            if let Ok(idx) = input.parse::<usize>() {
                                                if idx > 0 && idx <= app.available_models.len() {
                                                    app.model = app.available_models[idx-1].clone();
                                                    app.save_env();
                                                    app.is_setup = false;
                                                    app.status_message = "SYSTEM ONLINE".to_string();
                                                    let _ = app.tx.send(ApiPayload::Chat(
                                                        app.api_key.clone(), 
                                                        app.provider.clone(), 
                                                        app.model.clone(), 
                                                        vec![("user".into(), "Bonjour".into())]
                                                    ));
                                                    app.is_waiting = true;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    app.history.push(("User".to_string(), input.clone()));
                                    let _ = app.tx.send(ApiPayload::Chat(
                                        app.api_key.clone(), 
                                        app.provider.clone(), 
                                        app.model.clone(), 
                                        app.history.clone()
                                    ));
                                    app.is_waiting = true;
                                }
                            }
                            app.user_input.clear();
                            edit.request_focus();
                        }
                    }); 
            });
        });
    });
}
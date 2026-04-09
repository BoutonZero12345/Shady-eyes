use eframe::egui::{self, Color32, Pos2, Rect, ScrollArea, Stroke, Vec2};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum Role {
    System,
    User,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

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
        ctx.request_repaint();

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::from_rgb(10, 10, 10)))
            .show(ctx, |ui| {
                let available_rect = ui.available_rect_before_wrap();
                let center_x = available_rect.center().x;
                let center_y = available_rect.center().y - 120.0;

                let pointer_pos = ctx.pointer_hover_pos().unwrap_or(Pos2::new(center_x, center_y));
                let has_focus = ctx.memory(|mem| mem.focus().is_some());

                let eye_radius = 18.0;
                let eye_spacing = 90.0;
                let max_movement = 12.0;

                let mut draw_eye = |origin: Pos2| {
                    let mut target_x = origin.x;
                    let mut target_y = origin.y;

                    if has_focus {
                        target_y += max_movement;
                    } else {
                        let dx = pointer_pos.x - origin.x;
                        let dy = pointer_pos.y - origin.y;
                        let angle = dy.atan2(dx);
                        let dist = (dx * dx + dy * dy).sqrt();
                        let movement = f32::min(dist * 0.1, max_movement);
                        target_x += angle.cos() * movement;
                        target_y += angle.sin() * movement;
                    }

                    ui.painter().circle_filled(
                        Pos2::new(target_x, target_y),
                        eye_radius,
                        Color32::WHITE,
                    );
                };

                draw_eye(Pos2::new(center_x - eye_spacing / 2.0, center_y));
                draw_eye(Pos2::new(center_x + eye_spacing / 2.0, center_y));

                let terminal_rect = Rect::from_min_size(
                    Pos2::new(40.0, center_y + 80.0),
                    Vec2::new(
                        available_rect.width() - 80.0,
                        available_rect.height() - (center_y + 80.0) - 20.0,
                    ),
                );

                ui.allocate_ui_at_rect(terminal_rect, |ui| {
                    ui.vertical(|ui| {
                        ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);

                        if !self.is_unlocked {
                            ui.visuals_mut().override_text_color = Some(Color32::from_rgb(220, 50, 50));
                            ui.label("SYSTEM LOCKED. ENTER API KEY:");
                            ui.add_space(15.0);

                            let response = ui.add_sized(
                                [ui.available_width(), 30.0],
                                egui::TextEdit::singleline(&mut self.api_key)
                                    .frame(false)
                                    .password(true),
                            );

                            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                if !self.api_key.is_empty() {
                                    self.is_unlocked = true;
                                }
                            }
                        } else {
                            ScrollArea::vertical()
                                .auto_shrink([false; 2])
                                .max_height(ui.available_height() - 90.0)
                                .stick_to_bottom(true)
                                .show(ui, |ui| {
                                    for msg in &self.history {
                                        match msg.role {
                                            Role::System => {
                                                ui.visuals_mut().override_text_color = Some(Color32::from_rgb(180, 180, 180));
                                                ui.label(&msg.content);
                                            }
                                            Role::User => {
                                                ui.visuals_mut().override_text_color = Some(Color32::from_rgb(220, 50, 50));
                                                ui.label(format!("> {}", msg.content));
                                            }
                                        }
                                        ui.add_space(8.0);
                                    }
                                });

                            ui.add_space(10.0);

                            egui::Frame::none()
                                .stroke(Stroke::new(1.0, Color32::from_rgb(30, 30, 30)))
                                .inner_margin(12.0)
                                .rounding(4.0)
                                .show(ui, |ui| {
                                    ui.visuals_mut().override_text_color = Some(Color32::from_rgb(220, 50, 50));
                                    ui.visuals_mut().selection.cursor_bg = Color32::from_rgb(220, 50, 50);

                                    let response = ui.add_sized(
                                        [ui.available_width(), 50.0],
                                        egui::TextEdit::multiline(&mut self.user_input)
                                            .frame(false)
                                            .cursor_at_end(true)
                                            .hint_text("..."),
                                    );

                                    if response.has_focus() 
                                        && ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift) 
                                    {
                                        let text = self.user_input.trim().to_string();
                                        if !text.is_empty() {
                                            self.history.push(Message {
                                                role: Role::User,
                                                content: text,
                                            });
                                            self.history.push(Message {
                                                role: Role::System,
                                                content: "PROCESSING...".to_string(),
                                            });
                                            self.user_input.clear();
                                        }
                                    }
                                });
                        }
                    });
                });
            });
    }
}
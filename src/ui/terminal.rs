use eframe::egui::{self, ScrollArea, Stroke, Ui, Id, Vec2, FontId};
use crate::core::types::{Message, Role};
use crate::core::config::*;

pub fn show_terminal_screen(ui: &mut Ui, history: &mut Vec<Message>, user_input: &mut String) {
    ui.style_mut().override_font_id = Some(FontId::monospace(TERMINAL_FONT_SIZE));

    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .max_height(ui.available_height() - 120.0)
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            for msg in history.iter() {
                match msg.role {
                    Role::System => {
                        ui.visuals_mut().override_text_color = Some(TEXT_SYSTEM_COLOR);
                        ui.label(&msg.content);
                    }
                    Role::User => {
                        ui.visuals_mut().override_text_color = Some(TEXT_USER_COLOR);
                        ui.label(format!("> {}", msg.content));
                    }
                }
                ui.add_space(8.0);
            }
        });

    ui.add_space(10.0);

    egui::Frame::none()
        .stroke(Stroke::new(1.0, BORDER_COLOR))
        .inner_margin(12.0)
        .rounding(4.0)
        .show(ui, |ui| {
            ui.visuals_mut().override_text_color = Some(TEXT_USER_COLOR);

            let output = egui::TextEdit::multiline(user_input)
                .desired_width(f32::INFINITY)
                .min_size(Vec2::new(0.0, 90.0))
                .frame(false)
                .cursor_at_end(true)
                .hint_text(STR_HINT)
                .show(ui);

            let response = output.response;

            if let Some(cursor_range) = output.cursor_range {
                let cursor_rect = output.galley.pos_from_cursor(&cursor_range.primary);
                let screen_pos = response.rect.min + cursor_rect.center().to_vec2();
                ui.ctx().data_mut(|d| d.insert_temp(Id::new("cursor_pos"), screen_pos));
            }

            if response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift) {
                let text = user_input.trim().to_string();
                if !text.is_empty() {
                    history.push(Message { role: Role::User, content: text });
                    history.push(Message { role: Role::System, content: STR_PROCESSING.to_string() });
                    user_input.clear();
                }
            }
        });
}
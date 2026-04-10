use eframe::egui::{self, ScrollArea, Stroke, Ui};
use crate::core::types::{Message, Role};
use crate::core::config::{TEXT_SYSTEM_COLOR, TEXT_USER_COLOR, BORDER_COLOR};

pub fn show_terminal_screen(
    ui: &mut Ui,
    history: &mut Vec<Message>,
    user_input: &mut String,
) {
    ScrollArea::vertical()
        .auto_shrink([false; 2])
        .max_height(ui.available_height() - 90.0)
        .stick_to_bottom(true)
        .show(ui, |ui| {
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

            let response = ui.add_sized(
                [ui.available_width(), 50.0],
                egui::TextEdit::multiline(user_input)
                    .frame(false)
                    .cursor_at_end(true)
                    .hint_text("..."),
            );

            if response.has_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift)
            {
                let text = user_input.trim().to_string();
                if !text.is_empty() {
                    history.push(Message {
                        role: Role::User,
                        content: text,
                    });
                    
                    history.push(Message {
                        role: Role::System,
                        content: "PROCESSING...".to_string(),
                    });
                    
                    user_input.clear();
                }
            }
        });
}
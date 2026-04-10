use eframe::egui::{self, Ui, FontId};
use crate::core::config::{TEXT_USER_COLOR, STR_LOCKED, TERMINAL_FONT_SIZE};

pub fn show_login_screen(ui: &mut Ui, api_key: &mut String, is_unlocked: &mut bool) {
    ui.style_mut().override_font_id = Some(FontId::monospace(TERMINAL_FONT_SIZE));
    ui.visuals_mut().override_text_color = Some(TEXT_USER_COLOR);
    
    ui.label(STR_LOCKED); // Utilise la config
    ui.add_space(15.0);

    let response = ui.add_sized(
        [ui.available_width(), 30.0],
        egui::TextEdit::singleline(api_key)
            .frame(false)
            .password(true),
    );

    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
        if !api_key.is_empty() {
            *is_unlocked = true;
        }
    }
}
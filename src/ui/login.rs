use eframe::egui::{self, Ui};
use crate::core::config::TEXT_USER_COLOR;

pub fn show_login_screen(ui: &mut Ui, api_key: &mut String, is_unlocked: &mut bool) {
    ui.visuals_mut().override_text_color = Some(TEXT_USER_COLOR);
    ui.label("SYSTEM LOCKED. ENTER API KEY:");
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
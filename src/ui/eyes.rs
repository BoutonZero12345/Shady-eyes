use eframe::egui::{Color32, Context, Pos2, Rect, Vec2};
use crate::core::config::*;

pub fn draw_eyes(ctx: &Context, center_x: f32, center_y: f32, user_input: &str) {
    let pointer_pos = ctx.pointer_hover_pos().unwrap_or(Pos2::new(center_x, center_y));
    let has_focus = ctx.memory(|mem| mem.focused().is_some());

    // --- Calcul du Regard Parallèle Absolu ---
    let dir_vec = Vec2::new(pointer_pos.x - center_x, pointer_pos.y - center_y);
    let look_dir = if dir_vec.length() > 0.0 { dir_vec.normalized() } else { Vec2::ZERO };
    let dist = dir_vec.length();

    let look_offset = look_dir * (dist * 0.1).min(MAX_EYE_MOVEMENT);

    // --- Logique Saisie / Souris ---
    let (target_offset_x, target_offset_y) = if has_focus {
        let lines: Vec<&str> = user_input.split('\n').collect();
        let current_line_len = lines.last().unwrap_or(&"").len() as f32;
        let text_progress = (current_line_len / 50.0).min(1.0);
        
        let x_offset = -MAX_EYE_MOVEMENT + (text_progress * MAX_EYE_MOVEMENT * 2.0);
        (x_offset, MAX_EYE_MOVEMENT) 
    } else {
        (look_offset.x, look_offset.y) 
    };

    let origins = [
        Pos2::new(center_x - EYE_SPACING / 2.0, center_y),
        Pos2::new(center_x + EYE_SPACING / 2.0, center_y),
    ];

    for origin in origins {
        let current_pupil_center = origin + Vec2::new(target_offset_x, target_offset_y);

        // 1. Dessin de l'Aura
        draw_pixelated_circle(
            ctx,
            current_pupil_center,
            AURA_RADIUS_PIXELS,
            Color32::from_white_alpha(AURA_ALPHA_DECAY),
        );

        // 2. Dessin de la Pupille
        draw_pixelated_circle(
            ctx,
            current_pupil_center,
            PUPIL_RADIUS_PIXELS,
            Color32::WHITE,
        );
    }
}

// Fonction utilitaire pour dessiner un cercle pixelisé
fn draw_pixelated_circle(ctx: &Context, center: Pos2, radius_in_pixels: isize, color: Color32) {
    let r2 = radius_in_pixels * radius_in_pixels;

    // LA SOLUTION EST ICI :
    // On réduit la limite de la boucle de 1. 
    // Cela empêche de dessiner les 4 pixels solitaires qui dépassent, sans altérer le calcul du cercle.
    let limit = radius_in_pixels - 1;

    for y in -limit..=limit {
        for x in -limit..=limit {
            if x * x + y * y <= r2 {
                let pixel_center = center + Vec2::new(
                    x as f32 * PIXEL_SIZE,
                    y as f32 * PIXEL_SIZE,
                );
                
                let rect = Rect::from_center_size(
                    pixel_center,
                    Vec2::splat(PIXEL_SIZE),
                );

                ctx.layer_painter(eframe::egui::LayerId::background()).rect_filled(
                    rect,
                    0.0,
                    color,
                );
            }
        }
    }
}
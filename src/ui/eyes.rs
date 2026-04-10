use eframe::egui::{Color32, Context, Pos2, Rect, Stroke, Vec2, LayerId, Id};
use crate::core::config::*;

pub fn draw_eyes(ctx: &Context, center_x: f32, center_y: f32, current_offset: Vec2, y_scale: f32) -> Vec2 {
    let pointer_pos = ctx.pointer_hover_pos().unwrap_or(Pos2::new(center_x, center_y));
    let has_focus = ctx.memory(|mem| mem.focused().is_some());
    let cursor_pos: Option<Pos2> = ctx.data(|d| d.get_temp(Id::new("cursor_pos")));

    let target_offset = if has_focus && cursor_pos.is_some() {
        let cp = cursor_pos.unwrap();
        let dir_to_cursor = Vec2::new(cp.x - center_x, cp.y - center_y);
        let look_dir = dir_to_cursor.normalized();
        let dist = dir_to_cursor.length();
        look_dir * (dist * EYE_TRACKING_SENSITIVITY).min(MAX_EYE_MOVEMENT)
    } else {
        let dir_vec = Vec2::new(pointer_pos.x - center_x, pointer_pos.y - center_y);
        let look_dir = if dir_vec.length() > 0.0 { dir_vec.normalized() } else { Vec2::ZERO };
        let dist = dir_vec.length();
        look_dir * (dist * 0.1).min(MAX_EYE_MOVEMENT)
    };

    let smoothed_offset = current_offset + (target_offset - current_offset) * EYE_SMOOTHING;

    if has_focus {
        if let Some(cp) = cursor_pos {
            ctx.layer_painter(LayerId::debug()).line_segment(
                [Pos2::new(center_x, center_y), cp],
                Stroke::new(1.0, LASER_COLOR.linear_multiply(LASER_ALPHA as f32 / 255.0)),
            );
        }
    }

    let origins = [
        Pos2::new(center_x - EYE_SPACING / 2.0, center_y),
        Pos2::new(center_x + EYE_SPACING / 2.0, center_y),
    ];

    for origin in origins {
        let pupil_center = origin + smoothed_offset;
        
        // On passe le y_scale à la fonction de dessin
        draw_pixelated_circle(
            ctx, 
            pupil_center, 
            AURA_RADIUS_PIXELS, 
            Color32::from_white_alpha(GLOW_ALPHA),
            GLOW_PIXEL_OVERLAP,
            y_scale
        );
        
        draw_pixelated_circle(
            ctx, 
            pupil_center, 
            PUPIL_RADIUS_PIXELS, 
            Color32::WHITE,
            PUPIL_PIXEL_OVERLAP,
            y_scale
        );
    }

    smoothed_offset
}

fn draw_pixelated_circle(ctx: &Context, center: Pos2, radius: isize, color: Color32, overlap: f32, y_scale: f32) {
    let r_f32 = radius as f32;
    let r2 = r_f32 * r_f32;
    
    // ON AJOUTE CETTE LIGNE :
    let limit = radius - 1; 

    // ET ON UTILISE "limit" DANS LES DEUX BOUCLES ICI :
    for y in -limit..=limit {
        let scaled_y = y as f32 / y_scale.max(0.001); 
        
        for x in -limit..=limit {
            if (x as f32).powi(2) + scaled_y.powi(2) <= r2 {
                let pixel_center = center + Vec2::new(x as f32 * PIXEL_SIZE, y as f32 * PIXEL_SIZE);
                let rect = Rect::from_center_size(pixel_center, Vec2::splat(PIXEL_SIZE + overlap));
                ctx.layer_painter(LayerId::background()).rect_filled(rect, 0.0, color);
            }
        }
    }
}
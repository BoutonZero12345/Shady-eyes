// src/app/logic.rs
use std::f32::consts::PI;
use rand::Rng;
use crate::app::ShadyApp;
use crate::app::types::{ApiResponse, SetupStep};

pub fn update_state(app: &mut ShadyApp) {
    // --- GESTION DU TEMPS (BLINK) ---
    let dt = 0.016; 
    app.blink_timer += dt;
    if app.blink_timer > app.next_blink {
        let blink_progress = (app.blink_timer - app.next_blink) * 15.0;
        if blink_progress > PI {
            app.eye_y_scale = 1.0;
            app.blink_timer = 0.0;
            app.next_blink = rand::thread_rng().gen_range(2.0..6.0);
        } else {
            app.eye_y_scale = 1.0 - (blink_progress.sin() * 0.95);
        }
    }

    // --- RÉCEPTION DES RÉPONSES API ---
    if let Ok(res) = app.rx.try_recv() {
        app.is_waiting = false;
        match res {
            ApiResponse::ModelsFetched(m) => {
                app.available_models = m;
                app.setup_step = SetupStep::SelectModel;
                app.status_message = "SELECT MODEL [1, 2, 3...]".to_string();
            }
            ApiResponse::ChatResponse(t) => {
                app.history.push(("Sum Sum".to_string(), t));
                app.status_message = "ONLINE".to_string();
            }
            ApiResponse::Error(e) => {
                app.status_message = format!("ERROR: {}", e);
            }
        }
    }
}
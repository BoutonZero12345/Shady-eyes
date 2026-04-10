use eframe::egui::Color32;

// --- CONFIGURATION FENÊTRE ---
pub const WINDOW_TITLE: &str = "Sum Sum";           // Le petit nom de ton compagnon
pub const WINDOW_WIDTH: f32 = 300.0;              // Largeur de la fenêtre (pixels)
pub const WINDOW_HEIGHT: f32 = 200.0;             // Hauteur de la fenêtre (pixels)
pub const BG_COLOR: Color32 = Color32::from_rgb(10, 10, 10); // Couleur du fond (Noir gris)

// --- SYSTÈME & TEXTE ---
pub const TARGET_FPS: u64 = 120;                   // Vitesse de rafraîchissement (24 = saccadé, 60 = fluide)
pub const TERMINAL_FONT_SIZE: f32 = 15.0;         // Taille de la police de caractères
pub const STR_LOCKED: &str = "SYSTEM LOCKED. ENTER API KEY:"; // Message écran de verrouillage
pub const STR_PROCESSING: &str = "PROCESSING..."; // Message pendant que l'IA réfléchit
pub const STR_HINT: &str = "...";                 // Texte d'aide dans la barre de saisie
pub const STR_WELCOME: &str = "HOW CAN I HELP YOU TODAY?"; // Premier message au démarrage

// --- PHYSIQUE DES YEUX ---
pub const EYE_SPACING: f32 = 180.0;               // Écartement entre les deux yeux
pub const MAX_EYE_MOVEMENT: f32 = 64.0;           // Rayon de déplacement max des pupilles
pub const EYE_TRACKING_SENSITIVITY: f32 = 0.5;   // Force du suivi (0.1 bas, 0.5 très nerveux)
pub const EYE_SMOOTHING: f32 = 0.05;              // VITESSE DE DÉPLACEMENT (0.01 lent, 1.0 instantané)

// --- RÉGLAGES DU CLIGNEMENT (BLINK) ---
pub const BLINK_INTERVAL_MEAN: f32 = 3.0;         // Temps moyen entre deux clignements (secondes)
pub const BLINK_INTERVAL_VAR: f32 = 2.0;          // Variation aléatoire (+/- secondes)
pub const BLINK_SPEED: f32 = 50.0;                // Vitesse de fermeture/ouverture (plus haut = plus rapide)
pub const BLINK_MIN_Y_SCALE: f32 = 0.05;          // Écrasement vertical max (0.0 = fermé total)

// --- RENDU PIXEL (8-BIT) ---
pub const PIXEL_SIZE: f32 = 4.0;                  // Taille d'un bloc de l'œil (en pixels réels)
pub const PUPIL_PIXEL_OVERLAP: f32 = 0.5;         // Surplus pour la pupille (supprime le quadrillage)
pub const GLOW_PIXEL_OVERLAP: f32 = -2.0;          // Surplus pour l'aura (plus grand pour plus de flou)
pub const PUPIL_RADIUS_PIXELS: isize = 9;         // Rayon de la pupille (en nombre de blocs)
pub const AURA_RADIUS_PIXELS: isize = 13;         // Rayon de l'aura (en nombre de blocs)

// --- EFFETS VISUELS & COULEURS ---
pub const GLOW_ALPHA: u8 = 15;                    // Transparence de l'aura (0 invisible, 255 opaque)
pub const LASER_ALPHA: u8 = 0;                  // Transparence du trait vert (0 à 255)
pub const LASER_COLOR: Color32 = Color32::from_rgb(0, 255, 0); // Couleur du trait de visée (Vert)
pub const TEXT_SYSTEM_COLOR: Color32 = Color32::from_rgb(180, 180, 180); // Couleur réponse IA (Gris)
pub const TEXT_USER_COLOR: Color32 = Color32::from_rgb(0, 255, 0);     // Couleur ton texte (Rouge)
pub const BORDER_COLOR: Color32 = Color32::from_rgb(30, 30, 30);         // Couleur cadre de saisie
pub const EYE_COLOR: Color32 = Color32::from_rgb(255, 255, 255); // JAUNE SOLEIL (Sum Sum !)
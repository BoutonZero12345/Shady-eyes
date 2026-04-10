use eframe::egui::Color32;

// ==========================================
// PARAMÈTRES DE LA FENÊTRE PRINCIPALE
// ==========================================

/// Largeur de la fenêtre au démarrage (en pixels réels)
pub const WINDOW_WIDTH: f32 = 600.0;

/// Hauteur de la fenêtre au démarrage (en pixels réels)
pub const WINDOW_HEIGHT: f32 = 500.0;

/// Couleur de fond de l'application (Rouge, Vert, Bleu - de 0 à 255)
/// Actuellement : Un gris extrêmement sombre, presque noir.
pub const BG_COLOR: Color32 = Color32::from_rgb(10, 10, 10);


// ==========================================
// PARAMÈTRES D'ANIMATION ET DE POSITION
// ==========================================

/// Vitesse de l'animation. 24 donne un style "cinéma/rétro" légèrement saccadé.
/// 60 donnera un mouvement parfaitement fluide.
pub const TARGET_FPS: u64 = 24;

/// Distance (en pixels réels) entre le centre de l'œil gauche et l'œil droit.
/// Augmente cette valeur pour les écarter.
pub const EYE_SPACING: f32 = 140.0;

/// Distance maximale (en pixels réels) à laquelle les yeux peuvent se déplacer 
/// pour suivre la souris ou le texte.
pub const MAX_EYE_MOVEMENT: f32 = 90.0;


// ==========================================
// PARAMÈTRES DU RENDU PIXELISÉ (STYLE 8-BIT)
// ==========================================

/// La taille d'un "bloc" à l'écran. 
/// Si tu mets 4.0, chaque pixel de l'œil sera un carré de 4x4 vrais pixels.
/// Augmente pour un effet Minecraft, diminue pour un effet plus lisse.
pub const PIXEL_SIZE: f32 = 4.0;

/// Le rayon de la pupille blanche, compté en "gros pixels".
/// Exemple: 7 signifie que la pupille fait 14 gros pixels de large.
pub const PUPIL_RADIUS_PIXELS: isize = 7;

/// Le rayon de l'aura grise/transparente, compté en "gros pixels".
/// Doit toujours être strictement supérieur à PUPIL_RADIUS_PIXELS.
pub const AURA_RADIUS_PIXELS: isize = 12;

/// Opacité de l'aura (de 0 = invisible, à 255 = totalement opaque).
/// 15 donne cet effet de "lueur" très discrète dans le noir.
pub const AURA_ALPHA_DECAY: u8 = 15;


// ==========================================
// PARAMÈTRES DE L'INTERFACE TEXTE (TERMINAL)
// ==========================================

/// Couleur du texte quand le Système (l'IA) parle.
/// Actuellement : Gris clair.
pub const TEXT_SYSTEM_COLOR: Color32 = Color32::from_rgb(180, 180, 180);

/// Couleur du texte quand l'Utilisateur (toi) parle, ou pour la saisie.
/// Actuellement : Rouge terminal.
pub const TEXT_USER_COLOR: Color32 = Color32::from_rgb(220, 50, 50);

/// Couleur de la fine ligne autour de la zone où tu écris ton texte.
/// Actuellement : Gris très sombre, pour rester discret.
pub const BORDER_COLOR: Color32 = Color32::from_rgb(30, 30, 30);
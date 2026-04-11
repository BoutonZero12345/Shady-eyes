# SHADY EYES : SUM SUM

```
      ___ _             _        ___   
     / __| |_  __ _  __| |_  _  | __|_  _ ___ ___  
     \__ \ ' \/ _` |/ _` | || | | _|| || / -_|_-<  
     |___/_||_\__,_|\__,_|\_, | |___|\_, \___/__/  
                          |__/       |__/
```

VERSION : 1.0.0

![Shady Eyes](./the%20shady%20eyes.png)

Un esprit pixelisé vivant dans un terminal. Shady Eyes est une application de bureau développée en Rust (via eframe/egui), intégrant un compagnon IA nommé Sum Sum (ou Summer). Elle propose une interface rétro-terminal minimaliste et un système de rendu d'yeux interactifs.

## FONCTIONNALITÉS PRINCIPALES

* [X] Interface Rétro : Design épuré, typographie monospace et interface sans fioritures inspirée des terminaux classiques.
* [X] Yeux Interactifs : Système de rendu de yeux pixelisés avec suivi fluide de la saisie utilisateur et physique de clignements aléatoires.
* [X] Cerveau IA Asynchrone : Communication non-bloquante avec les modèles LLM via API, garantissant une interface toujours réactive.
* [X] Gestion Automatisée : Sauvegarde et restauration automatiques de la clé API et du modèle choisi (fichier .env local).

---

## PRÉREQUIS

Le compilateur Rust et son gestionnaire de paquets Cargo doivent être installés sur votre machine.

> Installation officielle : https://www.rust-lang.org/tools/install

---

## DÉPLOIEMENT ET EXÉCUTION

### 1. Lancement classique

Pour compiler et lancer l'application avec les performances optimales (fortement recommandé pour la fluidité des animations). Si vous vous êtes déjà connecté, l'application se souviendra de votre configuration.

cargo run --release

### 2. Réinitialisation du système (Hard Reset)

L'application sauvegarde vos identifiants dans un fichier caché .env. Pour forcer l'application à purger sa mémoire et afficher l'écran de configuration initial au démarrage, exécutez l'une des commandes suivantes selon votre OS :

Sous Windows (PowerShell) :
Remove-Item .env -ErrorAction SilentlyContinue; cargo run --release

Sous Linux / macOS :
rm -f .env && cargo run --release

---

## ARCHITECTURE DU PROJET

L'application suit une architecture modulaire séparant strictement la logique d'état, l'interface graphique et les appels réseau.

shady_eyes/
├── src/
│   ├── api/
│   │   └── client.rs       # Client HTTP asynchrone pour les fournisseurs LLM
│   ├── core/
│   │   ├── config.rs       # Constantes physiques, couleurs et prompt système
│   │   ├── llm_config.rs   # Détection et formatage des requêtes API
│   │   └── types.rs        # Structures de données (Messages, Rôles)
│   ├── ui/
│   │   ├── eyes.rs         # Moteur de rendu mathématique des yeux pixelisés
│   │   ├── login.rs        # Interface de configuration initiale
│   │   └── terminal.rs     # Zone d'affichage du chat
│   ├── app.rs               # Boucle principale (State Machine) et routage
│   └── main.rs             # Point d'entrée et configuration de la fenêtre eframe
├── Cargo.toml              # Dépendances et métadonnées du projet
└── README.md               # Documentation

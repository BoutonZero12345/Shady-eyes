


# Shady Eyes - Sum Sum

Un esprit pixelisé vivant dans un terminal. Application de bureau développée en Rust avec `eframe`/`egui`, intégrant un compagnon IA (Sum Sum / Summer) avec des yeux interactifs et une interface style rétro-terminal.


## Fonctionnalités

* **Interface Rétro :** Design épuré, typographie monospace et interface sans fioritures inspirée des terminaux classiques.
* **Yeux Interactifs :** Système de rendu de yeux pixelisés avec suivi fluide et physique de clignements aléatoires.
* **Cerveau IA :** Communication asynchrone avec les modèles LLM via API.
* **Gestion de Configuration :** Sauvegarde automatique de la clé API et du modèle choisi.


## Prérequis

* [Rust et Cargo](https://www.rust-lang.org/tools/install) installés sur votre machine.


## Commandes d'exécution


### 1. Lancement classique

Pour compiler et lancer l'application avec les performances optimales. Si vous vous êtes déjà connecté, l'application se souviendra de votre clé et de votre modèle.

```
powershell
cargo run --release
```



### 2. Réinitialisation de l'accès (Changer de Clé API / Modèle)

L'application sauvegarde vos identifiants dans un fichier local `.env`. Pour forcer l'application à "oublier" ces informations et afficher l'écran de sélection de l'API au démarrage, exécutez la commande suivante :

```
powershell
Remove-Item .env -ErrorAction SilentlyContinue; cargo run --release
```



## Structure du projet

* src/main.rs : Point d'entrée de l'application.
* src/app.rs : Logique principale, gestion des états et communication asynchrone avec l'API.
* src/ui/ : Composants de l'interface graphique (yeux, écran de connexion, terminal).
* src/core/ : Configuration globale (couleurs, dimensions, prompt système) et types de données.
* src/api/ : Client HTTP pour communiquer avec les fournisseurs d'IA.

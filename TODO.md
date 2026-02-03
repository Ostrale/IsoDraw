# TODO IsoDraw

Liste des fonctionnalités à implémenter

## 🏗️ Architecture de Base

### Système de Shapes Extensible
- [ ] Créer un trait `Shape` pour gérer différents types de formes
- [ ] Transformer le `Shape` actuel en `Cuboid`
- [ ] Préparer l'ajout de Cylinder, Sphere (plus tard)
- [ ] Créer `src/models/shape.rs`
- [ ] Créer `src/models/cuboid.rs`

### Document et Layers
- [ ] Créer la structure `Document` pour gérer tout le projet
- [ ] Implémenter un système de layers (ordre Z)
- [ ] Permettre de réorganiser l'ordre des shapes (devant/derrière)
- [ ] Créer `src/models/document.rs`

### Système de Folders
- [ ] Créer la structure `Folder` pour regrouper des shapes
- [ ] Support des folders imbriqués (folders dans folders)
- [ ] Déplacer tout un folder d'un coup
- [ ] Sélectionner individuellement un shape dans un folder
- [ ] Créer `src/models/folder.rs`

## 🔄 Gestion de l'État

### État Global
- [ ] Créer `AppState` avec le document, la sélection, l'outil actif
- [ ] Ajouter un clipboard pour copier/coller
- [ ] Créer `src/state/app_state.rs`

### Sélection
- [ ] Implémenter la sélection simple (clic sur un shape)
- [ ] Implémenter la sélection multiple (Ctrl+clic)
- [ ] Highlight visuel des shapes sélectionnés
- [ ] Créer `src/state/selection.rs`

### Undo/Redo
- [ ] Créer le système d'historique
- [ ] Sauvegarder un snapshot avant chaque modification
- [ ] Raccourci Ctrl+Z pour annuler
- [ ] Raccourci Ctrl+Y pour refaire
- [ ] Créer `src/state/history.rs`

## 🖱️ Interactions Souris

### Détection de Clic
- [ ] Détecter le clic sur un shape
- [ ] Gérer les événements mousedown, mousemove, mouseup
- [ ] Convertir coordonnées écran → isométrique
- [ ] Créer `src/interactions/mouse_handler.rs`

### Ajouter des Shapes
- [ ] Mode "Ajouter un shape" activable
- [ ] Clic sur la grille pour placer un nouveau cuboid
- [ ] Prévisualisation avant placement

### Déplacer des Shapes
- [ ] Drag & drop pour déplacer un shape
- [ ] Déplacer plusieurs shapes en même temps
- [ ] Déplacer tout un folder
- [ ] Snap to grid
- [ ] Créer `src/interactions/transform.rs`

### Redimensionner des Shapes
- [ ] Afficher 6 poignées de redimensionnement quand un shape est sélectionné
  - [ ] Width (largeur vers +X)
  - [ ] Depth (profondeur vers +Z)
  - [ ] Height (hauteur vers +Y)
  - [ ] Anti-Width (vers -X)
  - [ ] Anti-Depth (vers -Z)
  - [ ] Anti-Height (vers -Y)
- [ ] Drag sur une poignée pour redimensionner
- [ ] Feedback visuel pendant le redimensionnement

## 🎨 Interface Utilisateur

### Panneau de Propriétés
- [ ] Créer le composant `PropertiesPanel`
- [ ] Afficher les dimensions (Width, Height, Depth) du shape sélectionné
- [ ] Inputs numériques pour modifier les dimensions
- [ ] Section style : couleur, opacité, stroke
- [ ] Position (X, Y, Z)
- [ ] Créer `src/components/panels/properties_panel.rs`

### Panneau de Layers
- [ ] Créer le composant `LayersPanel`
- [ ] Afficher la liste hiérarchique des shapes et folders
- [ ] Drag & drop pour réorganiser l'ordre Z
- [ ] Boutons "Move Up" / "Move Down"
- [ ] Icônes visibilité et verrouillage
- [ ] Créer `src/components/panels/layers_panel.rs`

### Toolbar
- [ ] Créer le composant `Toolbar`
- [ ] Boutons : Select, Add Shape, Delete
- [ ] Boutons Undo/Redo
- [ ] Boutons Copy/Cut/Paste
- [ ] Boutons Export SVG/PNG
- [ ] Boutons Save/Load
- [ ] Créer `src/components/panels/toolbar.rs`

### Menu Clic Droit
- [ ] Créer le composant `ContextMenu`
- [ ] Apparaît au clic droit sur un shape
- [ ] Options : Copy, Cut, Paste, Delete, Duplicate
- [ ] Bring to Front / Send to Back
- [ ] Group (créer un folder)
- [ ] Créer `src/components/context_menu.rs`

## 📋 Opérations de Manipulation

### Copier/Coller/Couper
- [ ] Implémenter Copy (Ctrl+C)
- [ ] Implémenter Cut (Ctrl+X)
- [ ] Implémenter Paste (Ctrl+V)
- [ ] Support pour copier des folders entiers
- [ ] Créer `src/utils/clipboard.rs`

### Autres Opérations
- [ ] Suppression (touche Delete)
- [ ] Duplication (Ctrl+D)
- [ ] Confirmation pour suppressions importantes

## 💾 Sauvegarde et Export

### Format de Projet
- [ ] Définir le format JSON (.isodraw)
- [ ] Implémenter la sérialisation avec serde
- [ ] Versioning du format
- [ ] Créer `src/serialization/project.rs`

### Save/Load
- [ ] Bouton "Save" pour télécharger le fichier .isodraw
- [ ] Bouton "Load" pour charger un fichier .isodraw
- [ ] Gestion des erreurs de chargement
- [ ] Auto-save dans localStorage (optionnel)

### Export SVG
- [ ] Générer un SVG complet du document
- [ ] Respecter l'ordre des layers
- [ ] Bouton pour télécharger le .svg
- [ ] Créer `src/serialization/export_svg.rs`

### Export PNG
- [ ] Convertir SVG en PNG
- [ ] Bouton pour télécharger le .png
- [ ] Résolution configurable
- [ ] Créer `src/serialization/export_png.rs`

## ✨ Améliorations

### Raccourcis Clavier
- [ ] Ctrl+Z : Undo
- [ ] Ctrl+Y : Redo
- [ ] Ctrl+C : Copy
- [ ] Ctrl+X : Cut
- [ ] Ctrl+V : Paste
- [ ] Ctrl+D : Duplicate
- [ ] Delete : Supprimer
- [ ] Ctrl+S : Save
- [ ] Ctrl+A : Sélectionner tout
- [ ] Esc : Désélectionner

### Polish
- [ ] Messages d'erreur utilisateur
- [ ] Feedback visuel sur toutes les actions
- [ ] Optimisation des performances
- [ ] Thème clair/sombre (optionnel)

## 🚀 Fonctionnalités Futures

### Nouveaux Types de Shapes
- [ ] Cylinder
- [ ] Sphere
- [ ] Pyramid
- [ ] Custom polygons

### Fonctionnalités Avancées
- [ ] Rotation des shapes
- [ ] Textures et patterns
- [ ] Ombres et lumières
- [ ] Animations
- [ ] Alignement et distribution
- [ ] Guides et règles

---

## 📝 Notes

### Dépendances à ajouter au Cargo.toml
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
web-sys = { version = "0.3", features = ["File", "FileReader", "Blob", "Url"] }
```

### Ordre Recommandé
1. Architecture de Base (Shapes, Document, Folders)
2. Gestion de l'État (AppState, Sélection, History)
3. Interactions Souris (Clic, Déplacement, Redimensionnement)
4. Interface Utilisateur (Panneaux)
5. Opérations (Copy/Paste, Delete)
6. Sauvegarde et Export
7. Améliorations et Polish


# Notes Manager

**Notes Manager** est une application en ligne de commande écrite en Rust pour gérer des notes. Elle permet d'ajouter, lister, lire et supprimer des notes via des commandes simples. Les notes sont stockées dans un fichier JSON pour une gestion facile et sécurisée.

## Table des Matières

- [Prérequis](#prérequis)
- [Installation](#installation)
- [Utilisation](#utilisation)
- [Exemple](#exemple)
- [Licence](#licence)

## Prérequis

Avant de pouvoir utiliser **Notes Manager**, vous devez avoir [Rust](https://www.rust-lang.org/learn/get-started) installé sur votre système.

## Installation

1. **Clonez le dépôt :**

   ```bash
   git clone https://github.com/yourusername/notes_manager.git
   ```

2. **Naviguez dans le répertoire du projet :**

   ```bash
   cd notes_manager
   ```

3. **Installez les dépendances :**

   ```bash
   cargo build
   ```

   Cette commande télécharge et compile les dépendances nécessaires pour le projet.

## Utilisation

Après avoir installé le projet, vous pouvez utiliser les commandes suivantes :

### Ajouter une note

Ajoute une nouvelle note avec un titre et un contenu.

```bash
cargo run -- add "Titre de la note" "Contenu de la note"
```

### Lister toutes les notes

Affiche toutes les notes existantes.

```bash
cargo run -- list
```

### Lire une note spécifique par ID

Affiche le contenu d'une note spécifique en utilisant son ID.

```bash
cargo run -- read <ID>
```

### Supprimer une note spécifique par ID

Supprime une note spécifique en utilisant son ID.

```bash
cargo run -- delete <ID>
```

## Exemple

Voici quelques exemples de commandes :

- Ajouter une note :

  ```bash
  cargo run -- add "Meeting Notes" "Discuss project roadmap and milestones."
  ```

- Lister les notes :

  ```bash
  cargo run -- list
  ```

- Lire une note avec ID 1 :

  ```bash
  cargo run -- read 1
  ```

- Supprimer une note avec ID 1 :

  ```bash
  cargo run -- delete 1
  ```

## Licence

Ce projet est sous Aucune licence. 

---

Merci d'utiliser **Notes Manager** ! Si vous avez des questions ou des suggestions


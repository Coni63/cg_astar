# A-Star-Craft

Solveur en Rust pour le puzzle CodinGame [A-Star-Craft](https://github.com/CodinGameCommunity/A-Star-Craft), résolu par recuit simulé (simulated annealing).

## But du jeu

Marquez autant de points que possible en guidant vos robots sur les plateformes.

## Règles

Vous devez guider des robots `Automaton2000` en plaçant des flèches au premier tour. À chaque tour vous marquez autant de points qu'il y a de robots fonctionnels.

**La carte**

- Le jeu se joue sur une grille de `19` unités de largeur et `10` unités de hauteur.
- La grille est sous forme de **tore**. C'est-à-dire que quitter la grille par un côté vous fait réapparaître de l'autre côté.
- La cellule `X=0, Y=0` se trouve en haut à gauche.
- Chaque cellule est d'un type parmi trois : plateforme, flèche ou vide.

**Automaton2000**

- Automaton2000 avance en ligne droite d'une cellule par tour.
- Automaton2000 ne change de direction que s'il marche sur une flèche.
- Automaton2000 cesse de fonctionner s'il visite un état (position, direction) déjà visité.
- Automaton2000 cesse de fonctionner s'il marche hors de la plateforme.

**Flèches**

- Les flèches pointent vers l'une des quatre directions : `U` (haut), `R` (droite), `D` (bas) et `L` (gauche).
- Les flèches doivent être placées sur des cellules de plateforme sans flèche. Toute autre action sera ignorée.

**Sortie**

- Placez une flèche en sortant la commande `X Y d` où `d` est la direction (`U`, `R`, `D` ou `L`) et `X`, `Y` sont les coordonnées de la cellule.
- Placez de multiples flèches en séparant les commandes par des espaces.

## Détails

**Ordre des événements lors d'un tour :**

1. Le score est incrémenté de 1 pour chaque robot en vie.
2. Les Automaton2000 avancent d'une case dans la direction vers laquelle ils font face.
3. Les Automaton2000 changent de direction s'ils sont sur une flèche.
4. Les Automaton2000 meurent s'ils ont marché dans le vide ou s'ils sont dans un état (position, direction) déjà visité (les Automaton2000 ne partagent pas leur historique d'états).

Au premier tour, Automaton2000 change de direction s'il est sur une flèche (i.e. vous pouvez changer la direction initiale d'Automaton2000 en plaçant une flèche sous lui).

## Entrées du jeu

**Entrées pour un tour de jeu**

- `10` lignes : une chaîne de caractères de longueur `19` dont chaque caractère représente :
  - `#` : cellule de vide
  - `.` : cellule de plateforme vide
  - `U` : cellule de plateforme avec une flèche vers le haut
  - `R` : cellule de plateforme avec une flèche vers la droite
  - `D` : cellule de plateforme avec une flèche vers le bas
  - `L` : cellule de plateforme avec une flèche vers la gauche
- Ligne suivante : un entier `robotCount`, le nombre total de robots Automaton2000.
- `robotCount` lignes suivantes : pour chaque robot Automaton2000,
  - 2 entiers `x` et `y`, ses coordonnées,
  - 1 caractère `direction`, la direction vers laquelle il fait face (`U`, `R`, `D` ou `L`).

**Sortie pour un tour de jeu**

Une ligne composée de triplets de la forme `X Y DIR` avec `X Y` deux entiers et `DIR` un caractère, soit `U`, `R`, `D` ou `L`.

**Contraintes**

- `1 ≤ robotCount ≤ 10`.
- Nombre de caractères maximum autorisé pour une sortie : `10 000`.
- Temps de réponse ≤ `1s`.

## Algorithme (`src/solver.rs`)

Recuit simulé sur la grille de flèches, ~900 ms au total :

- **Prétraitement** : flèche forcée dans les culs-de-sac ; sur chaque cellule, seules les flèches ne pointant pas vers le vide sont proposées (les demi-tours dans les couloirs restent autorisés, ils rapportent des points).
- **Simulation rapide** : table de transitions `état (cellule, direction) -> état suivant`, mise à jour en O(1) quand une cellule change.
- **Évaluation incrémentale** : seuls les robots passant par une cellule modifiée sont resimulés, et uniquement à partir du premier pas où ils l'atteignent.
- **Symétrie** : si la carte et les robots sont invariants par rotation de 180° (sur le tore), la phase d'exploration ne cherche que des solutions symétriques (espace de recherche et coût de simulation divisés par deux). Les symétries miroir ont été testées : elles dégradent fortement le score.
- **Composantes connexes** : le meilleur état est conservé par composante indépendante et recombiné entre tous les recuits.
- **Planning** : 2 recuits d'exploration depuis la grille vide (60 % du temps, T 10 → 2), puis un raffinement non symétrique de la meilleure solution (T 3 → 2).

## Structure du projet

```
src/            Solveur Rust (board, cell, loader, robot, solution, solver)
tests/          Jeux de tests (30 cartes, format texte)
A-Star-Craft/   Copie du référentiel officiel de l'arbitre (config, inputs, validateur Java)
full_bench.py   Script de benchmark exécutant tous les jeux de tests
```

## Compiler et lancer

```sh
cargo build --release
cargo run --release < tests/01-simple.txt
```

Le score final et le déroulé de la résolution sont affichés sur `stderr`, la solution (liste de flèches à placer) sur `stdout`. Le score est recalculé par le simulateur de référence de `main.rs` (un `WARNING` est affiché en cas d'écart).

## Lancer les tests

Le dossier `tests/` contient 30 jeux de tests. Pour lancer l'ensemble et obtenir le score total :

```sh
python full_bench.py rust_release   # build en mode release puis lance tous les tests
python full_bench.py rust           # build en mode debug puis lance tous les tests
```

Le script compile le binaire, exécute chaque fichier de `tests/` en entrée standard, lit le score final (`All runs best Score: N` sur `stderr`) et affiche un récapitulatif par test ainsi que le score total cumulé.

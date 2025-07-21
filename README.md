# Rust - Cours Basique

Apprentissage d'un nouveau langage 
Le fait que le langage soit optimisé est sympa (étude faite en 2021 qui le montre dans le top 3 des languages en termes de performance sur plusieurs échelles)
https://haslab.github.io/SAFER/scp21.pdf
Mais ce n'est pas un langage adapté au Master IA et Big DATA, et il rejoindra donc la liste des langages que j'ai vu, mais que je n'utiliserais pas dans le future
(JAVA, Javascript, PHP, TypeScript, Scala, et maintenant Rust)

## Types de données

### Types entiers
- `i32` : entier signé sur 32 bits (valeurs de -2,147,483,648 à 2,147,483,647)
- `u32` : entier non signé sur 32 bits (valeurs de 0 à 4,294,967,295)
- `i64` : entier signé sur 64 bits (très grand intervalle)
- `u8` : entier non signé sur 8 bits (0 à 255)

### Types décimaux
- `f32` : nombre à virgule flottante sur 32 bits
- `f64` : nombre à virgule flottante sur 64 bits (plus précis)

### Types texte
- `&str` : référence vers une chaîne de caractères (immutable)
- `String` : chaîne de caractères dynamique (mutable, peut grandir/rétrécir)

## Variables et mutabilité

```rust
let nom = "Kevin";              // variable immutable
let mut age = 30;               // variable mutable (modifiable)
let temperature: f32 = 32.5;    // avec type explicite
age = 31;                       // modification possible car mut
```

**Convention Rust** : utiliser le `snake_case` (jamais de majuscules, underscore pour séparer les mots).

## Fonctions

### Syntaxe de base
```rust
fn addition(n1: i32, n2: i32) -> i32 {   // -> i32 indique le type de retour
    n1 + n2                              // return implicite (pas de point-virgule)
}

fn say_hello(nom: &str) {                // fonction sans retour
    println!("Bonjour, {}", nom);
}

// Utilisation
let resultat = addition(12, 3);
say_hello("Alice");
```

### Points importants
- `fn` définit une fonction
- `&str` est une référence vers une chaîne
- Pas de `return` explicite nécessaire sur la dernière ligne
- Le point-virgule transforme une expression en instruction

## Conditions

```rust
let nombre = 16;
if nombre % 2 == 0 {
    println!("Pair");
} else if nombre > 10 {
    println!("Grand nombre impair");
} else {
    println!("Petit nombre impair");
}
```

## Boucles et itération

### Boucle for avec range
```rust
for i in 1..=10 {              // 1 à 10 inclus
    println!("i vaut {}", i);
}

for i in 1..5 {                // 1 à 4 (5 exclu)
    println!("i vaut {}", i);
}
```

### Itération sur collections
```rust
let voitures = ["jeep", "renault", "bmw"];
for voiture in voitures {
    println!("Voiture : {}", voiture);
}
```

### Enumerate (index + valeur)
```rust
let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

for (i, option) in options.iter().enumerate() {
    println!("{}. {}", i + 1, option);  // Affiche : 1. Afficher solde, etc.
}
```

**Explication** :
- `iter()` : crée un itérateur sur la collection sans la consommer
- `enumerate()` : transforme l'itérateur en séquence (index, valeur)

## Collections

### Tableaux (arrays)
```rust
let voitures = ["jeep", "renault", "bmw"];  // taille fixe, type homogène
let nombres: [i32; 3] = [1, 2, 3];          // déclaration avec type et taille
```

### Vecteurs (vectors)
```rust
let mut noms = vec![String::from("Kevin"), String::from("Alice")];
noms.push(String::from("Bob"));             // ajouter un élément
println!("Taille : {}", noms.len());        // obtenir la taille
```

## Gestion des entrées utilisateur

### Lecture simple
```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input).expect("Erreur de lecture");
println!("Vous avez saisi : {}", input.trim());
```

### Conversion et validation
```rust
let mut choix = String::new();
io::stdin().read_line(&mut choix).expect("Erreur de lecture");

let choix: usize = match choix.trim().parse() {
    Ok(num) => num,                      // conversion réussie
    Err(_) => {                          // erreur de conversion
        println!("Veuillez saisir un numéro valide");
        return;                          // sortir de la fonction
    }
};
```

**Explication** :
- `trim()` : enlève les espaces et retours à la ligne
- `parse()` : tente de convertir la chaîne en nombre
- `match` : gestion des cas de succès (`Ok`) et d'erreur (`Err`)
- `usize` : type d'entier pour les indices de collections

## Pattern Matching avec match

```rust
match choix {
    1 => println!("Option 1 choisie"),
    2 => println!("Option 2 choisie"),
    3 => {
        println!("Option 3 choisie");
        // plusieurs instructions possibles
    }
    _ => println!("Option invalide"),    // cas par défaut
}
```

## Structures (struct)

### Définition et utilisation
```rust
struct CompteBancaire {
    nom: String,
    solde: f32,
}

impl CompteBancaire {
    fn new(nom: String, solde_initial: f32) -> CompteBancaire {
        CompteBancaire {
            nom,                         // raccourci pour nom: nom
            solde: solde_initial,
        }
    }

    fn afficher_solde(&self) {           // &self = référence à l'instance
        println!("Solde : {:.2}€", self.solde);
    }

    fn retrait(&mut self, montant: f32) { // &mut self = référence mutable
        self.solde -= montant;
    }
}
```

## Gestion de projet avec Cargo

### Commandes essentielles
```bash
cargo new mon_projet       # Créer un nouveau projet
cd mon_projet
cargo run                  # Compiler et exécuter
cargo build               # Compiler seulement
cargo check               # Vérifier la syntaxe rapidement
```

### Structure d'un projet
```
mon_projet/
├── Cargo.toml            # Configuration du projet
└── src/
    └── main.rs           # Code principal
```

## Outils de développement

### Extension VSCode recommandée
`rust-lang.rust-analyzer` : Fournit l'autocomplétion, la détection d'erreurs, et l'aide au développement.

### Debugging et affichage
```rust
println!("Valeur : {}", variable);          // affichage simple
println!("Nom : {}, âge : {}", nom, age);   // plusieurs variables
println!("Prix : {:.2}€", prix);            // 2 décimales
```

## Bonnes pratiques

1. **Nommage** : Utiliser le `snake_case` pour les variables et fonctions
2. **Mutabilité** : Préférer `let` à `let mut` quand possible
3. **Gestion d'erreurs** : Utiliser `match` ou `expect()` pour les opérations pouvant échouer
4. **Documentation** : Commenter le code de manière concise
5. **Types** : Laisser Rust inférer les types quand c'est évident

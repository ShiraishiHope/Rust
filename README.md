# Rust - Cours Basique

Apprentissage d'un nouveau langage avec focus sur la programmation réseau.

---

# TPs Réseau

## TP 7 - DNS Client/Server
Simple DNS server and client implementation using UDP.

```bash
cd tp7_dns
cargo build

# Terminal 1 - Server (requires sudo for port 53)
sudo cargo run --bin dns_server

# Terminal 2 - Client  
cargo run --bin dns_client
# Test with: example.com, google.com, localhost
```
<img width="1482" height="775" alt="image" src="https://github.com/user-attachments/assets/781f9adb-8aba-483e-8848-c2a70042e7e7" />

## TP 8 - Custom Protocol
JSON-based messaging protocol over TCP with authentication.

```bash
cd tp8_protocol
cargo build

# Terminal 1 - Server
cargo run --bin protocol_server

# Terminal 2+ - Clients
cargo run --bin protocol_client
# Commands: message <text>, list, logout
```
<img width="1625" height="904" alt="image" src="https://github.com/user-attachments/assets/8b3c465c-6276-457c-bb72-d379abf99b67" />

## TP 9 - WebSocket Chat
Real-time chat server using WebSocket protocol.

```bash
cd tp9_websocket  
cargo build

# Terminal 1 - Server
cargo run --bin websocket_server

# Terminal 2+ - Clients
cargo run --bin websocket_client
# Type messages, use /quit to exit
```

<img width="1647" height="933" alt="image" src="https://github.com/user-attachments/assets/051a5534-7dfa-4d48-aec7-c1bff4d5606d" />

---

<details>
<summary><b>💭 Avis Personnel sur Rust</b></summary>

Le fait que le langage soit optimisé est sympa (étude faite en 2021 qui le montre dans le top 3 des languages en termes de performance sur plusieurs échelles)
- https://haslab.github.io/SAFER/scp21.pdf
- Mais ce n'est pas un langage adapté au Master IA et Big DATA, à mon job de data analiste/data scientist, et à mes choix de direction future, ML Engineer.
- Il rejoindra donc la liste des langages que j'ai vu, mais que je n'utiliserais pas dans le future
- (JAVA, Javascript, PHP, TypeScript, Scala, et maintenant Rust)

</details>

<details>
<summary><b>📊 Types de Données</b></summary>

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

</details>

<details>
<summary><b>🔧 Variables et Mutabilité</b></summary>

```rust
let nom = "Kevin";              // variable immutable
let mut age = 30;               // variable mutable (modifiable)
let temperature: f32 = 32.5;    // avec type explicite
age = 31;                       // modification possible car mut
```

**Convention Rust** : utiliser le `snake_case` (jamais de majuscules, underscore pour séparer les mots).

</details>

<details>
<summary><b>⚙️ Fonctions</b></summary>

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

</details>

<details>
<summary><b>🔀 Conditions</b></summary>

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

</details>

<details>
<summary><b>🔄 Boucles et Itération</b></summary>

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

</details>

<details>
<summary><b>📦 Collections</b></summary>

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

</details>

<details>
<summary><b>⌨️ Gestion des Entrées Utilisateur</b></summary>

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

</details>

<details>
<summary><b>🔍 Pattern Matching avec match</b></summary>

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

</details>

<details>
<summary><b>🏗️ Structures (struct)</b></summary>

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

</details>

<details>
<summary><b>📦 Gestion de Projet avec Cargo</b></summary>

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

</details>

<details>
<summary><b>🛠️ Outils de Développement</b></summary>

### Extension VSCode recommandée
`rust-lang.rust-analyzer` : Fournit l'autocomplétion, la détection d'erreurs, et l'aide au développement.

### Debugging et affichage
```rust
println!("Valeur : {}", variable);          // affichage simple
println!("Nom : {}, âge : {}", nom, age);   // plusieurs variables
println!("Prix : {:.2}€", prix);            // 2 décimales
```

</details>

<details>
<summary><b>✅ Bonnes Pratiques</b></summary>

1. **Nommage** : Utiliser le `snake_case` pour les variables et fonctions
2. **Mutabilité** : Préférer `let` à `let mut` quand possible
3. **Gestion d'erreurs** : Utiliser `match` ou `expect()` pour les opérations pouvant échouer
4. **Documentation** : Commenter le code de manière concise
5. **Types** : Laisser Rust inférer les types quand c'est évident

</details>

use std::fs;
use std::io;

struct FileManager {
    nom: String,
}

impl FileManager {
    fn lire(&self, fichier: &str) {
        match fs::read_to_string(fichier) {
            Ok(contenu) => println!("{}", contenu),
            Err(_) => println!("Erreur lecture"),
        }
    }

    fn ecrire(&self, fichier: &str, contenu: &str) {
        match fs::write(fichier, contenu) {
            Ok(_) => println!("Écrit"),
            Err(_) => println!("Erreur écriture"),
        }
    }

    fn modifier(&self, fichier: &str, nouveau: &str) {
        match fs::write(fichier, nouveau) {
            Ok(_) => println!("Modifié"),
            Err(_) => println!("Erreur modification"),
        }
    }

    fn supprimer(&self, fichier: &str) {
        match fs::remove_file(fichier) {
            Ok(_) => println!("Supprimé"),
            Err(_) => println!("Erreur suppression"),
        }
    }

    fn afficher(&self) {
        println!("Gestionnaire: {}", self.nom);
    }
}

fn main() {
    let manager = FileManager {
        nom: String::from("Admin"),
    };

    let options = ["Lire", "Écrire", "Modifier", "Supprimer", "Quitter"];
    
    loop {
        manager.afficher();
        
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur");

        let choix: usize = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choix {
            1 => {
                let mut fichier = String::new();
                println!("Fichier:");
                io::stdin().read_line(&mut fichier).expect("Erreur");
                manager.lire(fichier.trim());
            }
            2 => {
                let mut fichier = String::new();
                let mut contenu = String::new();
                println!("Fichier:");
                io::stdin().read_line(&mut fichier).expect("Erreur");
                println!("Contenu:");
                io::stdin().read_line(&mut contenu).expect("Erreur");
                manager.ecrire(fichier.trim(), contenu.trim());
            }
            3 => {
                let mut fichier = String::new();
                let mut nouveau = String::new();
                println!("Fichier:");
                io::stdin().read_line(&mut fichier).expect("Erreur");
                println!("Nouveau contenu:");
                io::stdin().read_line(&mut nouveau).expect("Erreur");
                manager.modifier(fichier.trim(), nouveau.trim());
            }
            4 => {
                let mut fichier = String::new();
                println!("Fichier:");
                io::stdin().read_line(&mut fichier).expect("Erreur");
                manager.supprimer(fichier.trim());
            }
            5 => break,
            _ => continue,
        }

        let mut compteur = 0;
        while compteur < 1 {
            println!("Opération terminée");
            compteur += 1;
        }
    }
}
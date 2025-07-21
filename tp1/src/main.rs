//Import library
use std::io;

//Create new personalized data type. 
//CompteBancaire includes: nom (String), solde (float32)
struct CompteBancaire {
    nom: String,
    solde: f32,
}

//Define Struc's methods
impl CompteBancaire {
    //fonction constructor. Create new account.
    //param: nom, solde_initial
    //return: CompteBancaire
    fn new(nom: String, solde_initial: f32) -> CompteBancaire {
        CompteBancaire {
            nom,
            solde: solde_initial,
        }
    }

    //Get amount
    //param: &self (instance of the value (like this in Java))
    fn afficher_solde(&self) {
        println!("Solde de {} : {:.2}€", self.nom, self.solde);
    }

    //function to get money
    //param: &mut (mutable ref to instance, allow modification of the value in the account), montant
    //return bool (return success of the operation)
    fn retrait(&mut self, montant: f32) -> bool {
        if montant <= self.solde {
            self.solde -= montant;
            println!("Retrait de {:.2}€ effectué", montant);
            true
        } else {
            println!("Solde insuffisant");
            false
        }
    }
}

//main process
fn main() {
    //mut variable (accounts)
    //vec![] macro to créate a dynamicvector that can grow or shrink
    let mut comptes = vec![
        // call to contructor function (static)
        CompteBancaire::new(String::from("Kevin"), 1000.0),
        CompteBancaire::new(String::from("Alice"), 500.0),
        CompteBancaire::new(String::from("Bob"), 750.0),
    ];

    //options to choose from
    let options = ["Afficher solde", "Retrait", "Liste comptes", "Quitter"];

    //main loop
    loop {
        println!("\n=== BANQUE MENU ===");
        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        //user choices
        println!("Choisissez une option:");
        let mut choix = String::new();
        //stop the program with error message
        io::stdin().read_line(&mut choix).expect("Erreur lecture");

        //remove crlf, convert into number, deal with success/error
        //usize: integer used for table index
        let choix: usize = match choix.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Numéro invalide");
                //return to start of loop
                continue;
            }
        };

        //pattern matching
        match choix {
            1 => afficher_solde(&comptes),
            2 => effectuer_retrait(&mut comptes),
            3 => lister_comptes(&comptes),
            4 => {
                println!("Au revoir!");
                break;
            }
            _ => println!("Option invalide"),
        }
    }
}

// function to read amount
fn afficher_solde(comptes: &[CompteBancaire]) {
    println!("Quel compte? (0-{}):", comptes.len() - 1);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lecture");
    
    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if index < comptes.len() {
        comptes[index].afficher_solde();
    } else {
        println!("Compte inexistant");
    }
}

//function to remove amount
fn effectuer_retrait(comptes: &mut [CompteBancaire]) {
    println!("Quel compte? (0-{}):", comptes.len() - 1);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur lecture");
    
    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if index >= comptes.len() {
        println!("Compte inexistant");
        return;
    }

    println!("Montant à retirer:");
    let mut montant_str = String::new();
    io::stdin().read_line(&mut montant_str).expect("Erreur lecture");

    let montant: f32 = match montant_str.trim().parse() {
        Ok(m) => m,
        Err(_) => {
            println!("Montant invalide");
            return;
        }
    };

    comptes[index].retrait(montant);
}

//function to list accounts
fn lister_comptes(comptes: &[CompteBancaire]) {
    println!("\n=== LISTE DES COMPTES ===");
    for (i, compte) in comptes.iter().enumerate() {
        println!("{}: {} - {:.2}€", i, compte.nom, compte.solde);
    }
}
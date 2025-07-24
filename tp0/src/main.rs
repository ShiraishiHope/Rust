use std::io;
use chrono::Utc;

fn main() {

    let name = "Ryan";
    let age: u32= 30;
    //let dad_age:u32= 70;
    let temperature: f32= 14.6;
    //i32 32 signé -2XXX à 2XXXXXXXX
    //u32 32 non signé 0 à 4XXXXXXXX
    // i64 64  signé grands intervalles
    //u8 non signé 0 - 225

    println!("Hello, world!");
    println!("I'm {} and I'm {} years old",name, age);
    println!("it's {} today", temperature);

    //2. les fonctions : 
        // fn défini une fonction
        // &str est de type chaine de caractère (référence)
        // On créé une fonction addition() qui retourne une somme et on l'appelle depuis le main

    

    let age_combined = addition(7, 8);
    println!("Somme : {}",age_combined);

    let hi= say_hello("John");
    println!("{}",hi);


    //les conditions les boucles
    let number = 16;
        if number%2 ==0{
            println!("Pair");
        } else {
            println!("Impair");
        }

    //boucle
    for i in 1..=10{
        println!(" i = {}",i);
    }

    // A noter que  1..5
    //  ..  intervalle exculsif ( fin exclue ) : 1,2,3,4
    // ..=  intervalle inclusif ( fin incluse ) : 1,2,3,4,5

    // Exemple de tableau : itérer sur un tableau 

    let  voitures = ["jeep", "renault", "bmw"];
    for voiture in voitures {
       println!("Voiture : {}", voiture);
    }

    //    for ( index, valeur) in  collection.iter().enumerate(){
    //  on peut utiliser index et valeur ici }

    // je reprends l'exemple de voiture 
    for (i,voiture) in voitures.iter().enumerate(){
       println!("Index {} : {}", i, voiture);
    }
    // iter(): crée un itérateur sur la collection sans le consommer
    // enumerate: transforme l'itérateur en une séquence de index,valeur 

    // Exemple de vecteur 

    let noms = vec![String::from("Kevin"), String::from("Nourdine")];
    for (i,nom) in noms.iter().enumerate(){
       println!("Nom {} :{}", i, nom);
    }

    // Usage de enumerate dans un cas réel : Afficher un Menu avec numéro et choix

    let options = ["Afficher solde","Retrait","Liste comptes","Quitter"];

    println!("Menu:");
    for ( i,option) in options.iter().enumerate(){
       // afficher chaque option et on commence par 1 
       println!("{}. {}", i+1, option); 
    }

    println!("Veuillez saisir un numéro de votre choix:");

    let mut choix = String::new();
    io::stdin().read_line(&mut choix).expect("Attention erreur de lecture");
    
    let choix:usize = match choix.trim().parse(){
       Ok(num) => num,
       Err(_)=> {
           println!("Veuillez saisir un numero valide");
           return;
       }
    };

    if choix == 0 || choix > options.len(){
       println!(" choix hors système !! limite système ");
    } else {
       println!("Vous avez sélectionné : {}", options[choix-1]);
       // ici on peut exécuter une action selon choix dans options 
    }

    //tableaux
    let tab:[i32;4] = [1,2,3,4];
    // pour éviter le warning, _ devant le nom de la variable
    let _tab2:[i32;4] = [1,2,3,4];

    for i in 0..tab.len(){
       println!("le tableau tab {}", tab[i]);

    }
    for &elt in &tab{
       println!("l'element est {}", elt);
    }


    println!("**************loop****************");
    let mut compteur = 0;
    loop {
        println!(" Compteur: {}",compteur);
        compteur+=1;
    if compteur == 3{
        break; // on sort de la boucle quand compteur atteint 3
    }
    }

    println!("*******while*************");
    let mut compteur2 = 0;
    while compteur2<4{
        println!("Compteur 2 = {}", compteur2);
        compteur2 +=1;
    }

    struct Salarie{
        nom: String,
        ville: String,
        age: u32
    }

    let kevin = Salarie{
    nom: String::from("Kevin"),
    ville: String::from("Lyon"), 
    age: 666                      
    };

    println!("Name: {} Town: {} Age: {}", kevin.nom, kevin.ville, kevin.age);


    //match
    let number = 5;

    match number {
    1 => println!("Un"),
    2 => println!("Deux"),
    3 => println!("Trois"),
    4 => println!("Quatre"),
    5 => println!("Cinq"),
    6 => println!("Six"),
    7 => println!("Sept"),
    8 => println!("Huit"),
    9 => println!("Neuf"),
    10 => println!("Dix"),
    _ => println!("Nombre non reconnu")
}

impl Salarie{
    fn afficher(&self){
        println!("La personne suivante: {} est convoquée ", self.nom);
    }
}

let salarie = Salarie{
    nom:"Alexandre".to_string(),
    ville:"Hell".to_string(),
    age:666,
};

salarie.afficher();


   // Exemple  compteur struct

   struct Compteur {
     value :u32
   }

//   A noter : 

   // &self -> lecture seulement
  // &mut self -> modification possible
  //  self -> transfert complet ( consommation )


   impl Compteur {
       fn afficher(&self){
        println!("la valeur actuelle :{}",self.value);
       }

       fn incrementer (&mut self){
        self.value +=1;
       }

         fn deplacer (self){
          println!("Dépalacé : {}",self.value);  // self deplacé ici, plus accessible 
       }

   }


      let mut compteur = Compteur {value:0};
       compteur.afficher();
       compteur.incrementer();
       compteur.deplacer();


       // Usage  de chrono 
       let maintenant = Utc::now();
       println!(" la date et l heure actuelle UTC est {}",maintenant);
       println!("Format FR : {}", maintenant.format("%d/%m/%Y")); // Format 24/07/2025
       println!("Format FR : {}", maintenant.format("%d/%m/%Y %H:%M:%S")); // Format 24/07/2025 10:18:22


               //1. Ownership
           // chaque valeur a un propriétaire unique, responsable de libérer la mémoire
           // lorsqu'elle sort du scop 
           // quand le propriétaire est déplacé, l'ancien propriétaire ,ne peut plus y accéder
           // quand le propriétaire sort du scop, la valeur est automatiquement libérée 

         //exemple : 

         let prenom = String::from("Nourddine"); // prenom est propriétaire de la String
         // println!("{}",prenom); 
         let secu = String::from("1897272824252");
         let prenom2 = prenom.clone();
         greetings(prenom); // propriétaire est transferé à la fonction greetings()
          println!("{}",prenom2); 

          greetings2(&secu);  // emprunt immuable 
          println!("{}",secu); 

       // 3 MemberShip : ( Appartenance à une structure )
          // décrit quelles sont les données contenues dans une structure Struct

          // exemple :

          let user = User {
            nom : String::from("Alexandre"),
            secu: String::from("1825678290 55")
          };
        
         println!("nom {}",user.nom);
         //
         // display(&user); // &emprumter un champ d'une structure
         display(user); 
}
// en C/C++     int age = 25; 
//           Contact contact1; 
//     usage contact1.age  contact.prenom 
//  public  class Voiture ( int puissance , String couleur , Vec  marque) {}

    // fonction display
    fn display(user: User) -> User{
      println!(" Nom: {}, num secu : {}", user.nom, user.secu);
      user
    }


  // avec emprunt & 
      fn greetings2(msg:&String){
      println!("Hello Mister {}",msg);
     }   

  // sans emprunt
     fn greetings(msg:String){
      println!("Hello Mister {}",msg);
     
}


fn addition(n1:i32, n2:i32) -> i32{
        return n1+n2;
    }

fn say_hello(nom:&str) -> String{
    format!("Bonjour {}",nom)
}
use std::io;

mod menu;
mod personne;
mod reader_service;
mod write;

fn main() {
    let _ = menu::menu();

    println!("Bienvenue Romain ! ");

    #[warn(while_true)]
    while true {
        println!("1: Afficher la liste des contacts");
        println!("2: Ajouter un contact");
        println!("3: Supprimer un contact");

        // lire l'entrer utlisateur
        let mut result = String::new();

        io::stdin()
            .read_line(&mut result)
            .ok()
            .expect("Couldn't read line");
        // match sur la reponse et di si 1 alors fait ca si 2 alors fait ca etc....

        // Erreur tout le temps a resoudre !
        match result.as_str() {
            "1" => {
                println!("Vous avez choisi 1 !");
            }
            "2" => {
                println!("Vous avez choisi 2 !");
            }
            "3" => {
                println!("Vous avez choisi 3 !");
            }
            _ => {
                println!("Erreur !")
            }
        }
    }

    let csv = vec!["Personne.csv"];
    let csv_read = reader_service::reader_multi(csv);
    let file_name: &str = "csv.json";
    let create_folder = write::create(file_name);
    if create_folder.is_ok() {
        println!("File Create")
    }

    let write_folder = write::write(csv_read);
    if write_folder.is_ok() {
        println!("Json Ajouter")
    }

    let _ = write::read(file_name);

    // on est samedi woula je peux pas coder il y a l'annive
    // surment ce soir mais pas sur re rentrer aveant 00h
}

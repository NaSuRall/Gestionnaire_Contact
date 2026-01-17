mod personne;
mod reader_service;
mod write;

fn main() {
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

mod personne;
mod reader_service;
mod write;

fn main() {
    let csv = vec!["Personne.csv"];
    let csv_read = reader_service::reader_multi(csv);

    let create_folder = write::create("csv.json");
    if create_folder.is_ok() {
        println!("File Create")
    }

    let write_folder = write::write(csv_read);
    if write_folder.is_ok() {
        println!("Json Ajouter")
    }
}

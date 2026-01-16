mod personne;
mod reader_service;

fn main() {
    let csv = vec!["Personne.csv"];
    let csv_read = reader_service::reader_multi(csv);
    println!("{:#?}", csv_read);
}

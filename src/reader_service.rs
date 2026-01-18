use crate::personne::Personne;
use csv::ReaderBuilder;

fn reader(file_name: &str) -> Vec<Personne> {
    let mut builder = ReaderBuilder::new();
    builder.double_quote(false).delimiter(b',');

    let mut my_reader = builder.from_path(file_name).expect("File not found");

    let mut tab: Vec<Personne> = Vec::new();

    for record in my_reader.deserialize() {
        let var: Personne = record.unwrap();
        tab.push(var);
    }
    return tab;
}

pub fn reader_multi(files: Vec<&str>) -> String {
    let mut result: Vec<Personne> = Vec::new();

    for file in files {
        let mut content = reader(file);
        result.append(&mut content);
    }

    serde_json::to_string(&result).unwrap()
}

// Add function to read jsp on est dimanche frr

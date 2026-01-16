use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Personne {
    pub nom: String,
    pub prenom: String,
    pub email: String,
    pub tel: String,
    pub clases: String,
}

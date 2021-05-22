//! Club description / Metadata


#[derive(Debug, Serialize, Deserialize)]
pub struct Club {
    pub name: String,
    pub purpose: String,
    pub address: Vec<String>,
    pub phone: String,
}


impl Club {
    pub fn open() -> Club {
        Club {
            name: String::from("Club Name"),
            purpose: String::from("Clubs reason for being"),
            address: vec!(
                String::from("123 Elm St."),
                String::from("Anywhere, ST 99999-9999"),
            ),
            phone: String::from("(999)-999-9999"),
        }         
    }
}

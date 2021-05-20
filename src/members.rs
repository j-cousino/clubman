c/// membership database 
use std::string::String;
use std::error::Error;

use uuid::Uuid;

use argon2::{password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};
use rand_core::OsRng;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberID (Uuid);

impl MemberID {
    pub fn new() -> MemberID {
        MemberID ( Uuid::new_v4() )
    }
}

impl ToString for MemberID {
    fn to_string( &self ) -> String {
        self.0.to_simple().to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    pub id: MemberID,
    pub email: String,
    pub pass_hash: String,
}

impl Member {
    pub fn new( email: &String, password: &String ) -> Member {
        Member {
            id: MemberID::new(),
            email: email.clone(),
            pass_hash: hash_password( password ).unwrap(),
        }
    }
}

fn hash_password(password: &String) -> Option<String> {
    match Argon2::default()
        .hash_password_simple(
            password.as_bytes(), 
            SaltString::generate(&mut OsRng).as_ref()
        ) {
            Ok(n) => Some(n.to_string()),
            Err(..) => None
        }
}

pub fn authenticate_member(email: &String, password: &String) -> bool {
    false
}

#[cfg(test)]
mod test{
    
    use std::string::String;
    use argon2::{Argon2, password_hash::{PasswordVerifier, PasswordHash}};
    use super::{MemberID, Member};

    #[test]
    fn test_hash_password() {
        let pwd = "password".to_string();
       let password_hash_result = super::hash_password(&pwd);
       assert!( password_hash_result.is_some());
       let password_hash = password_hash_result.unwrap();
       let parsed_hash = PasswordHash::new(&password_hash).unwrap();
       assert!(Argon2::default().verify_password(&pwd.as_bytes(), &parsed_hash).is_ok());
    }

    #[test]
    fn test_memberid_new() {
        let member_id = MemberID::new().to_string();
        println!("Member ID: {}", member_id);
        assert_eq!(member_id.len(), 32);
    }

}
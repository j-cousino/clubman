use super::basic::{Address, PhoneNumber};

#[derive(Debug, Default)]
pub struct Club {
    name: String,
    purpose: String,
    address: Address,
    phone: PhoneNumber,
}

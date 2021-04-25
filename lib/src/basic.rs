//! Basic types

#[derive(Debug, Default)]
/// Represents an en-US proper name
///
/// A ptoper name begins with
pub struct ProperName {
    pub value: String,
}


#[derive(Debug, Default)]
/// Represent a en-US Address
///
pub struct Address {
    street: String,
    city: String,
    state: String,
    zip: String,
}

#[derive(Debug, Default)]
/// an en-US Phone number
///
pub struct PhoneNumber {
    value: String,
}

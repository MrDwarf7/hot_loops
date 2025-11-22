#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    zip_code: String,
}

impl Clone for Address {
    fn clone(&self) -> Self {
        Address {
            street: self.street.clone(),
            city: self.city.clone(),
            zip_code: self.zip_code.clone(),
        }
    }
}

impl TryFrom<String> for Address {
    type Error = String;

    fn try_from(address: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = address.split(',').collect();
        if parts.len() != 3 {
            return Err("Address must be in the format 'street, city, zip_code'".to_string());
        }
        Ok(Address {
            street: parts[0].trim().to_string(),
            city: parts[1].trim().to_string(),
            zip_code: parts[2].trim().to_string(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub height: f32,
    address: Address,
}

impl Person {
    #[rustfmt::skip]
    pub fn new(name: String, age: u8, height: f32, address: String) -> Self {
        let address = Address::try_from(address).expect("Invalid address format");
        Person { name, age, height, address }
    }
}

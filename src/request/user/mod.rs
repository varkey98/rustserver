use serde::{ser::{Serialize, SerializeStruct}, Deserialize};

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    pub telephone: i32,
    pub curr_city: String,
    pub weather: String,
    pub nested: Option<Vec<User>>
}

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut s = serializer.serialize_struct("User", 5)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("telephone", &self.telephone)?;
        s.serialize_field("curr_city", &self.curr_city)?;
        s.serialize_field("weather", &self.weather)?;
        s.serialize_field("nested", &self.nested)?;
        s.end()
    }
}
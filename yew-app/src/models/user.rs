use serde::Deserialize;

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub first: String,
    pub last: String,
    pub title: String,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Age {
    pub age: u8,
}
#[derive(Clone, Deserialize, PartialEq)]
pub struct Location {
    pub country: String,
    pub city: String,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Picture {
    pub large: String,
    pub medium: String,
    pub thumbnail: String,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub name: Name,
    pub email: String,
    pub gender: String,
    pub phone: String,
    pub picture: Picture,
    pub location: Location,
    pub dob: Age,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct Users {
    pub results: Vec<User>
}

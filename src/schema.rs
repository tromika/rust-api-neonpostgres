use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePeopleSchema {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePeopleSchema {
    pub name: Option<String>,
}

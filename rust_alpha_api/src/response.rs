use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub collection: Vec<T>,
}

impl<T> Response<T> {
    pub fn new() -> Self {
        Self { collection: vec![] }
    }
}

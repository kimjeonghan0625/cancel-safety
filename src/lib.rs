use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Foo {
    bar: String,
}

impl Foo {
    pub fn new(data: &str) -> Self {
        Foo {
            bar: data.to_string(),
        }
    }
}

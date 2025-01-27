use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub(crate) struct BinResponse {
    args: HashMap<String, String>,
    headers: HashMap<String, String>,
    url: String,
}

impl BinResponse {
    pub(crate) fn new(
        args: HashMap<String, String>,
        headers: HashMap<String, String>,
        url: String,
    ) -> Self {
        Self { args, headers, url }
    }
}

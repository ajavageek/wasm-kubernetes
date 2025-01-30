use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub(crate) struct BinResponse {
    flavor: &'static str,
    args: HashMap<String, String>,
    headers: HashMap<String, String>,
    url: String,
}

impl BinResponse {
    pub(crate) fn new(
        flavor: &'static str,
        args: HashMap<String, String>,
        headers: HashMap<String, String>,
        url: String,
    ) -> Self {
        Self { flavor, args, headers, url }
    }
}

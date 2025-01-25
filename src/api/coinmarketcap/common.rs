use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Status {
    pub timestamp: String,
    pub error_code: u32,
    pub error_message: Option<String>,
    pub elapsed: u32,
    pub credit_count: u32,
    pub notice: Option<String>,
}

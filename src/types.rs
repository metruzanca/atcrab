use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Record<T> {
    pub uri: String,
    pub cid: Option<String>,
    pub value: T,
}

#[derive(Deserialize, Debug)]
pub struct ListRecords<T> {
    pub records: Vec<Record<T>>,
    pub cursor: Option<String>,
}

// --- Write types ---

#[derive(Deserialize, Debug)]
pub struct CreateRecordOutput {
    pub uri: String,
    pub cid: String,
}

#[derive(Deserialize, Debug)]
pub struct PutRecordOutput {
    pub uri: String,
    pub cid: String,
}

#[derive(Serialize)]
pub struct DeleteRecordInput {
    pub repo: String,
    pub collection: String,
    pub rkey: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap_record: Option<String>,
}

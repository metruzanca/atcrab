use serde::Deserialize;

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

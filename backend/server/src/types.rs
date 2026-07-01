use serde::Serialize;
#[derive(ts_rs::TS, Serialize)]
#[ts(export)]
#[allow(dead_code, reason = "just for testing purposes")]
pub struct MyDummyStruct {
    pub id: u32,
    pub label: String,
    pub values: Vec<usize>,
    pub enumeration: MyDummyEnum,
}

#[derive(ts_rs::TS, Serialize)]
#[ts(export)]
pub enum MyDummyEnum {
    One,
    Two(String),
    Three(Result<u32, String>),
    Four(Option<bool>),
}

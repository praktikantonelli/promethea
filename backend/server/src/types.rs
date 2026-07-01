use serde::Serialize;
#[derive(ts_rs::TS, Serialize)]
#[ts(export)]
#[allow(dead_code, reason = "just for testing purposes")]
pub struct MyDummyStruct {
    id: u32,
    label: String,
    values: Vec<usize>,
    enumeration: MyDummyEnum,
}

#[derive(ts_rs::TS, Serialize)]
#[ts(export)]
pub enum MyDummyEnum {
    One,
    Two(String),
    Three(Result<u32, String>),
    Four(Option<bool>),
}

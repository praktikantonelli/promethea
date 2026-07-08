use serde::Serialize;

/// sample struct that is used in type returned from axum handler
#[derive(ts_rs::TS, Serialize, Debug)]
#[ts(export)]
#[non_exhaustive]
pub struct MyDummyStruct {
    /// sample u32 value
    pub id: u32,
    /// sample string value
    pub label: String,
    /// sample collection value
    pub values: Vec<usize>,
    /// sample use of other defined type
    pub enumeration: MyDummyEnum,
}

/// sample enum that is used in type returned from axum handler
#[derive(ts_rs::TS, Serialize, Debug)]
#[ts(export)]
#[non_exhaustive]
pub enum MyDummyEnum {
    /// sample regular enum variant
    One,
    /// sample enum variant containing atomic value
    Two(String),
    /// sample enum variant containing another enum
    Three(Result<u32, String>),
    /// sample enum variant containing yet another enum
    Four(Option<bool>),
}

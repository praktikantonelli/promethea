#[derive(ts_rs::TS)]
#[ts(export)]
pub struct MyDummyStruct {
    id: u32,
    label: String,
    values: Vec<usize>,
    enumeration: MyDummyEnum
}

#[derive(ts_rs::TS)]
#[ts(export)]
pub enum MyDummyEnum {
    One,
    Two(String),
    Three(Result<u32, String>),
    Four(Option<bool>)
}

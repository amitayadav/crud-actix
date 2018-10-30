use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::query::*;
use cdrs::query::QueryExecutor;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::types::prelude::Row;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
pub struct Student {
    pub roll_no: i32,
    pub name: String,
    pub marks: i32,
}
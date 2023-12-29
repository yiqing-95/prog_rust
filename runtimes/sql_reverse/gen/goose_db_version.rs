use serde_derive;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[crud_table]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GooseDbVersion {
	pub id: u64,
	pub version_id: i64,
	pub is_applied: i8,
	pub tstamp: Option<NaiveDateTime>,
}
// ***************************************以下是自定义代码区域******************************************
/*
example: [
    {"skip_fields": ["updated_at", "created_at"], "filename": "table_name1"},
    {"contain_fields": ["updated_at", "created_at"], "filename": "table_name2"}
]
*/
// *************************************************************************************************
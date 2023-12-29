use serde_derive;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[crud_table]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct B {
	pub parentId: Option<i64>,
	pub childId: Option<i64>,
}
// ***************************************以下是自定义代码区域******************************************
/*
example: [
    {"skip_fields": ["updated_at", "created_at"], "filename": "table_name1"},
    {"contain_fields": ["updated_at", "created_at"], "filename": "table_name2"}
]
*/
// *************************************************************************************************
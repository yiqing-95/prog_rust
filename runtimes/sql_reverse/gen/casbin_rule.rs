use serde_derive;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[crud_table]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CasbinRule {
	pub p_type: Option<String>,
	pub v0: Option<String>,
	pub v1: Option<String>,
	pub v2: Option<String>,
	pub v3: Option<String>,
	pub v4: Option<String>,
	pub v5: Option<String>,
}
// ***************************************以下是自定义代码区域******************************************
/*
example: [
    {"skip_fields": ["updated_at", "created_at"], "filename": "table_name1"},
    {"contain_fields": ["updated_at", "created_at"], "filename": "table_name2"}
]
*/
// *************************************************************************************************
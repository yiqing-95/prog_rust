//#[macro_use] define in 'root crate' or 'mod.rs' or 'main.rs'
#[macro_use]
extern crate rbatis;


use rbatis::rbdc::datetime::DateTime;
use serde_json::json;
use rbdc_sqlite::driver::SqliteDriver;
use rbatis::RBatis;

/// table
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<DateTime>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

impl_insert!(BizActivity{});

#[tokio::main]
async fn main() {
    /// enable log crate to show sql logs
    fast_log::init(fast_log::Config::new().console()).expect("rbatis init fail");
    /// initialize rbatis. also you can call rb.clone(). this is  an Arc point
    let rb = RBatis::new();
    /// connect to database 

    //init() just set driver
    //rb.init(rbdc_sqlite::driver::SqliteDriver {}, "sqlite://target/sqlite.db" ).unwrap();

    // use std::fs::File;
    // let f = File::create("../target/test.db");
//         if f.is_err() {
//             println!("{}", f.err().unwrap());
//         } else {
//             drop(f);
//         }

    // link() will set driver and try use acquire() link database
    // sqlite 
    rb.link(SqliteDriver {}, "sqlite://../target/sqlite.db").await.unwrap();
    // mysql 
    // rb.link(MysqlDriver{},"mysql://root:123456@localhost:3306/test").await.unwrap();
    // postgresql 
    // rb.link(PgDriver{},"postgres://postgres:123456@localhost:5432/postgres").await.unwrap();
    // mssql/sqlserver
    // rb.link(MssqlDriver{},"jdbc:sqlserver://localhost:1433;User=SA;Password={TestPass!123456};Database=test").await.unwrap();

    let table = BizActivity {
        id: Some("2".into()),
        name: Some("2".into()),
        pc_link: Some("2".into()),
        h5_link: Some("2".into()),
        pc_banner_img: None,
        h5_banner_img: None,
        sort: Some("2".to_string()),
        status: Some(2),
        remark: Some("2".into()),
        create_time: Some(DateTime::now()),
        version: Some(1),
        delete_flag: Some(1),
    };
    let tables = [table.clone(), {
        let mut t3 = table.clone();
        t3.id = "3".to_string().into();
        t3
    }];

    let data = BizActivity::insert(&rb, &table).await;
    println!("insert = {}", json!(data));

    let data = BizActivity::insert_batch(&rb, &tables, 10).await;
    println!("insert_batch = {}", json!(data));
}
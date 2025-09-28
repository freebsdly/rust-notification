use chrono::DateTime;
use sea_orm::DeriveEntityModel;

pub struct Repository {}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct PipeLineEntity {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: Option<u8>,
    pub created_at: DateTime<chrono::Utc>,
}

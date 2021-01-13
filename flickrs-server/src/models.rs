#[derive(Queryable)]
pub struct Image {
    pub id: i32,
    pub path: Option<String>,
}

use serde::Serialize;
#[derive(Queryable, Serialize)]
pub struct Attribute {
    pub id: i32,
    pub name: String,
}

use super::schema::attributes;
#[derive(Insertable)]
#[table_name = "attributes"]
pub struct NewAttribute<'a> {
    pub name: &'a str,
}

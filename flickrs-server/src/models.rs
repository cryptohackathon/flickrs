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
    pub group_id: Option<i32>,
}

#[derive(Queryable, Serialize)]
pub struct AttributeGroup {
    pub id: i32,
    pub name: String,
}

use super::schema::attributes;
#[derive(Insertable)]
#[table_name = "attributes"]
pub struct NewAttribute<'a> {
    pub name: &'a str,
    pub group_id: Option<&'a i32>,
}

use super::schema::attribute_groups;
#[derive(Insertable)]
#[table_name = "attribute_groups"]
pub struct NewAttributeGroup<'a> {
    pub name: &'a str,
}

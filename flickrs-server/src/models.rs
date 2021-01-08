#[derive(Queryable)]
pub struct Image {
    pub id: i32,
    pub path: Option<String>,
}
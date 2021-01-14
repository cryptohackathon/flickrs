pub mod models;
pub mod schema;

use crate::models::{Attribute, Image, NewAttribute, NewAttributeGroup};
use diesel::prelude::*;

extern crate dotenv;
#[macro_use]
extern crate diesel;

pub mod key_manager;

// Get the last inserted row
no_arg_sql_function!(
    last_insert_rowid,
    diesel::sql_types::Integer,
    "Represents the SQL last_insert_row() function"
);

/// Allocate a row for the image, setting the path to NULL. Returns the id of the image.
pub fn allocate_image_row<'a>(conn: &SqliteConnection) -> i32 {
    use schema::images;
    diesel::insert_into(images::table)
        .default_values()
        .execute(conn)
        .expect("Could not add row for image");
    diesel::select(last_insert_rowid)
        .get_result::<i32>(conn)
        .unwrap()
}

/// Set the path of an image with given id.
pub fn set_image_path<'a>(
    conn: &SqliteConnection,
    image_id: &i32,
    image_path: &str,
) -> Result<usize, diesel::result::Error> {
    use self::schema::images::dsl::{images, path};
    diesel::update(images.find(image_id))
        .set(path.eq(image_path))
        .execute(conn)
}

/// Get an image entry
///
/// We use .first because the id is UNIQUE in the database scheme
pub fn get_image_row(
    conn: &SqliteConnection,
    image_id: &i32,
) -> Result<Image, diesel::result::Error> {
    use self::schema::images::dsl::{id, images};
    images.filter(id.eq(image_id)).first::<Image>(conn)
}

/// Get all image ids
pub fn get_all_image_ids(conn: &SqliteConnection) -> Result<Vec<i32>, diesel::result::Error> {
    use self::schema::images::dsl::{id, images};
    images.select(id).load(conn)
}

/// Get all attributes
pub fn get_all_image_paths(
    conn: &SqliteConnection,
) -> Result<Vec<Option<String>>, diesel::result::Error> {
    use self::schema::images::dsl::{images, path};
    images.select(path).load(conn)
}

/// Get all attributes
pub fn get_all_attributes(
    conn: &SqliteConnection,
) -> Result<Vec<Attribute>, diesel::result::Error> {
    use self::schema::attributes::dsl::attributes;
    attributes.load(conn)
}

/// Get an attribute entry by its id
///
/// We use .first because the id is UNIQUE in the database scheme
pub fn get_attribute_by_id(
    conn: &SqliteConnection,
    attribute_id: &i32,
) -> Result<Attribute, diesel::result::Error> {
    use self::schema::attributes::dsl::{attributes, id};
    attributes
        .filter(id.eq(attribute_id))
        .first::<Attribute>(conn)
}

/// Get an attribute entry by its name
///
/// We use .first because the name is UNIQUE in the database scheme
pub fn get_attribute_by_name(
    conn: &SqliteConnection,
    attribute_name: &str,
) -> Result<Attribute, diesel::result::Error> {
    use self::schema::attributes::dsl::{attributes, name};
    attributes
        .filter(name.eq(attribute_name))
        .first::<Attribute>(conn)
}

/// Struct to parse the new attribute JSON argument to.
struct NewAttributeJson {
    name: String,
    group_id: Option<i32>,
}
/// Add a new attribute by name
pub fn add_attribute(conn: &SqliteConnection, name: &string, group_id: Option<&i32>) -> Result<i32, diesel::result::Error> {
    use self::schema::attributes;
    let res = diesel::insert_into(attributes::table)
        .values(NewAttribute { name, group_id })
        .execute(conn);
    res.map(|_| {
        diesel::select(last_insert_rowid)
            .get_result::<i32>(conn)
            .unwrap()
    })
}

/// Add a new attribute group by name
pub fn add_attribute_group(conn: &SqliteConnection, name: &str) -> Result<i32, diesel::result::Error> {
    use self::schema::attribute_groups;
    let res = diesel::insert_into(attribute_groups::table)
        .values(NewAttributeGroup { name })
        .execute(conn);
    res.map(|_| {
        diesel::select(last_insert_rowid)
            .get_result::<i32>(conn)
            .unwrap()
    })
}

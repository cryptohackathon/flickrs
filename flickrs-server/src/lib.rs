pub mod schema;
pub mod models;

use diesel::prelude::*;
use crate::models::Image;

extern crate dotenv;
#[macro_use] extern crate diesel;

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
        .execute(conn).expect("Could not add row for image");
    diesel::select(last_insert_rowid)
        .get_result::<i32>(conn)
        .unwrap()
}

/// Set the path of an image with given id.
pub fn set_image_path<'a>(conn: &SqliteConnection, image_id: &i32, image_path: &str) -> Result<usize, diesel::result::Error> {
    use self::schema::images::dsl::{images,path};
    diesel::update(images.find(image_id))
        .set(path.eq(image_path))
        .execute(conn)
}

/// Get an image entry
///
/// We use .first because the id is UNIQUE in the database scheme
pub fn get_image_row(conn: &SqliteConnection, image_id: &i32) -> Result<Image, diesel::result::Error> {
    use self::schema::images::dsl::{id,images};
    images.filter(id.eq(image_id))
        .first::<Image>(conn)
}
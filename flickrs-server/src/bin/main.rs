#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate dotenv;
use rocket::Data;
use rocket_contrib::json::Json;
use flickrs_sqlite::*;
use std::env;
use dotenv::dotenv;
use std::fs;
use serde::Serialize;
use flickrs_sqlite::models::Attribute;

/// The struct needed to connect to the database
#[database("imagesdb")]
pub struct ImagesDbConn(diesel::SqliteConnection);

/// The return value of the /<image_id> GET operation
#[derive(Serialize)]
struct GetImageReturnValue {
    succes: bool,
    image: Option<Vec<u8>>,
    error: Option<String>,
}

/// Fetch the bytes of an image with a given path.
fn get_image_bytes(path: Option<String>) -> Json<GetImageReturnValue> {
    let value = match path {
        None => GetImageReturnValue{succes: false, image: None,
            error: Some("image path was NULL".to_string())},
        Some(a) => match fs::read(a) {
            Ok(v) => GetImageReturnValue{succes: true, image: Some(v), error: None},
            Err(e) => GetImageReturnValue{succes: false, image: None,
                error: Some(e.to_string())}
        }};
    Json(value)
}


/// GET the image with given id. Returns an empty Vec if the image is not found.
///
/// ## API
/// * Address: root/<image_id>
/// * Data Needed: None
/// * Returns: {success: bool, image: Byte vector or Null, error: String or Null}
#[get("/<image_id>")]
fn api_get_get_image(connection: ImagesDbConn, image_id: i32) -> Json<GetImageReturnValue> {
    match get_image_row(&*connection, &image_id) {
        Ok(row) => get_image_bytes(row.path),
        _ => Json(GetImageReturnValue{
                  succes: false,
                  image: None,
                  error: Some(format!("image with id {} does not exist in the database", image_id))}),
    }
}

/// The return value from the /upload POST operation
#[derive(Serialize)]
struct UploadReturnValue {
    succes: bool,
    id: Option<i32>,
    error: Option<String>,
}

/// POST a new image to the database. Returns the image's ID, or -1 if something went wrong.
///
/// The name and directory for the new image can be changed in .env.
///
/// ## API
/// * Address: root/upload
/// * Data Needed: Any data
/// * Returns: {success: bool, id: int or NULL, error: string or Null}
#[post("/upload", data="<data>")]
fn api_post_upload_image(connection: ImagesDbConn, data: Data) -> Json<UploadReturnValue> {
    let id = allocate_image_row(&*connection);
    let filename = format!("{}{}", env::var("IMAGE_BASE_URL").expect("Image path not set!"), id);
    data.stream_to_file(&filename).expect("Could not read file");
    return match set_image_path(&*connection, &id, &filename) {
        Ok(_) => Json(UploadReturnValue{succes: true, id: Some(id), error: None}),
        Err(e) => Json(UploadReturnValue{succes: false, id: None, error: Some(e.to_string())}),
    }

}

///Launch the application
fn main() {
    dotenv().ok();
    rocket::ignite()
        .attach(ImagesDbConn::fairing())
        .mount("/", routes![api_get_get_image, api_post_upload_image])
        .launch();
}

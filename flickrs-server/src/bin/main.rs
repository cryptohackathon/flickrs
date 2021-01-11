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
use std::str::from_utf8;

/// The struct needed to connect to the database
#[database("imagesdb")]
pub struct ImagesDbConn(diesel::SqliteConnection);

/// The return value of the /<image_id> GET operation
#[derive(Serialize)]
struct GetImageReturnValue {
    success: bool,
    image: Option<Vec<u8>>,
    error: Option<String>,
}

/// Fetch the bytes of an image with a given path.
fn get_image_bytes(path: Option<String>) -> Json<GetImageReturnValue> {
    let value = match path {
        None => GetImageReturnValue{success: false, image: None,
            error: Some("image path was NULL".to_string())},
        Some(a) => match fs::read(a) {
            Ok(v) => GetImageReturnValue{success: true, image: Some(v), error: None},
            Err(e) => GetImageReturnValue{success: false, image: None,
                error: Some(e.to_string())}
        }};
    Json(value)
}


/// GET the image with given id.
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
                  success: false,
                  image: None,
                  error: Some(format!("image with id {} does not exist in the database", image_id))}),
    }
}

/// The return value from the /upload POST operation
#[derive(Serialize)]
struct UploadReturnValue {
    success: bool,
    id: Option<i32>,
    error: Option<String>,
}

/// POST a new image to the database.
///
/// The name and directory for the new image can be changed in .env.
///
/// ## API
/// * Address: root/upload
/// * Data Needed: Any data
/// * Returns: {success: bool, id: int or Null, error: string or Null}
#[post("/upload", data="<data>")]
fn api_post_upload_image(connection: ImagesDbConn, data: Data) -> Json<UploadReturnValue> {
    let id = allocate_image_row(&*connection);
    let filename = format!("{}{}", env::var("IMAGE_BASE_URL").expect("Image path not set!"), id);
    data.stream_to_file(&filename).expect("Could not read file");
    Json(match set_image_path(&*connection, &id, &filename) {
        Ok(_) => UploadReturnValue{success: true, id: Some(id), error: None},
        Err(e) => UploadReturnValue{success: false, id: None, error: Some(e.to_string())},
    })

}

/// The return value from the /attributes GET operation
#[derive(Serialize)]
struct AttributesReturnValue {
    success: bool,
    attributes: Option<Vec<Attribute>>,
    error: Option<String>,
}

/// GET all attributes from the database
///
/// ## API
/// * Address: root/attributes
/// * Data Needed: None
/// * Returns: {success: bool, attributes: List<????> or Null, error: string or Null}
#[get("/attributes")]
fn api_get_all_attributes(connection: ImagesDbConn) -> Json<AttributesReturnValue> {
    Json(match get_all_attributes(&*connection) {
        Ok(attributes) => AttributesReturnValue{success: true,
            attributes: Some(attributes), error: None},
        Err(e) => AttributesReturnValue{success: false,
            attributes: None, error: Some(e.to_string())}
    })
}


/// The return value from the /attributes/<attribute> GET operation
#[derive(Serialize)]
struct AttributeReturnValue {
    success: bool,
    attribute: Option<Attribute>,
    error: Option<String>,
}
/// GET an attribute from the database based on the attribute's
///
/// ## API
/// * Address: root/attributes/<attribute>
/// * Data Needed: None
/// * Returns: {success: bool, attribute: {id: int, name: string} or Null, error: string or Null}
#[get("/attributes/<attribute>", rank=0)]
fn api_get_get_attribute_by_id(connection: ImagesDbConn, attribute: i32) -> Json<AttributeReturnValue> {
    Json(match get_attribute_by_id(&*connection, &attribute) {
        Ok(attribute) => AttributeReturnValue{success: true, attribute: Some(attribute), error: None},
        Err(e) => AttributeReturnValue{success: false, attribute: None, error: Some(e.to_string())}
    })
}
/// GET an attribute from the database based on the attribute's name
///
/// ## API
/// * Address: root/attributes/<attribute>
/// * Data Needed: None
/// * Returns: {success: bool, attribute: {id: int, name: string} or Null, error: string or Null}
#[get("/attributes/<attribute>", rank=1)]
fn api_get_get_attribute_by_name(connection: ImagesDbConn, attribute: String) -> Json<AttributeReturnValue> {
    Json(match get_attribute_by_name(&*connection, &attribute) {
        Ok(attribute) => AttributeReturnValue{success: true, attribute: Some(attribute), error: None},
        Err(e) => AttributeReturnValue{success: false, attribute: None, error: Some(e.to_string())}
    })
}

/// POST a new attribute by name
///
/// WARNING: we reuse the UploadReturnValue for
///
/// * Address: root/attributes/new
/// * Data Needed: String containing the new name
/// * Returns: {success: bool, id: int or NULL, error: string or Null}
#[post("/attributes/new", data="<data>")]
fn api_post_new_attribute(connection: ImagesDbConn, data: Data) -> Json<UploadReturnValue> {
    Json(match add_attribute(&*connection, from_utf8(data.peek()).unwrap()){
        Ok(id) => UploadReturnValue{success: true, id:Some(id), error: None},
        Err(e) => UploadReturnValue{success: false, id: None,
            error: Some(e.to_string())}
    })
}

///Launch the application
fn main() {
    dotenv().ok();
    rocket::ignite()
        .attach(ImagesDbConn::fairing())
        .mount("/", routes![
        api_get_get_image,
         api_post_upload_image,
         api_get_all_attributes,
         api_get_get_attribute_by_id,
         api_get_get_attribute_by_name,
         api_post_new_attribute,
         ])
        .launch();
}

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::convert::TryFrom;
use std::env;
use std::fs;
use std::str::from_utf8;

use cife_rs::abe::dippe::{UserPrivateKey, UserPrivateKeySlice};
use dotenv::dotenv;
use flickrs_sqlite::key_manager::*;
use flickrs_sqlite::models::Attribute;
use flickrs_sqlite::*;
use rocket::{Data, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

/// The struct needed to connect to the database
#[database("imagesdb")]
pub struct ImagesDbConn(diesel::SqliteConnection);

/// Fetch the bytes of an image with a given path.
fn get_image_bytes(path: Option<String>) -> GetImageReturnValue {
    match path {
        None => GetImageReturnValue {
            success: false,
            image: None,
            error: Some("image path was NULL".to_string()),
        },
        Some(a) => match fs::read(a) {
            Ok(v) => GetImageReturnValue {
                success: true,
                image: Some(v),
                error: None,
            },
            Err(e) => GetImageReturnValue {
                success: false,
                image: None,
                error: Some(e.to_string()),
            },
        },
    }
}

/// The return value of the /<image_id> GET operation
#[derive(Serialize)]
struct GetImageReturnValue {
    success: bool,
    image: Option<Vec<u8>>,
    error: Option<String>,
}

/// GET the image with given id.
///
/// ## API
/// * Address: root/<image_id>
/// * Data Needed: None
/// * Returns: {success: bool, image: Byte vector or Null, error: String or Null}
#[get("/<image_id>")]
fn api_get_get_image(connection: ImagesDbConn, image_id: i32) -> Json<GetImageReturnValue> {
    Json(match get_image_row(&*connection, &image_id) {
        Ok(row) => get_image_bytes(row.path),
        _ => GetImageReturnValue {
            success: false,
            image: None,
            error: Some(format!(
                "image with id {} does not exist in the database",
                image_id
            )),
        },
    })
}

/// The return value of the /images GET operation
#[derive(Serialize)]
struct GetImagesReturnValue {
    success: bool,
    images: Option<Vec<Vec<u8>>>,
    error: Option<String>,
}
/// GET all images
///
/// ## API
/// * Address: root/images
/// * Data Needed: None
/// * Returns: {success: bool, image: List<Byte vector> or Null, error: String or Null}
#[get("/images")]
fn api_get_all_images(connection: ImagesDbConn) -> Json<GetImagesReturnValue> {
    let paths = get_all_image_paths(&*connection);
    if paths.is_err() {
        return Json(GetImagesReturnValue {
            success: false,
            images: None,
            error: paths.err().map(|e| e.to_string()),
        });
    }
    let paths = paths.unwrap();
    let images = paths
        .into_iter()
        .filter(Option::is_some)
        .map(get_image_bytes)
        .filter(|i| i.success)
        .map(|i| i.image.unwrap())
        .collect();
    Json(GetImagesReturnValue {
        success: true,
        images: Some(images),
        error: None,
    })
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
#[post("/upload", data = "<data>")]
fn api_post_upload_image(connection: ImagesDbConn, data: Data) -> Json<UploadReturnValue> {
    let id = allocate_image_row(&*connection);
    let filename = format!(
        "{}{}",
        env::var("IMAGE_BASE_URL").expect("Image path not set!"),
        id
    );
    data.stream_to_file(&filename).expect("Could not read file");
    Json(match set_image_path(&*connection, &id, &filename) {
        Ok(_) => UploadReturnValue {
            success: true,
            id: Some(id),
            error: None,
        },
        Err(e) => UploadReturnValue {
            success: false,
            id: None,
            error: Some(e.to_string()),
        },
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
        Ok(attributes) => AttributesReturnValue {
            success: true,
            attributes: Some(attributes),
            error: None,
        },
        Err(e) => AttributesReturnValue {
            success: false,
            attributes: None,
            error: Some(e.to_string()),
        },
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
#[get("/attributes/<attribute>", rank = 0)]
fn api_get_get_attribute_by_id(
    connection: ImagesDbConn,
    attribute: i32,
) -> Json<AttributeReturnValue> {
    Json(match get_attribute_by_id(&*connection, &attribute) {
        Ok(attribute) => AttributeReturnValue {
            success: true,
            attribute: Some(attribute),
            error: None,
        },
        Err(e) => AttributeReturnValue {
            success: false,
            attribute: None,
            error: Some(e.to_string()),
        },
    })
}
/// GET an attribute from the database based on the attribute's name
///
/// ## API
/// * Address: root/attributes/<attribute>
/// * Data Needed: None
/// * Returns: {success: bool, attribute: {id: int, name: string} or Null, error: string or Null}
#[get("/attributes/<attribute>", rank = 1)]
fn api_get_get_attribute_by_name(
    connection: ImagesDbConn,
    attribute: String,
) -> Json<AttributeReturnValue> {
    Json(match get_attribute_by_name(&*connection, &attribute) {
        Ok(attribute) => AttributeReturnValue {
            success: true,
            attribute: Some(attribute),
            error: None,
        },
        Err(e) => AttributeReturnValue {
            success: false,
            attribute: None,
            error: Some(e.to_string()),
        },
    })
}

/// The return value for a /setup GET request
#[derive(Serialize, Deserialize)]
struct ServerKeyResponse {
    server_key: cife_rs::abe::dippe::PublicKey,
}

/// GET the authority public key that is used for all attributes.
///
/// ## API
/// * Address: root/setup
/// * Data Needed: None
/// * Returns: {server_key: PublicKey}
#[get("/setup")]
fn api_get_server_key(keys: State<KeyMaterial>) -> Json<ServerKeyResponse> {
    Json(ServerKeyResponse {
        server_key: keys.inner().public.clone(),
    })
}

/// The request body of the /user/register POST operation
#[derive(Serialize, Deserialize)]
struct RegistrationRequest {
    gid: String,
    attributes: Vec<i32>,
}

/// The return value of the /user/register POST operation
#[derive(Serialize)]
struct RegistrationDetails {
    success: bool,
    registration_key: Option<UserPrivateKey>,
    error: Option<String>,
}

/// Register or re-register a user with a specific global identifier.
///
/// This endpoint expects some honest behaviour from the requestor.
/// Specifically, it expects that the requested GID is unique, and it doesn't validate the
/// requested attributes.
///
/// ## API
/// * Address: root/user/register
/// * Data Needed: [`RegistrationRequest`] POST'ed as JSON: `{gid: string, attributes: [int]}`
/// * Returns: {success: bool, registration_key: BLOB or Null, error: string or Null}
#[post("/user/register", data = "<request>")]
fn api_post_register_user_with_attributes(
    connection: ImagesDbConn,
    keys: State<KeyMaterial>,
    request: Json<RegistrationRequest>,
) -> Json<RegistrationDetails> {
    let keys = keys.inner();

    let db_attrs = match get_all_attributes(&*connection) {
        Ok(attrs) => attrs,
        Err(err) => {
            return Json(RegistrationDetails {
                success: false,
                registration_key: None,
                error: Some(format!("{}", err)),
            })
        }
    };

    let attr_count = db_attrs
        .iter()
        .map(|x| x.id)
        .max()
        .map(|x| x + 1)
        .unwrap_or(0) as usize;
    // We are the authority for every single key
    let authorities: Vec<_> = (0..attr_count).map(|_i| &keys.public).collect();

    let mut av = vec![];
    for attr in db_attrs {
        if request.attributes.contains(&attr.id) {
            av.push(attr.id as usize);
        }
    }
    if av.len() != request.attributes.len() {
        return Json(RegistrationDetails {
            success: false,
            registration_key: None,
            error: Some(format!(
                "Requested {} non-existing attribute(s)",
                request.attributes.len() - av.len()
            )),
        });
    }
    let av = keys.dippe.create_attribute_vector(attr_count, &av);

    let mut upks = vec![];
    for i in 0..attr_count {
        let upk = keys.dippe.generate_user_private_key_part(
            &keys.private,
            i,
            &authorities,
            request.gid.as_bytes(),
            &av,
        );
        upks.push(upk);
    }
    let upk: Result<UserPrivateKeySlice, _> = upks.into_iter().collect();
    let upk = UserPrivateKey::try_from(upk.unwrap()).unwrap();

    Json(RegistrationDetails {
        success: true,
        registration_key: Some(upk),
        error: None,
    })
}

/// POST a new attribute by name
///
/// WARNING: we reuse the UploadReturnValue for
///
/// * Address: root/attributes/new
/// * Data Needed: String containing the new name
/// * Returns: {success: bool, id: int or NULL, error: string or Null}
#[post("/attributes/new", data = "<data>")]
fn api_post_new_attribute(connection: ImagesDbConn, data: Data) -> Json<UploadReturnValue> {
    Json(
        match add_attribute(&*connection, from_utf8(data.peek()).unwrap()) {
            Ok(id) => UploadReturnValue {
                success: true,
                id: Some(id),
                error: None,
            },
            Err(e) => UploadReturnValue {
                success: false,
                id: None,
                error: Some(e.to_string()),
            },
        },
    )
}

///Launch the application
fn main() {
    dotenv().ok();

    let keys = KeyMaterial::load_from_storage()
        .unwrap_or_else(|| KeyMaterial::generate_and_persist(b"flickrs", 2));

    rocket::ignite()
        .attach(ImagesDbConn::fairing())
        .manage(keys)
        .mount(
            "/",
            routes![
                api_get_get_image,
                api_post_upload_image,
                api_get_all_images,
                api_get_all_attributes,
                api_get_get_attribute_by_id,
                api_get_get_attribute_by_name,
                api_post_new_attribute,
                api_post_register_user_with_attributes,
                api_get_server_key,
            ],
        )
        .launch();
}

#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel_migrations;

embed_migrations!();

use std::convert::TryFrom;
use std::env;
use std::fs;
use std::str::from_utf8;

use cife_rs::abe::dippe::{UserPrivateKey, UserPrivateKeySlice};
use dotenv::dotenv;
use flickrs_sqlite::key_manager::*;
use flickrs_sqlite::models::Attribute;
use flickrs_sqlite::*;
use rocket::{fairing::AdHoc, Data, Rocket, State};
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

/// The return value of the /delete_all_images POST operation
#[derive(Serialize)]
struct DeleteAllImagesReturnValue {
    success: bool,
    error: Option<String>,
}

/// POST to delete all images in the database
///
/// ## API
/// * Address: root/delete_all_images
/// * Data Needed: None
/// * Returns: { success: bool, error: Sting or Null}
#[post("/delete_all_images")]
fn api_delete_all_images(connection: ImagesDbConn) -> Json<DeleteAllImagesReturnValue> {
    let paths = get_all_image_paths(&*connection);

    if paths.is_err() {
        return Json(DeleteAllImagesReturnValue {
            success: false,
            error: paths.err().map(|e| e.to_string()),
        });
    }

    let paths = paths.unwrap();
    paths
        .into_iter()
        .filter(Option::is_some)
        .for_each(|f| std::fs::remove_file(f.unwrap()).unwrap());

    Json(match delete_all_images(&*connection) {
        Ok(_) => DeleteAllImagesReturnValue {
            success: true,
            error: None,
        },
        _ => DeleteAllImagesReturnValue {
            success: false,
            error: Some("Unable to delete all images".into()),
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

/// The return value of the /images_list GET operation
#[derive(Serialize)]
struct GetImageIdsReturnValue {
    success: bool,
    ids: Option<Vec<i32>>,
    error: Option<String>,
}
/// GET all images
///
/// ## API
/// * Address: root/images
/// * Data Needed: None
/// * Returns: {success: bool, image: List<Byte vector> or Null, error: String or Null}
#[get("/images_list")]
fn api_get_all_image_ids(connection: ImagesDbConn) -> Json<GetImageIdsReturnValue> {
    let ids = get_all_image_ids(&*connection);
    if ids.is_err() {
        return Json(GetImageIdsReturnValue {
            success: false,
            ids: None,
            error: ids.err().map(|e| e.to_string()),
        });
    }
    let ids = ids.unwrap();
    Json(GetImageIdsReturnValue {
        success: true,
        ids: Some(ids),
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
    let base = env::var("IMAGE_BASE_URL").expect("Image path not set!");
    let base = std::path::Path::new(&base);
    if let Err(e) = std::fs::create_dir_all(base) {
        log::warn!("{}", e);
    }
    let filename = base.join(id.to_string());
    data.stream_to_file(&filename).expect("Could not read file");
    Json(
        match set_image_path(&*connection, &id, filename.to_str().unwrap()) {
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
        },
    )
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
    server_key: Vec<u8>,
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
        server_key: keys.inner().public.clone().into_bytes(),
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
    registration_key: Option<Vec<u8>>,
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

    let attr_count = db_attrs.iter().map(|x| x.id).max().unwrap_or(0) as usize;
    let av_len = attr_count + 1;
    // We are the authority for every single key
    let authorities: Vec<_> = (0..av_len).map(|_i| &keys.public).collect();

    let mut av = vec![];
    for attr in db_attrs {
        if request.attributes.contains(&attr.id) {
            av.push(attr.id as usize - 1);
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
    for i in 0..av_len {
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
        registration_key: Some(upk.into_bytes()),
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

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    let c = ImagesDbConn::get_one(&rocket).expect("database connection");
    match embedded_migrations::run(&*c) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            log::error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    }
}

///Launch the application
fn main() {
    dotenv().ok();

    let keys = KeyMaterial::load_from_storage(b"flickrs", 2)
        .unwrap_or_else(|| KeyMaterial::generate_and_persist(b"flickrs", 2));

    rocket::ignite()
        .attach(ImagesDbConn::fairing())
        .attach(AdHoc::on_attach("Database Migrations", run_db_migrations))
        .manage(keys)
        .mount(
            "/api/",
            routes![
                api_get_get_image,
                api_post_upload_image,
                api_delete_all_images,
                api_get_all_images,
                api_get_all_image_ids,
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

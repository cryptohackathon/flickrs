#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate dotenv;
use rocket::Data;
use flickrs_sqlite::*;
use std::env;
use dotenv::dotenv;
use std::fs;

#[database("imagesdb")]
pub struct ImagesDbConn(diesel::SqliteConnection);

fn get_image_bytes(path: Option<String>) -> Vec<u8> {
    match path {
        None => Vec::new(),
        Some(a) => fs::read(a).unwrap_or(Vec::new()),
    }
}

#[get("/<image_id>")]
fn get_image(connection: ImagesDbConn, image_id: i32) -> Vec<u8> {
    match get_image_row(&*connection, &image_id) {
        Ok(row) => get_image_bytes(row.path),
        _ => Vec::new(),
    }
}

#[post("/upload", data="<data>")]
fn upload(connection: ImagesDbConn, data: Data) -> String {
    let id = allocate_image_row(&*connection);
    let filename = format!("{}{}", env::var("IMAGE_BASE_URL").expect("Image path not set!"), id);
    data.stream_to_file(&filename).expect("Could not read file");
    return match set_image_path(&*connection, &id, &filename) {
        Ok(_) => format!("Image uploaded: {}", id),
        Err(e) => format! {"Could not upload: {}", e.to_string()},
    }

}

fn main() {
    dotenv().ok();
    rocket::ignite()
        .attach(ImagesDbConn::fairing())
        .mount("/", routes![get_image, upload])
        .launch();
}

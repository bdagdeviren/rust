use std::io;
use rocket::response::{NamedFile};

use super::util::upload;

#[get("/")]
pub fn index() -> io::Result<NamedFile>{
    NamedFile::open("static/index.html")
}

#[post("/")]
pub fn upload_page() -> io::Result<NamedFile>{
    upload::upload();
    NamedFile::open("static/index.html")
}
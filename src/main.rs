#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::fs::File;
use std::path::Path;

use rocket::Data;

mod paste_id;

use paste_id::PasteId;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    let context = "";
    Template::render("index", &context)
}

#[post("/", data = "<paste>")]
fn upload(paste: Data) -> Result<String, std::io::Error> {
    let id = PasteId::new(10);
    let filename = format!("upload/{id}", id = id);
    let url = format!("{host}/{id}\n", host = "http://localhost:8000", id = id);

    paste.stream_to_file(Path::new(&filename))?;
    Ok(url)
}

#[get("/<id>")]
fn retrieve(id: PasteId) -> Option<File> {
    let filename = format!("upload/{id}", id = id);
    File::open(&filename).ok()
}

fn main() {
    rocket::ignite().attach(Template::fairing()).mount("/", routes![index, upload, retrieve]).launch();
}

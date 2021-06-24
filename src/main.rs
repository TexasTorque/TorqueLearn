#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod processing;
use rocket::fs::{FileServer, relative};

#[get("/*")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    processing::process();
    rocket::build()
        .mount("/", FileServer::from(relative!("deploy/"))) 
}


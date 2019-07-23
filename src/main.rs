#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

mod controller;
use crate::controller::{ get };

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                get::index,
                get::upload_page,
            ],
        )
}

fn main() {
    rocket().launch();
}
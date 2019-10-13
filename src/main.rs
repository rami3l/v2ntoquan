#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate lazy_static;

mod core;
mod routes;

use routes::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, v2rayn_to_quan,])
        .launch();
}

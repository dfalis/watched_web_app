#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    // get arguments
    let args: Vec<String> = env::args().collect();

    if args.len() {
        // TODO: check if any arguments were inserted
        let uri = &args[1];
        if !uri.starts_with("/") {
            panic!("Bad uri!\n");
        }
    }

    rocket::ignite().mount("/", routes![index]).launch();
}

#![feature(proc_macro_hygiene, decl_macro)]
use std::fmt::{self, Formatter, Display};
#[macro_use]
extern crate rocket;


#[get("/<number>")]
fn index(number: usize) -> String {
    format!("{}", (50+number))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    rocket().launch();
}

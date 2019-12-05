#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use std::collections::HashMap;

use rocket_contrib::databases::diesel;
use rocket_contrib::templates::Template;


#[database("default")]
struct DefaultDatabase(diesel::PgConnection);


#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", context)
}


#[get("/<path>")]
fn redirect(conn: DefaultDatabase, path: String) -> String {
    path
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![redirect])
        .attach(DefaultDatabase::fairing())
        .attach(Template::fairing())
        .launch();
}

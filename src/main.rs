#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

mod models;
mod schema;

use std::collections::HashMap;

use diesel::PgConnection;
use diesel::prelude::*;
use rocket::http::RawStr;
use rocket::request::Form;
use rocket::request::FromFormValue;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use models::MassiveURL;


#[database("default")]
struct DefaultDatabase(PgConnection);


#[get("/")]
fn index() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", context)
}


struct ValidatedUrl(String);


impl<'v> FromFormValue<'v> for ValidatedUrl {
    type Error = &'static str;
    fn from_form_value(form_value: &'v RawStr) -> Result<ValidatedUrl, Self::Error> {
        let mut decoded_string = String::from_form_value(form_value)
            .map_err(|_| "decode error")?;

        // Rewrite HTTP to HTTPS
        if decoded_string.starts_with("http://") {
            decoded_string.insert(4, 's');
        }

        // Don't allow javascript:, data: etc.
        if !decoded_string.starts_with("https://") {
            return Err("not HTTPS");
        }

        Ok(ValidatedUrl(decoded_string))
    }
}


#[derive(FromForm)]
struct URLForm {
    url: ValidatedUrl,
}


#[post("/", data="<form>")]
fn create(form: Form<URLForm>) -> Template {
    let mut context: HashMap<&str, &str> = HashMap::new();
    context.insert("url", &form.url.0);
    Template::render("result", context)
}


#[get("/<path_param>")]
fn redirect(conn: DefaultDatabase, path_param: String) -> Option<Redirect> {
    use schema::massive_urls::dsl::*;
    let result = massive_urls.filter(path.eq(&path_param))
        .limit(1)
        .load::<MassiveURL>(&*conn)
        .unwrap()
        .into_iter().nth(0);

    match result {
        Some(url) => Some(Redirect::to(url.destination)),
        None      => None,
    }
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/", routes![create])
        .mount("/", routes![redirect])
        .attach(DefaultDatabase::fairing())
        .attach(Template::fairing())
        .launch();
}

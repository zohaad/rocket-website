#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::response::NamedFile;

#[get("/")]
fn index() -> NamedFile {
    NamedFile::open("./static/index.html").unwrap()
} // contains link ( <a> ) to login page ...

// ... handle link to login page
#[get("/login")]
fn login() -> NamedFile {
    NamedFile::open("./static/login.html").unwrap()
} // contains sumbit button to / (post request)

#[derive(FromForm)]
struct Login {
    username: String,
    password: String,
}

use serde::Serialize;

#[derive(Serialize)]
struct LoginResponse {
    success: bool,
    username: String,
    password: String,
}

use rocket::request::Form;
use rocket_contrib::json::Json;

// ... handle submit data
#[post("/", data = "<login>")]
fn login_form(login: Form<Login>) -> Json<LoginResponse> {
    let login_response = LoginResponse { success: login.password == "hello", username: login.username.clone(), password: login.password.clone()};
    Json( login_response )
} // return JSON to /

use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
           .mount("/", routes![index, login, login_form])
           .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static"))) // stole this from the docs
           .launch();
}
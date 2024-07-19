#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket::fs::FileServer;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![index])
    .attach(Template::fairing())
    .mount("/static", FileServer::from("./static"))
}
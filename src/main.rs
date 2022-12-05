use api::dotafetcher::search_match_history_by_player_id;
use structs::cors_struct::Cors;

mod api;
mod constants;
mod structs;
mod traits;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn launch() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/", routes![index, search_match_history_by_player_id])
}

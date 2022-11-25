use api::dotafetcher::search_match_history_by_player_id;

mod api;
mod constants;
mod structs;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, search_match_history_by_player_id])
        .launch()
        .await?;

    Ok(())
}

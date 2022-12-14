use std::error::Error;

use reqwest::Client;
use rocket::{http::Status, serde::json::Json};

use crate::{
    constants::appsettings::APP_SETTINGS,
    structs::{
        command_result_struct::CommandResultStruct, matches_struct::MatchHistoryByPlayerIdStruct,
    },
    traits::command_result_traits::CommandResult,
};

async fn search_match_history_by_player_id_opendota(
    url: &str,
) -> Result<Vec<MatchHistoryByPlayerIdStruct>, Box<dyn Error>> {
    let client = Client::new();

    let response: Vec<MatchHistoryByPlayerIdStruct> = client.get(url).send().await?.json().await?;
    println!("response: {:#?}", response);
    Ok(response)
}

#[get("/search-match-history-by-player-id?<id>")]
pub async fn search_match_history_by_player_id(
    id: &str,
) -> Result<
    Json<CommandResultStruct<Vec<MatchHistoryByPlayerIdStruct>>>,
    Json<CommandResultStruct<&str>>,
> {
    let url = format!(
        "{}/players/{}/matches?limit=10",
        APP_SETTINGS.get_url_opendota(),
        id
    );
    match search_match_history_by_player_id_opendota(&url).await {
        //Ok(result) => Ok(status::Custom(Status::Ok, Json(result))),
        Ok(result) => Ok(Json(CommandResultStruct::ok_response(result))),
        // Err(err) => {
        //     //println!("error: {:?}", err);
        //     // let mut retval: Vec<MatchHistoryByPlayerIdStruct> = Vec::new();
        //     // retval.push(MatchHistoryByPlayerIdStruct::create_error_type());
        //     //let retval: ErrorResponseStruct<&str> = ErrorResponseStruct::<&str> {
        //     //    message: "Error search match history by player id",
        //     //    data: None,
        //     //};
        //     //return Err(status::Custom(Status::InternalServerError, Json(retval)));
        // }
        Err(err) => Ok(Json(CommandResult::error_response(
            Status::InternalServerError.code,
            err.to_string().as_str(),
        ))),
    }
}

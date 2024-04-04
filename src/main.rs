use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "edit-roblox-place",
)]
struct Opt {
    place_id: u64,
}

#[derive(Serialize, Deserialize)]
struct UniverseJson {
    #[serde(rename = "universeId")]
    universe_id: u64
}

async fn get_universe_id(place_id: u64) -> u64 {
    let url: String = format!("https://apis.roblox.com/universes/v1/places/{}/universe", place_id);
    match reqwest::get(url).await {
        Ok(response) => {
            if response.status() != reqwest::StatusCode::OK {
                println!("Api request Status Code not OK!");
            }

            match response.json::<UniverseJson>().await {
                Ok(json) => { json.universe_id }
                Err(_) => {println!("Could not get JSON!"); 0}
            }
        }
        Err(_) => { println!("Could not make API Request!"); 0 }
    }
}

#[tokio::main]
async fn main() {
    let args = Opt::from_args();
    let universe_id: u64 = get_universe_id(args.place_id).await;

    let uri: String = format!("roblox-studio:1+task:EditPlace+placeId:{}+universeId:{}", args.place_id, universe_id);
    opener::open(uri).expect("Couldn't open Roblox Studio");
}

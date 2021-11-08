mod errors;
mod http;
mod serverapi;
mod twitchapi;
use std::env;
mod structs;

use dotenv::dotenv;

use serverapi::serverclient::ServerClient;
use serverapi::serverconfig::ServerConfig;
use twitchapi::twitchclient::TwitchClient;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client_id = env::var("TWITCH_USERID").unwrap().to_owned();
    let client_secret = env::var("TWITCH_SECRET").unwrap().to_owned();

    let app_name = env::var("APP_NAME").unwrap().to_owned();
    let app_password = env::var("APP_PASSWORD").unwrap().to_owned();
    let app_base_url = env::var("APP_BASE_URL").unwrap().to_owned();

    let handler_twitch = http::customclient::CustomClient::new(10);
    let mut twitch_client =
        TwitchClient::new(handler_twitch, client_id.as_str(), client_secret.as_str());

    let server_config = ServerConfig::new(app_name, app_password, app_base_url);
    let handler_server = http::customclient::CustomClient::new(5);
    let mut server_client = ServerClient::new(server_config, handler_server);

    let mut streamers = match server_client.get_all_channels().await {
        Ok(data) => data,
        Err(e) => { 
            println!("{}", e);
            return;
        }
    };

    for streamer in streamers.iter_mut() {
        if streamer.channel.broadcast_id.is_some() {
            continue;
        }

        match twitch_client.get_broadcaster_id(streamer.channel.broadcast_name.as_str()).await {
            Ok(data) => {
                streamer.channel.broadcast_id = Some(data);
                server_client.update_channel_broadcast_id(streamer, data).await.expect("Error updating broadcast_id");
            },
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    loop {
        for streamer in streamers.iter_mut() {
            let data = match twitch_client.get_stream_data(streamer).await {
                Ok(data) => data,
                Err(e) => {
                    println!("Error occured while fetching {}, error: {}", streamer.channel.broadcast_name, e);
                    continue;
                },
            };

            if data.data.len() == 0 {
                println!("{} not streaming", streamer.channel.broadcast_name);
                if streamer.was_live == true {
                    streamer.was_live = false;
                    streamer.is_live = false;
                    server_client.add_streamer_data(streamer).await.unwrap();
                }
            } else {
                println!(
                    "{} is streaming {} with title {}",
                    streamer.channel.broadcast_name, data.data[0].game_name, data.data[0].title
                );
                streamer.is_live = true;
                streamer.was_live = true;
                streamer.game_name = data.data[0].game_name.clone();
                streamer.viewer_count = data.data[0].viewer_count as i32;
                streamer.title = data.data[0].title.clone();

                server_client.add_streamer_data(streamer).await.unwrap();
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
        std::thread::sleep(std::time::Duration::from_secs(30));
    }
}

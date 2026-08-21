use capitalize::Capitalize;
use obws::{Client, requests::sources::SourceId};
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    let mute_source = args.get(1).expect("Mute source is not provided!");

    let text: Result<String, _> = trpl::block_on(async {
        let client: Client = match Client::connect("127.0.0.1", 4455, Some("")).await {
            Ok(client) => client,
            Err(e) => return Err(e),
        };

        let current_scene = get_current_scene(&client).await?;
        let is_muted = is_black_screen_muted(&client, mute_source).await?;

        let mut output = current_scene;
        if is_muted {
            output.push_str(" (muted)");
        }

        Ok(format!("{}", output))
    });

    println!("{}", text.unwrap_or("".to_string()).capitalize());
}

async fn get_current_scene(client: &Client) -> Result<String, obws::error::Error> {
    let scenes = client.scenes();
    let current_scene = scenes.current_program_scene().await?.id.name;
    Ok(current_scene)
}

async fn is_black_screen_muted(
    client: &Client,
    mute_source: &str,
) -> Result<bool, obws::error::Error> {
    let sources = client.sources();
    let black_screen_id = SourceId::from(mute_source);
    let is_muted = sources.active(black_screen_id).await?.active;
    Ok(is_muted)
}

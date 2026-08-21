use capitalize::Capitalize;
use obws::{Client, requests::sources::SourceId};
use std::env::args;

fn main() {
    let mute_source: Option<String> = args().nth(1);

    let text: Result<String, obws::error::Error> = trpl::block_on(async {
        let client: Client = Client::connect("127.0.0.1", 4455, Some("")).await?;

        let current_scene = get_current_scene(&client).await?;
        let is_muted = is_black_screen_muted(&client, mute_source.as_deref()).await?;

        let mut output = current_scene;
        if is_muted {
            output.push_str(" (muted)");
        }

        Ok(output)
    });

    println!("{}", text.unwrap_or_default().capitalize());
}

async fn get_current_scene(client: &Client) -> Result<String, obws::error::Error> {
    let scenes = client.scenes();
    let current_scene = scenes.current_program_scene().await?.id.name;
    Ok(current_scene)
}

async fn is_black_screen_muted(
    client: &Client,
    mute_source: Option<&str>,
) -> Result<bool, obws::error::Error> {
    if let Some(mute_source) = mute_source {
        let sources = client.sources();
        let black_screen_id = SourceId::from(mute_source);
        let is_muted = sources.active(black_screen_id).await?.active;
        Ok(is_muted)
    } else {
        Ok(false)
    }
}

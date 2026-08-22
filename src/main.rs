use capitalize::Capitalize;
use clap::Parser;
use obws::{Client, requests::sources::SourceId};

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub blur: Option<String>,
    #[arg(short, long, default_value = "127.0.0.1")]
    pub address: String,
    #[arg(short = 'P', long, default_value_t = 4455)]
    pub port: u16,
    #[arg(short, long, default_value = "")]
    pub password: String,
}

type RespSceneId = obws::responses::scenes::SceneId;

#[tokio::main]
async fn main() -> Result<(), obws::error::Error> {
    let args = Args::parse();

    let client = match Client::connect(args.address, args.port, Some(&args.password)).await {
        Ok(c) => c,
        Err(e) => match e {
            // OBS WebSocket plugin is not running
            obws::error::Error::Connect(_) => return Ok(()),
            _ => return Err(e),
        },
    };

    let current_scene = get_current_scene(&client).await?.name;
    let is_blurred = if let Some(blur_source) = &args.blur {
        is_source_active(&client, blur_source).await?
    } else {
        false
    };

    let mut output = current_scene;
    if is_blurred {
        output.push_str(" (blurred)");
    }

    // let active_sources = get_all_active_sources(&client).await.unwrap_or_default();

    let out = serde_json::json!({
        "text": output.capitalize(),
        "tooltip": "Soon!",
        "class": if is_blurred { "blurred" } else { "" }
    });

    println!("{}", out);
    Ok(())
}

async fn get_current_scene(client: &Client) -> Result<RespSceneId, obws::error::Error> {
    let scenes = client.scenes();
    let current_scene = scenes.current_program_scene().await?.id;
    Ok(current_scene)
}

async fn is_source_active(client: &Client, source_name: &str) -> Result<bool, obws::error::Error> {
    let sources = client.sources();
    let black_screen_id = SourceId::from(source_name);
    let is_blurred = sources.active(black_screen_id).await?.active;
    Ok(is_blurred)
}

// TODO
// async fn get_all_active_sources(client: &Client) -> Result<Vec<String>, obws::error::Error> {
//     let current_scene = get_current_scene(client).await?;
//     let scene_items_api = client.scene_items();
//     let source = scene_items_api.source().await?;
//     println!("{:?}", source);
//     let sources = scene_items_api
//         .list(ReqSceneId::from(current_scene))
//         .await?;

//     Ok(sources.into_iter().map(|s| s.).collect())
// }

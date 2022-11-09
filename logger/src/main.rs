#[macro_use]
extern crate log;

use battlefield_rcon::{rcon::{RconConnectionInfo}, bf4::{Bf4Client, Event, Visibility}};
use chrono_tz::Tz;
use dotenv::{dotenv, var};
use tokio_stream::StreamExt;
use ascii::{IntoAsciiString};

use crate::discord::send_message_webhook;

mod discord;
mod logging;

fn get_rcon_coninfo() -> anyhow::Result<RconConnectionInfo> {
    let ip = var("RCON_IP").unwrap_or_else(|_| "127.0.0.1".into());
    let port = var("RCON_PORT")
        .unwrap_or_else(|_| "47200".into())
        .parse::<u16>()
        .unwrap();
    let password = var("RCON_PASSWORD").unwrap_or_else(|_| "smurf".into());
    Ok(RconConnectionInfo {
        ip,
        port,
        password: password.into_ascii_string()?,
    })
}

fn get_timezone() -> Tz {
    let timezone = dotenv::var("CHRONO_TIMEZONE").unwrap_or("Europe/Helsinki".to_string());
    timezone.parse().unwrap()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    logging::init_logging();

    info!("BF4 Chat Logging starting");
    info!("Using time zone: {}", get_timezone().name());

    let webhook_path = dotenv::var("DISCORD_WEBHOOK").unwrap();
    let rconinfo = get_rcon_coninfo()?;

    let bf4 = Bf4Client::connect((rconinfo.ip, rconinfo.port), rconinfo.password).await.unwrap();
    let mut event_stream = bf4.event_stream().await.unwrap();
    while let Some(ev) = event_stream.next().await {
        match ev {
            Ok(Event::Chat { vis, player, msg }) => {
                if vis == Visibility::All {
                    if let Err(err) = send_message_webhook(&webhook_path, player.name.as_str(), msg.as_str()).await {
                        error!("Error sending message: {}", err)
                    };
                }
            },
            Ok(_) => {}, // ignore other events.
            Err(err) => {
                error!("Got error: {:?}", err);
            },
        }
    }

    Ok(())
}

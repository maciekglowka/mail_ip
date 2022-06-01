use serde::Deserialize;
use std::error::Error;
use std::net::IpAddr;
use tokio;
use toml;

mod email;
mod ip;

const SETTINGS_FILE: &str = "settings.toml";

#[derive(Clone, Debug, Deserialize)]
struct Settings {
    smtp_login: String,
    smtp_pass: String,
    smtp_host: String,
    send_to: String,
    ip_api_url: String,
    interval_secs: u64,
    ip_file_path: String
}

async fn get_settings() -> Settings {
    let file = tokio::fs::read_to_string(SETTINGS_FILE)
        .await
        .expect("Error while reading settings file!");
    toml::from_str(&file).expect("Settings could not be parsed!")
}

async fn handle_ip_change(ip: IpAddr, settings: Settings) -> Result<(), Box<dyn Error>> {
    email::send_ip_info(
        ip,
        &settings.send_to,
        &settings.smtp_login,
        &settings.smtp_pass,
        &settings.smtp_host
    ).await?;
    ip::write_current_ip(ip, &settings.ip_file_path).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let settings = get_settings().await;
    let delay = tokio::time::Duration::from_secs(settings.interval_secs);
    let mut last = None;
    if let Ok(s) = ip::read_last_ip(&settings.ip_file_path).await {
        last = Some(s);
    }

    loop {
        match ip::get_current_ip(&settings.ip_api_url).await {
            Ok(current) => {
                if Some(current) != last {
                    let s = settings.clone();
                    let join = tokio::spawn(async move {
                        let res = handle_ip_change(current, s).await;
                        match res {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                eprintln!("{}", e);
                                Err(())
                            }
                        }
                    }).await;

                    if let Ok(Ok(_)) = join {
                        last = Some(current);
                    }
                }
            },
            Err(e) => eprintln!("{}", e)
        }
        tokio::time::sleep(delay).await;
    }
}

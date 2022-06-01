use reqwest;
use std::error::Error;
use std::net::IpAddr;
use tokio;

pub async fn get_current_ip(api_url: &String) -> Result<IpAddr, Box<dyn Error>> {
    let res = reqwest::get(api_url).await?;
    let mut text = res.text().await?;
    text.retain(|a| !a.is_whitespace());

    Ok(text.parse::<IpAddr>()?)
}

pub async fn write_current_ip(ip: IpAddr, path: &String) -> Result<(), Box<dyn Error>> {
    tokio::fs::write(path, ip.to_string()).await?;
    Ok(())
}

pub async fn read_last_ip(path: &String) -> Result<IpAddr, Box<dyn Error>> {
    let file = tokio::fs::read_to_string(path).await?;
    Ok(file.parse::<IpAddr>()?)
}
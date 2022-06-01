use lettre::{
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, Message, Tokio1Executor, AsyncTransport};
use std::error::Error;
use std::net::IpAddr;

pub async fn send_ip_info(
    ip: IpAddr,
    receipient: &String,
    smtp_login: &String,
    smtp_pass: &String,
    smtp_host: &String
)  -> Result<(), Box<dyn Error>> {
    let body = format!("The IP has changed to: {}", ip);
    let email = Message::builder()
        .from(smtp_login.parse()?)
        .to(receipient.parse()?)
        .subject("IP has changed")
        .body(body)?;

    let credentials = Credentials::new(smtp_login.to_string(), smtp_pass.to_string());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_host)?
        .credentials(credentials).build();

    mailer.send(email).await?;
    Ok(())
}
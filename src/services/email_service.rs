use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
use crate::config::SmtpConfig;
use crate::models::email::{EmailAddress, SendEmailRequest};

pub async fn send_email(req: SendEmailRequest, smtp_config: &SmtpConfig) -> Result<(), mail_send::Error> {
    // Convert EmailAddress to tuple format (name, email)
    let from_tuple = match &req.from.name {
        Some(name) => (name.as_str(), req.from.email.as_str()),
        None => (req.from.email.as_str(), req.from.email.as_str()), // Use email as name if none provided
    };

    // Convert Vec<EmailAddress> to Vec<(name, email)> tuples
    let to_tuples: Vec<(&str, &str)> = req.to
        .iter()
        .map(|addr| {
            match &addr.name {
                Some(name) => (name.as_str(), addr.email.as_str()),
                None => (addr.email.as_str(), addr.email.as_str()), // Use email as name if none provided
            }
        })
        .collect();

    let message = MessageBuilder::new()
        .from(from_tuple)
        .to(to_tuples)
        .subject(req.subject)
        .html_body(req.html_body)
        .text_body(req.text_body);

    SmtpClientBuilder::new(smtp_config.host.clone(), smtp_config.port)
        .implicit_tls(smtp_config.use_tls)
        .credentials((smtp_config.username.clone(), smtp_config.password.clone()))
        .connect()
        .await?
        .send(message)
        .await?;

    Ok(())
}
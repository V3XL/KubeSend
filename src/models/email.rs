use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailAddress {
    pub name: Option<String>,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendEmailRequest {
    pub to: Vec<EmailAddress>,
    pub from: EmailAddress,
    pub subject: String,
    pub html_body: String,
    pub text_body: String,
}
use lettre::{
    message::{Mailbox, MultiPart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use secrecy::{ExposeSecret, SecretString};

use crate::domain::SubscriberEmail;

#[derive(Clone)]
pub struct EmailClient {
    sender: SubscriberEmail,
    timeout: std::time::Duration,
    #[allow(dead_code)]
    name: Option<String>,
    username: Option<String>,
    password: SecretString,
    smtp_server: String,
}

impl EmailClient {
    pub fn new(
        sender: SubscriberEmail,
        timeout: std::time::Duration,
        name: Option<String>,
        username: Option<String>,
        password: SecretString,
        smtp_server: String,
    ) -> Self {
        Self {
            sender,
            timeout,
            name,
            username,
            password,
            smtp_server,
        }
    }

    pub async fn send_email(
        &self,
        recipient: &SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), String> {
        let from = Mailbox {
            name: Some(self.smtp_server.clone()),
            email: self.sender.as_ref().parse().unwrap(),
        };

        let email = Message::builder()
            .from(from)
            .to(recipient.as_ref().parse().unwrap())
            .subject(subject)
            .multipart(MultiPart::alternative_plain_html(
                String::from(text_content),
                String::from(html_content),
            ))
            .unwrap();
        let username = match &self.username {
            None => self.sender.to_string(),
            Some(username) => username.clone(),
        };

        let mailer = SmtpTransport::relay(&self.smtp_server)
            .expect("Something went wrong")
            .credentials(Credentials::new(
                username,
                self.password.expose_secret().to_owned(),
            ))
            .timeout(Some(self.timeout))
            .build();

        // NOTE: send the email
        mailer.send(&email).expect("Can't send email!");

        Ok(())
    }
}

#[allow(dead_code)]
#[derive(serde::Serialize)]
#[serde(rename_all = "PascalCase")]
struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    html_body: &'a str,
    text_body: &'a str,
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::Faker;
    use fake::{faker::internet::en::SafeEmail, Fake};
    use secrecy::SecretString;

    /// Generate a random email subject
    fn subject() -> String {
        Sentence(1..2).fake()
    }

    /// Generate a random email content
    fn content() -> String {
        Paragraph(1..10).fake()
    }

    /// Generate a random subscriber email
    fn email() -> SubscriberEmail {
        SubscriberEmail::parse(SafeEmail().fake()).unwrap()
    }

    /// Get a test instance of `EmailClient`
    fn email_client(smtp_server: String) -> EmailClient {
        EmailClient::new(
            email(),
            std::time::Duration::from_millis(200),
            None,
            None,
            SecretString::from(Faker.fake::<String>()),
            smtp_server,
        )
    }

    // TODO: tests with smtp email client
    #[tokio::test]
    async fn test_smpt_email_client_send_email() {
        // FIXME: Maybe use PyO3 here? https://tinyurl.com/mwhxzan9
        // Command::new("python")
        //     .arg("-m smtpd -n -c DebuggingServer 127.0.0.1:2525")
        //     .output()
        //     .expect("Failed to execute command");
    }
}

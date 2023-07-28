use crate::AppState;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn email(state: AppState, mail: &str) {
    let email = Message::builder()
        .from("noreply <nobody@gardenofeden.rs>".parse().unwrap())
        .to(mail.parse().unwrap())
        .subject("Welcome to The Garden Of Eden")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(
            "Enter the Garden Of Eden. I'll let you partake the fruit🍑",
        ))
        .unwrap();

    let creds = Credentials::new(state.smtp_email, state.smtp_password);

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    if let Err(err) = mailer.send(&email) {
        eprintln!("{:?}", err);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[ignore]
    fn send_email_test() {
        let email = Message::builder()
            .from("GardenOfEden <nobody@gardenofeden.rs>".parse().unwrap())
            .to("<tmwangi599@gmail.com>".parse().unwrap())
            .subject("Welcome to GardenOfEden")
            .header(ContentType::TEXT_PLAIN)
            .body(String::from(
                "Enter the Garden Of Eden. I'll let you partake the fruit",
            ))
            .unwrap();

        // To get your smtp relay to work with gmail
        // enable 2-FA then autogenerate passwords for a specific app
        let creds = Credentials::new("your_email_address".to_owned(), "your_password".to_owned());

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        assert!(mailer.send(&email).is_ok());
    }
}

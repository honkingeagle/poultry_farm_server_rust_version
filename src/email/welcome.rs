use crate::SharedState;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub async fn mail(state: SharedState, mail: &str) {
    let activation_token: u128 = rand::random();

    let activation_token = activation_token.to_string();

    let query = sqlx::query("INSERT INTO activations (email, activation_id) VALUES ($1, $2)")
        .bind(&mail)
        .bind(&activation_token)
        .execute(&state.pool)
        .await;

    match query {
        Ok(_) => {
            let email = Message::builder()
                .from("noreply <nobody@gardenofeden.rs>".parse().unwrap())
                .to(mail.parse().unwrap())
                .subject("Email verification")
                .header(ContentType::TEXT_HTML)
                .body(format!(
                    "<div>
                    <h1>Enter the Garden Of Eden.</h1>
                    I'll let you partake the fruit. Only if you 
                    <a href='http://127.0.0.1:8000/users/activate/{mail}/{activation_token}'>
                        Activate your account
                    </a>
                </div>"
                ))
                .unwrap();
            let creds = Credentials::new(state.smtp_email.to_string(), state.smtp_password.to_string());

            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();

            if let Err(err) = mailer.send(&email) {
                eprintln!("{:?}", err);
            }
        }
        Err(err) => eprintln!("Something happened: {}", err),
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
            .header(ContentType::TEXT_HTML)
            .body(String::from(
                "<div>
                    <h1>Enter the Garden Of Eden.</h1>
                    I'll let you partake the fruit. Only if you verify
                </div>",
            ))
            .unwrap();

        // To get your smtp relay to work with gmail
        // enable 2-FA then autogenerate passwords for a specific app
        let creds = Credentials::new("your_email".to_owned(), "your_password".to_owned());

        let mailer = SmtpTransport::relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        assert!(mailer.send(&email).is_ok());
    }
}

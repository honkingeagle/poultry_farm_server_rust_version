use crate::SharedState;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Email {
    email: String
}

pub async fn mail(
    State(state): State<SharedState>, 
    Json(mail): Json<Email>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let activation_token: u16 = rand::random();
    let activation_token = activation_token.to_string();
    let email = &mail.email;

    let query = sqlx::query("INSERT INTO activations 
                            (email, activation_id) VALUES ($1, $2) ON CONFLICT (email)
                            DO UPDATE SET activation_id = EXCLUDED.activation_id")
        .bind(email)
        .bind(&activation_token)
        .execute(&state.pool)
        .await;

    match query {
        Ok(_) => {
            let email = Message::builder()
                .from("noreply <nobody@gardenofeden.rs>".parse().unwrap())
                .to(email.parse().unwrap())
                .subject("Email verification")
                .header(ContentType::TEXT_HTML)
                .body(format!(
                    "<div>
                     <h1>Garden Of Eden.</h1>
                       Enter code: {activation_token} 
                     </div>"
                ))
                .unwrap();
            let creds = Credentials::new(state.smtp_email.to_string(), state.smtp_password.to_string());

            let mailer = SmtpTransport::relay("smtp.gmail.com")
                .unwrap()
                .credentials(creds)
                .build();

             match mailer.send(&email) {
                Ok(_) => Ok((StatusCode::OK, "Verification code generated".to_string())),
                Err(err) => Err((StatusCode::BAD_REQUEST, format!("Something happened: {err}")))
             }
        }
        Err(err) => Err((StatusCode::BAD_REQUEST, format!("Something happened: {err}")))
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
            .subject("Email verification")
            .header(ContentType::TEXT_HTML)
            .body(String::from(
                "<div>
                    <h1>Garden Of Eden.</h1>
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

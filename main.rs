use std::env;
use reqwest::Error;
use serde::Deserialize;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use tokio;

#[derive(Deserialize)]
struct ExchangeRateResponse {
    rates: std::collections::HashMap<String, f64>,
}

async fn fetch_exchange_rate(base: &str, target: &str) -> Result<f64, Error> {
    let api_url = env::var("API_URL").expect("API_URL must be set in .env");
    let request_url = format!("{}/{}", api_url, base);  
    let response: ExchangeRateResponse = reqwest::get(&request_url).await?.json().await?;

    response.rates.get(target)
        .copied()
        .ok_or_else(|| Error::new(reqwest::StatusCode::NOT_FOUND, "Target currency not found"))
}

fn send_email_alert(base: &str, target: &str, exchange_rate: f64) {
    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set in .env");
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set in .env");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set in .env");
    let recipient_email = env::var("RECIPIENT_EMAIL").expect("RECIPIENT_EMAIL must be set in .env");

    let email = Message::builder()
        .from(smtp_username.parse().unwrap())
        .to(recipient_email.parse().unwrap())
        .subject(format!("Currency Alert: {} to {} Exchange Rate", base, target))
        .body(format!("The {} to {} exchange rate has reached {:.2}!", base, target, exchange_rate))
        .unwrap();

    let creds = Credentials::new(smtp_username.clone(), smtp_password);

    let mailer = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .credentials(creds)
        .authentication(Mechanism::Plain)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Alert email sent successfully."),
        Err(e) => println!("Could not send email: {:?}", e),
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let alert_threshold: f64 = env::var("ALERT_THRESHOLD")
        .expect("ALERT_THRESHOLD must be set in .env")
        .parse()
        .expect("ALERT_THRESHOLD must be a number");

    let base_currency = env::var("BASE_CURRENCY").unwrap_or_else(|_| "EUR".to_string());
    let target_currency = env::var("TARGET_CURRENCY").unwrap_or_else(|_| "BRL".to_string());

    match fetch_exchange_rate(&base_currency, &target_currency).await {
        Ok(rate) => {
            println!("Current {} to {} rate: {:.2}", base_currency, target_currency, rate);
            if rate <= alert_threshold {
                println!(
                    "Alert! The {} to {} rate is below the threshold of {:.2}.",
                    base_currency, target_currency, alert_threshold
                );
                send_email_alert(&base_currency, &target_currency, rate);
            } else {
                println!("Rate is above the threshold, no alert sent.");
            }
        }
      
        Err(e) => println!("Error fetching exchange rate: {:?}", e),
    }
}

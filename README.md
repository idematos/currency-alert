# Currency Alert 

This is a currency alert system that monitors exchange rates between two configurable currencies. When the exchange rate reaches a specified threshold, the program sends an email notification alerting the user. The base currency, target currency, and alert threshold are all configurable through environment variables.

## Features

- Fetches real-time exchange rates for any currency pair.
- Sends an email alert when the specified threshold is reached.
- Easily configurable alert conditions, currencies, and SMTP settings via .env file.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/) installed.
- An email service with SMTP access.
- Access to an exchange rate API, such as [ExchangeRate-API](https://www.exchangerate-api.com/) or any other provider that supports setting a base currency dynamically.

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/idematos/currency-alert.git
   cd currency-alert
   ```

2. Set up the .env file to configure the environment variables (see configuration details below).

3. Install the required Rust dependencies by running:

   ```bash
   cargo build
   ```

## Configuration

Create a .env file in the project directory and add the following configurations:

```plaintext
# Exchange Rate API URL (replace with actual API if necessary)
API_URL=https://api.exchangerate-api.com/v4/latest

# SMTP server configurations for sending email alerts
SMTP_SERVER=smtp.example.com
SMTP_USERNAME=your_email@example.com
SMTP_PASSWORD=your_password
RECIPIENT_EMAIL=recipient@example.com

# Threshold and currency settings
ALERT_THRESHOLD=5.0       # Trigger alert when rate falls below this value
BASE_CURRENCY=EUR         # Base currency to monitor
TARGET_CURRENCY=BRL       # Target currency to compare
```

- `API_URL`: The base URL of the exchange rate API, which should support setting a base currency dynamically.
- **SMTP Configurations**: Set up your SMTP server information and credentials.
- **Threshold and Currencies**: Customize `ALERT_THRESHOLD`, `BASE_CURRENCY`, and `TARGET_CURRENCY` to monitor different exchange rates and trigger alerts when the rate falls below the threshold.

## Usage

Run the program with:

```bash
cargo run
```


## Dependencies

This project relies on the following Rust crates:

- `reqwest`: For making HTTP requests to fetch exchange rates.
- `serde` and `serde_json`: For parsing JSON responses.
- `lettre`: For sending email notifications via SMTP.
- `dotenv`: For securely handling environment variables.

## License

Licensed under the [MIT License](https://opensource.org/license/MIT).



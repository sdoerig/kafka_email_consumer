extern crate lettre;
use std::collections::HashMap;

use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransport;
use lettre::{Message, Transport};

pub struct EmailClient {
    config: HashMap<String, String>,
}

impl EmailClient {
    pub fn new(config: HashMap<String, String>) -> Self {
        EmailClient { config }
    }


    pub async fn testmail(&self, message_str: &str) {
        let email = Message::builder()
            .from(self.config.get("email.from").unwrap().parse().unwrap())
            .reply_to(self.config.get("email.reply_to").unwrap().parse().unwrap())
            .to(self.config.get("email.to").unwrap().parse().unwrap())
            .subject(message_str)
            .body(String::from(message_str))
            .unwrap();
    
        let creds = Credentials::new(
            self.config.get("smpt_user").unwrap().to_string(),
            self.config.get("smpt_password").unwrap().to_string(),
        );
    
        // Open a remote connection to gmail
        let mailer = SmtpTransport::starttls_relay(self.config.get("starttls_relay").unwrap())
            .unwrap()
            .credentials(creds)
            .build();
    
        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
}


}

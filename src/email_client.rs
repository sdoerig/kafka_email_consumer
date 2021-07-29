extern crate lettre;
use std::collections::HashMap;

use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransport;
use lettre::{Message, Transport};

pub struct EmailClient {
    config: HashMap<String, String>,
    mailer: SmtpTransport
}

impl EmailClient {
    pub fn new(config: HashMap<String, String>) -> Self {
        let username = config.get("smpt_user").unwrap().to_string();
        let password = config.get("smpt_password").unwrap().to_string();
        let starttls_relay = config.get("starttls_relay").unwrap().to_string();
        let creds = Credentials::new(username, password);
        EmailClient { config, mailer: 
            SmtpTransport::starttls_relay(&starttls_relay)
            .unwrap()
            .credentials(creds)
            .build()
        }
    }


    pub async fn testmail(&self, message_str: &str) {
        let email = Message::builder()
            .from(self.config.get("email.from").unwrap().parse().unwrap())
            .reply_to(self.config.get("email.reply_to").unwrap().parse().unwrap())
            .to(self.config.get("email.to").unwrap().parse().unwrap())
            .subject(message_str)
            .body(String::from(message_str))
            .unwrap();
    
        
    
    
        // Send the email
        match self.mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {:?}", e),
        }
}


}

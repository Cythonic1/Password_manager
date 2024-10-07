
use lettre::{message::SinglePart, Message};
use rand::{ thread_rng, Rng};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};
use dotenv::dotenv;
use std::env;
pub struct OTP;


impl OTP {
    pub fn generate_otp() -> String{
        let rng = thread_rng().gen_range(100_000..999_999);
        rng.to_string()
    }


    pub fn validate_input(input_value: String, generated_otp: String) -> bool{
        if input_value == generated_otp{
            true
        }else{
            false
        }
    }
}

pub fn send_email(recive_email: String, otp:String){
    dotenv().ok();
    // Fetch Gmail credentials from environment variables
    let gmail_user = env::var("GMAIL_USER").expect("GMAIL_USER not set");
    let gmail_password = env::var("GMAIL_APP_PASSWORD").expect("GMAIL_APP_PASSWORD not set");
    let email = Message::builder()
        .from(gmail_user.parse().expect("Scam"))
        .to(recive_email.parse().expect("Scumbale"))
        .subject("Your OTP")
        .singlepart(SinglePart::plain(format!("Your otp code is: {}", otp)))
        .expect("Error while sending the email");


    let creds = Credentials::new(gmail_user.to_string(), gmail_password.to_string());
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Step 4: Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }

}

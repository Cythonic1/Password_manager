#[allow(non_snake_case, unused)]
use std::marker::PhantomData;
use Encryption::EncryptAES256;
use CredintailHandler::creds::{Values, Locked, Creds};
use std::io::{self,Write};
use multi_factor_authentication::{send_email, OTP};
#[allow(non_snake_case, unused)]
mod CredintailHandler;

#[allow(non_snake_case, unused)]
mod Encryption;

mod multi_factor_authentication;
const MYPATH:&str ="/home/pythonic/Desktop/rust/password_manager/test";
#[derive(Debug)]
#[allow(non_snake_case, unused)]
enum MultFactorMethods {
    PhoneNumber(String),
    Email(String),
}
#[derive(Debug)]
#[allow(non_snake_case, unused)]
struct UserInfo <State>{
    user_key: String,
    two_fa: MultFactorMethods,
    state: PhantomData<State>
}


fn main() {
//     let mut locked_values = Values::<Locked>::new(); // Start with a locked state

//     loop {
//         println!("\n--- Credential Manager ---");
//         println!("1. Unlock");
//         println!("2. Quit");
//         print!("Choose an option: \n");
//         let mut input = String::new();
//         io::stdin().read_line(&mut input).expect("Failed to read input");
//         let choice = input.trim();

//         match choice {
//             "1" => {
//                 // Unlock
//                 print!("Enter password to unlock: ");
//                 io::stdout().flush().unwrap();
//                 let mut password = String::new();
//                 io::stdin().read_line(&mut password).expect("Failed to read input");
//                 match locked_values.clone().unlock(password.trim().to_string()) {
//                     Ok(mut unlocked_values) => {
//                         loop {
//                             println!("\n--- Unlocked Credential Manager ---");
//                             println!("1. Add credential");
//                             println!("2. List credentials");
//                             println!("3. Save to JSON");
//                             println!("4. Read from JSON");
//                             println!("5. Encrypt Data");
//                             println!("6. Decrypt Data");
//                             println!("7. Lock and Exit");
//                             print!("Choose an option: \n");
//                             
//                             let mut unlocked_input = String::new();
//                             io::stdin().read_line(&mut unlocked_input).expect("Failed to read input");
//                             let unlocked_choice = unlocked_input.trim();

//                             match unlocked_choice {
//                                 "1" => {
//                                     // Add credential
//                                     let mut username = String::new();
//                                     let mut password = String::new();
//                                     let mut platform = String::new();
//                                     let mut description = String::new();

//                                     print!("Enter username: ");
//                                     io::stdout().flush().unwrap();
//                                     io::stdin().read_line(&mut username).expect("Failed to read input");

//                                     print!("Enter password: ");
//                                     io::stdout().flush().unwrap();
//                                     io::stdin().read_line(&mut password).expect("Failed to read input");

//                                     print!("Enter platform: ");
//                                     io::stdout().flush().unwrap();
//                                     io::stdin().read_line(&mut platform).expect("Failed to read input");

//                                     print!("Enter description: ");
//                                     io::stdout().flush().unwrap();
//                                     io::stdin().read_line(&mut description).expect("Failed to read input");

//                                     let cred = Creds::new(
//                                         username.trim().to_string(),
//                                         password.trim().to_string(),
//                                         platform.trim().to_string(),
//                                         description.trim().to_string(),
//                                     );

//                                     match cred {
//                                         Ok(c) => unlocked_values.add(&c),
//                                         Err(e) => println!("Error: {:?}", e),
//                                     }
//                                 }
//                                 "2" => {
//                                     // List credentials
//                                     unlocked_values.list();
//                                 }
//                                 "3" => {
//                                     // Save to JSON
//                                     unlocked_values.write_json();
//                                 }
//                                 "4" => {
//                                     // Read from JSON
//                                     match Values::read_json() {
//                                         Ok(read_values) => unlocked_values = read_values,
//                                         Err(e) => println!("Error reading from JSON: {:?}", e),
//                                     }
//                                 }
//                                 "5" => {
//                                     // Encrypt Data
//                                     let file = format!("{MYPATH}/test.json");
//                                     EncryptAES256::enc(&file, &file).expect("Error encrypting data");
//                                     println!("Data has been encrypted.");
//                                 }
//                                 "6" => {
//                                     // Decrypt Data
//                                     let file = format!("{MYPATH}/test.json");
//                                     EncryptAES256::dec(&file, &file).expect("Error decrypting data");
//                                     println!("Data has been decrypted.");
//                                 }
//                                 "7" => {
//                                     // Lock and exit
//                                     locked_values = unlocked_values.lock();
//                                     break;
//                                 }
//                                 _ => println!("Invalid option, please choose again."),
//                             }
//                         }
//                     }
//                     Err(e) => println!("Error unlocking: {:?}", e),
//                 }
//             }
//             "2" => {
//                 println!("Goodbye!");
//                 break;
//             }
//             _ => println!("Make Sure you provide the key first Or unlock"),
//         }
//     }

// Step 1: Generate an OTP
    let otp = OTP::generate_otp();
    // println!("Generated OTP: {}", otp);  // For debugging; remove in production

    // Step 2: Specify the recipient email (change this as necessary)
    let recipient_email = String::from("yasser050699@gmail.com");  // Replace with your actual email

    // Step 3: Send the OTP via email
    send_email(recipient_email, otp.clone());

    // Step 4: Prompt the user to input the OTP they received via email
    println!("Please enter the OTP you received:");

    let mut input_value = String::new();
    io::stdin().read_line(&mut input_value).expect("Failed to read input");
    let input_value = input_value.trim().to_string();

    // Step 5: Validate the input
    if OTP::validate_input(input_value, otp) {
        println!("OTP is correct. Authentication successful!");
    } else {
        println!("Incorrect OTP. Authentication failed.");
    }

}


#[allow(unused)]
use aes::Aes256;
use block_modes::cipher::NewCipher;
use rand::Rng;
use serde::de::Error;
use std::fs;
use std::io::{Read, Write};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Default, Serialize, Deserialize)] // Derive Clone for Creds so it can be cloned when passed by reference
pub struct EncryptAES256;

const MYPATH:&str ="/home/pythonic/Desktop/rust/password_manager/";
#[derive(Debug, Clone)] // Derive Clone for Creds so it can be cloned when passed by as
pub enum ErrorsEnc{
    InvalidFileFormat(String),
    InvalidKey(String),
    UnableToOpenFile(String),
    UnableToWrite(String),
    UnableToRead(String),
    UnvalidKey(String)
}

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

impl EncryptAES256 {
    
    pub fn enc(input_file: &str, output_file: &str)->Result<(), ErrorsEnc> {
        // Open the read file
        let mut file = match fs::File::open(input_file){
            Ok(opened_file) => opened_file,
            Err(e) => return Err(ErrorsEnc::UnableToOpenFile("Unable to open the file make sure the file exist".to_string()))
        };

        let mut buffer : Vec<u8>= Vec::new();
        file.read_to_end(&mut buffer).expect("Some error");


        // Generate a Random IV,
        let mut iv = [0u8; 16];
        rand::thread_rng().fill(&mut iv);

        //Generate Random Key,
        let key = rand::thread_rng().gen::<[u8; 32]>();
        let path_to_key = format!("{MYPATH}/key");
        let mut Key_file = fs::File::create(&path_to_key).expect("can not write the key");
        match Key_file.write_all(&key){
            Ok(_) => println!("The key as been written"),
            Err(e) => println!("unable to write the key:{e}")
        }
        
        // create the cipher.
        let cipher = Aes256Cbc::new_from_slices(&key, &iv).expect("can not encrypt");

        //Encrypt the data
        let cipher_text = cipher.encrypt_vec(&buffer);

        match fs::File::create(output_file){
            Ok(mut file) => {
                match file.write_all(&iv) {
                    Ok(_) =>  match file.write_all(&cipher_text){
                        Ok(_) => {
                            println!("You data has been ecncrypted: 🔐");
                            Ok(())
                        },
                        Err(e) => return Err(ErrorsEnc::UnableToWrite(e.to_string()))
                    }
                    Err(e) => return Err(ErrorsEnc::UnableToWrite(e.to_string())),
                }
            },
            Err(e) => return Err(ErrorsEnc::UnableToOpenFile(e.to_string()))
        }
    }

    pub fn dec(input_file: &str, output_file: &str) -> Result<(), ErrorsEnc>{

        let mut file = match fs::File::open(input_file){
            Ok(file) => file,
            Err(e) => return Err(ErrorsEnc::UnableToOpenFile(e.to_string()))  
        };

        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer){
            Ok(_) => {} ,
            Err(e) => return Err(ErrorsEnc::UnableToRead(e.to_string()))
        };

        let path_to_key = format!("{MYPATH}/key");
        let mut key_buf = Vec::new();
        match fs::File::open(&path_to_key){
            Ok(mut key) => {
                key.read_to_end(&mut key_buf)
            },
            Err(e) => return Err(ErrorsEnc::UnableToOpenFile(e.to_string()))
        };


        let (iv, content) = buffer.split_at(16); // extarct the IV.
        let cipher = Aes256Cbc::new_from_slices(&key_buf, &iv).expect("Unable to Generate the cipher while Decrypt");

        match cipher.decrypt_vec(&content){
            Ok(dec_data) => {
                let mut output_file = fs::OpenOptions::new()
                    .write(true)       // Enable write mode
                    .truncate(true)    // Truncate the file
                    .open(output_file) // Open the file specified by output_file
                    .expect("unable to open output file for writing");
                output_file.write_all(&dec_data);
                Ok(())
            },
            Err(e) => return Err(ErrorsEnc::UnvalidKey(e.to_string())) 
        }
        // println!("Decrypted data: {:?}", String::from_utf8_lossy(&dec_data));
    }
}



